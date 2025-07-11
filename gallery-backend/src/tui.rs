use arrayvec::ArrayString;
use bytesize::ByteSize;
use std::path::PathBuf;
use std::sync::{Arc, LazyLock, OnceLock, RwLock};
use std::time::Instant;
use terminal_size::{Width, terminal_size};
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tokio::{
    select,
    time::{Duration, interval},
};
use unicode_width::{UnicodeWidthChar, UnicodeWidthStr};

use superconsole::{Component, Dimensions, DrawMode, Line, Lines, SuperConsole};

/// Global sender for log messages written through `TokioPipe`.
pub static LOGGER_TX: OnceLock<UnboundedSender<String>> = OnceLock::new();

/// Maximum number of task rows shown equals the Rayon thread-pool size.
pub static MAX_ROWS: LazyLock<usize> = LazyLock::new(|| rayon::current_num_threads());

/// Pipe that forwards `stdout`/`stderr` to an async channel, allowing redirection into the TUI.
pub struct TokioPipe(pub UnboundedSender<String>);

impl std::io::Write for TokioPipe {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let s = String::from_utf8_lossy(buf);
        // Split by `\n`; the newline itself is discarded.
        for line in s.split_terminator('\n') {
            // Replace tabs with four spaces so alignment stays predictable.
            let clean = line.replace('\t', "    ");
            if !clean.is_empty() {
                let _ = self.0.send(clean.to_string());
            }
        }
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

/// Tokio task that drives the TUI. It listens for incoming log lines and
/// periodically redraws the `Dashboard`.
pub async fn tui_task(
    mut sc: SuperConsole,
    dashboard: Arc<RwLock<Dashboard>>, // shared, read-heavy
    mut rx: UnboundedReceiver<String>,
) -> anyhow::Result<()> {
    let mut tick = interval(Duration::from_millis(200));

    loop {
        select! {
            // A. New log line → emit to the top area.
            Some(line) = rx.recv() => {
                sc.emit(Lines(vec![
                    superconsole::content::Line::unstyled(&line)?
                ]));
            }

            // B. Periodic repaint of the dashboard.
            _ = tick.tick() => {
                let guard = dashboard.read().unwrap();
                sc.render(&*guard)?;
            }
        }
    }
}

/// A single running task shown in the dashboard.
pub struct TaskRow {
    pub hash: ArrayString<64>,
    pub path: PathBuf,
    pub started: Instant,
}

impl TaskRow {
    /// Format the row for display, truncating the path from the head if needed.
    fn fmt(&self) -> String {
        /* Terminal width and safety margin */
        const DEFAULT_COLS: usize = 120;
        let margin = std::env::var("UROCISSA_TERM_MARGIN")
            .ok()
            .and_then(|v| v.parse::<usize>().ok())
            .unwrap_or(4);
        let cols = terminal_size()
            .map(|(Width(w), _)| w as usize)
            .unwrap_or(DEFAULT_COLS);

        /* Prefix & suffix */
        let short_hash = &self.hash.as_str()[..5.min(self.hash.len())];
        let prefix = format!("• {:<5} │ ", short_hash);
        let prefix_w = UnicodeWidthStr::width(prefix.as_str());

        let secs = self.started.elapsed().as_secs_f64();
        let suffix = format!("│ {:>6.1}s", secs);
        let suffix_w = UnicodeWidthStr::width(suffix.as_str());

        /* Remaining budget for the path */
        let path_budget = cols.saturating_sub(prefix_w + suffix_w + margin).max(5);

        /* Truncate the path, keeping the tail and inserting an ellipsis if necessary. */
        let raw_path = self.path.display().to_string();
        let short_path = Self::tail_ellipsis(&raw_path, path_budget);

        let path_w = UnicodeWidthStr::width(short_path.as_str());
        let spaces = " ".repeat(path_budget.saturating_sub(path_w));

        format!("{prefix}{short_path}{spaces}{suffix}")
    }

    /// Keep the tail of `s` so that its display width fits `max`; prepend an ellipsis if truncated.
    fn tail_ellipsis(s: &str, max: usize) -> String {
        if UnicodeWidthStr::width(s) <= max {
            return s.to_owned();
        }

        let tail_len = max.saturating_sub(1); // leave room for ‘…’
        let mut acc = 0;
        let mut rev = String::new();

        for c in s.chars().rev() {
            let w = c.width().unwrap_or(0);
            if acc + w > tail_len {
                break;
            }
            acc += w;
            rev.push(c);
        }
        let tail: String = rev.chars().rev().collect();
        format!("…{tail}")
    }
}

/// Aggregate state rendered by the dashboard.
pub struct Dashboard {
    pub tasks: Vec<TaskRow>,
    /// Number of completed tasks.
    pub handled: u64,
    /// Bytes currently used by the on-disk database.
    pub db_bytes: u64,
}

/// Global dashboard instance.
pub static DASHBOARD: LazyLock<Arc<RwLock<Dashboard>>> =
    LazyLock::new(|| Arc::new(RwLock::new(Dashboard::new())));

impl Component for Dashboard {
    fn draw_unchecked(&self, _: Dimensions, _: DrawMode) -> anyhow::Result<Lines> {
        let cols = terminal_size()
            .map(|(Width(w), _)| w as usize)
            .unwrap_or(120);

        let sep = "─".repeat(cols);
        let mut lines: Vec<Line> = Vec::new();

        /* 1. top rule */
        lines.push(vec![sep.clone()].try_into()?);

        /* 2. statistics row */
        let human = ByteSize(self.db_bytes).to_string();
        let total = self.tasks.len();
        let max_rows = *MAX_ROWS;
        let remain = total.saturating_sub(max_rows);
        let extra = if remain > 0 {
            format!(" │  … remaining {remain}")
        } else {
            String::new()
        };

        let mut stats = format!(
            "• Processed: {:<6} │ DB size: {:>8}{extra}",
            self.handled, human
        );
        let pad = cols.saturating_sub(UnicodeWidthStr::width(stats.as_str()));
        stats.push_str(&" ".repeat(pad));
        lines.push(vec![stats].try_into()?);

        /* 3. second rule */
        lines.push(vec![sep].try_into()?);

        /* 4. task rows and padding */
        let shown_iter = self.tasks.iter().take(max_rows);
        let shown_cnt = shown_iter.len();
        for t in shown_iter {
            lines.push(vec![t.fmt()].try_into()?);
        }
        for _ in 0..max_rows.saturating_sub(shown_cnt) {
            lines.push(vec![" ".repeat(cols)].try_into()?);
        }

        Ok(Lines(lines))
    }
}

impl Dashboard {
    /// Create an empty dashboard.
    pub fn new() -> Self {
        Dashboard {
            tasks: Vec::new(),
            handled: 0,
            db_bytes: 0,
        }
    }

    /// Add a new task or reset an existing one that has the same hash.
    pub fn add_task(&mut self, hash: ArrayString<64>, path: PathBuf) {
        if let Some(t) = self.tasks.iter_mut().find(|t| t.hash == hash) {
            t.path = path;
            t.started = Instant::now();
        } else {
            self.tasks.push(TaskRow {
                hash,
                path,
                started: Instant::now(),
            });
        }
    }

    /// Remove a task after it completes and increment the `handled` counter.
    pub fn remove_task(&mut self, hash: &ArrayString<64>) {
        let mut removed = false;
        self.tasks.retain(|t| {
            let keep = &t.hash != hash;
            if !keep {
                removed = true;
            }
            keep
        });
        if removed {
            self.handled += 1;
        }
    }
}
