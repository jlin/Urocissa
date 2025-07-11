// tui.rs
use arrayvec::ArrayString;
use bytesize::ByteSize;
use std::{
    collections::VecDeque,
    path::PathBuf,
    sync::{Arc, LazyLock, OnceLock, RwLock},
    time::Instant,
};
use superconsole::{Component, Dimensions, DrawMode, Line, Lines, SuperConsole};
use terminal_size::{Width, terminal_size};
use tokio::{
    select,
    sync::mpsc::{UnboundedReceiver, UnboundedSender},
    time::{Duration, interval},
};
use unicode_width::{UnicodeWidthChar, UnicodeWidthStr};

pub static LOGGER_TX: OnceLock<UnboundedSender<String>> = OnceLock::new();
pub static MAX_ROWS: LazyLock<usize> = LazyLock::new(|| rayon::current_num_threads());

/// ---------- async driver ----------
pub async fn tui_task(
    mut sc: SuperConsole,
    dashboard: Arc<RwLock<Dashboard>>,
    mut rx: UnboundedReceiver<String>,
) -> anyhow::Result<()> {
    let mut tick = interval(Duration::from_millis(200));
    loop {
        select! {
            Some(line) = rx.recv() => sc.emit(Lines(vec![superconsole::content::Line::unstyled(&line)?])),
            _ = tick.tick() => {
                let guard = dashboard.read().unwrap();
                sc.render(&*guard)?;
            }
        }
    }
}

/// ---------- task model ----------
pub enum TaskState {
    Running(Instant),
    Done(f64),
}
pub struct TaskRow {
    pub hash: ArrayString<64>,
    pub path: PathBuf,
    pub state: TaskState,
}

impl TaskRow {
    pub fn fmt(&self) -> String {
        const DEFAULT_COLS: usize = 120;
        let margin = std::env::var("UROCISSA_TERM_MARGIN")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(4);
        let cols = terminal_size()
            .map(|(Width(w), _)| w as usize)
            .unwrap_or(DEFAULT_COLS);

        let (bullet, secs) = match self.state {
            TaskState::Running(t0) => ('•', t0.elapsed().as_secs_f64()),
            TaskState::Done(d) => ('✓', d),
        };

        let short = &self.hash.as_str()[..5.min(self.hash.len())];
        let prefix = format!("{bullet} {:<5} │ ", short);
        let suffix = format!("│ {:>6.1}s", secs);

        let path_budget = cols
            .saturating_sub(
                UnicodeWidthStr::width(prefix.as_str())
                    + UnicodeWidthStr::width(suffix.as_str())
                    + margin,
            )
            .max(5);

        let raw_path = self.path.display().to_string();
        let short_path = Self::tail_ellipsis(&raw_path, path_budget);
        let pad =
            " ".repeat(path_budget.saturating_sub(UnicodeWidthStr::width(short_path.as_str())));
        format!("{prefix}{short_path}{pad}{suffix}")
    }

    fn tail_ellipsis(s: &str, max: usize) -> String {
        if UnicodeWidthStr::width(s) <= max {
            return s.to_owned();
        }
        let tail_len = max.saturating_sub(1);
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
        format!("…{}", rev.chars().rev().collect::<String>())
    }
}

/// ---------- dashboard ----------
pub struct Dashboard {
    pub tasks: Vec<TaskRow>,          // running
    pub completed: VecDeque<TaskRow>, // finished (oldest at front)
    pub handled: u64,
    pub db_bytes: u64,
}
pub static DASHBOARD: LazyLock<Arc<RwLock<Dashboard>>> =
    LazyLock::new(|| Arc::new(RwLock::new(Dashboard::new())));

impl Dashboard {
    pub fn new() -> Self {
        Self {
            tasks: vec![],
            completed: VecDeque::new(),
            handled: 0,
            db_bytes: 0,
        }
    }

    pub fn add_task(&mut self, hash: ArrayString<64>, path: PathBuf) {
        if let Some(t) = self.tasks.iter_mut().find(|t| t.hash == hash) {
            t.path = path;
            t.state = TaskState::Running(Instant::now());
        } else {
            self.tasks.push(TaskRow {
                hash,
                path,
                state: TaskState::Running(Instant::now()),
            });
        }
    }

    pub fn remove_task(&mut self, hash: &ArrayString<64>) {
        if let Some(pos) = self.tasks.iter().position(|t| &t.hash == hash) {
            let mut row = self.tasks.remove(pos);
            if let TaskState::Running(t0) = row.state {
                row.state = TaskState::Done(t0.elapsed().as_secs_f64());
            }
            self.completed.push_back(row); // newest at back
            while self.completed.len() > *MAX_ROWS {
                self.completed.pop_front(); // keep at most MAX_ROWS
            }
            self.handled += 1;
        }
    }
}

/// ---------- renderer ----------
impl Component for Dashboard {
    fn draw_unchecked(&self, _: Dimensions, _: DrawMode) -> anyhow::Result<Lines> {
        let cols = terminal_size()
            .map(|(Width(w), _)| w as usize)
            .unwrap_or(120);
        let sep = "─".repeat(cols);
        let mut lines: Vec<Line> = Vec::new();

        // top rule
        lines.push(vec![sep.clone()].try_into()?);

        // stats
        let human = ByteSize(self.db_bytes).to_string();
        let remain = self.tasks.len().saturating_sub(*MAX_ROWS);
        let extra = if remain > 0 {
            format!(" │  … remaining {remain}")
        } else {
            String::new()
        };
        let mut stats = format!(
            "• Processed: {:<6} │ DB size: {:>8}{extra}",
            self.handled, human
        );
        stats.push_str(&" ".repeat(cols.saturating_sub(UnicodeWidthStr::width(stats.as_str()))));
        lines.push(vec![stats].try_into()?);

        // second rule
        lines.push(vec![sep.clone()].try_into()?);

        // fill lines: completed first, then running
        let max = *MAX_ROWS;
        let running_len = self.tasks.len();

        if running_len >= max {
            // more running tasks than viewport; show the last `max` running rows
            for t in self.tasks.iter().rev().take(max).rev() {
                lines.push(vec![t.fmt()].try_into()?);
            }
        } else {
            // show all completed rows needed to fill space
            let needed_completed = max - running_len;
            let start = self.completed.len().saturating_sub(needed_completed);
            for t in self.completed.iter().skip(start) {
                // oldest → newest
                lines.push(vec![t.fmt()].try_into()?);
            }
            // then running rows in insertion order (oldest running first, newest last)
            for t in &self.tasks {
                lines.push(vec![t.fmt()].try_into()?);
            }
        }

        // ensure exactly MAX_ROWS lines by padding if necessary
        while lines.len() - 3 < max {
            // minus rule+stats+rule already added
            lines.push(vec![" ".repeat(cols)].try_into()?);
        }

        Ok(Lines(lines))
    }
}
