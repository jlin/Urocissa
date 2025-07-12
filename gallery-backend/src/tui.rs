// tui.rs

use arrayvec::ArrayString;
use std::{
    collections::VecDeque,
    mem,
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

#[derive(Debug, Clone)]
pub enum FileType {
    Image,
    Video,
}

impl TryFrom<&str> for FileType {
    type Error = anyhow::Error;

    fn try_from(s: &str) -> anyhow::Result<Self> {
        match s {
            "image" => Ok(FileType::Image),
            "video" => Ok(FileType::Video),
            _ => Err(anyhow::anyhow!("Unknown file type: {}", s)),
        }
    }
}

pub enum TaskState {
    Indexing(Instant),
    Transcoding(Instant),
    Done(f64),
}

pub struct TaskRow {
    pub hash: ArrayString<64>,
    pub path: PathBuf,
    pub file_type: FileType,
    pub state: TaskState,
    pub progress: Option<f64>,
}

impl TaskRow {
    /// Advances the task to its next state based on its current state and file_type.
    /// This is the core of the state machine.
    pub fn advance_state(&mut self) {
        // We use mem::replace to take ownership of the current state,
        // allowing us to consume it in the match statement.
        let current_state = mem::replace(&mut self.state, TaskState::Done(0.0)); // Dummy value

        let new_state = match current_state {
            // When the current state is Indexing...
            TaskState::Indexing(t0) => {
                // ...check the file type to decide the next state.
                match self.file_type {
                    FileType::Image => TaskState::Done(t0.elapsed().as_secs_f64()),
                    FileType::Video => TaskState::Transcoding(Instant::now()),
                }
            }
            // When the current state is Transcoding...
            TaskState::Transcoding(t0) => {
                // ...the only next state is Done.
                TaskState::Done(t0.elapsed().as_secs_f64())
            }
            // If the task was already done, it remains done.
            TaskState::Done(d) => TaskState::Done(d),
        };

        // Set the new state.
        self.state = new_state;

        // Reset progress if we have just entered a new processing state.
        if matches!(self.state, TaskState::Transcoding(_)) {
            self.progress = None;
        }
    }

    /// REFACTORED: Formatting adjusted to the user's new specification.
    pub fn fmt(&self) -> String {
        const COL_STATUS: usize = 6; // 100.0% ⇒ 6
        const COL_HASH: usize = 5;
        const DEFAULT_COLS: usize = 120;

        let margin = std::env::var("UROCISSA_TERM_MARGIN")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(4);
        let cols = terminal_size()
            .map(|(Width(w), _)| w as usize)
            .unwrap_or(DEFAULT_COLS);

        /* ----------  status / progress  ---------- */
        let status = match (&self.state, self.progress) {
            (TaskState::Transcoding(_), Some(p)) => format!("{:>5.1}%", p.min(100.0)),
            (TaskState::Done(_), _) => "✓".into(),
            _ => "•".into(),
        };
        let status_col = format!("{:<COL_STATUS$}", status);

        /* ----------  hash  ---------- */
        let short_hash = &self.hash.as_str()[..COL_HASH.min(self.hash.len())];
        let hash_col = format!("{:>COL_HASH$}", short_hash);

        /* ----------  elapsed secs  ---------- */
        let secs = match self.state {
            TaskState::Indexing(t0) | TaskState::Transcoding(t0) => t0.elapsed().as_secs_f64(),
            TaskState::Done(d) => d,
        };
        let suffix = format!(" │ {:>6.1}s", secs);

        /* ----------  path  ---------- */
        let prefix_w = COL_STATUS + 3 /* │ */ + COL_HASH + 3 /* │ */;
        let path_budget = cols
            .saturating_sub(prefix_w + UnicodeWidthStr::width(suffix.as_str()) + margin)
            .max(5);

        let raw_path = self.path.display().to_string();
        let short_path = Self::tail_ellipsis(&raw_path, path_budget);
        let pad =
            " ".repeat(path_budget.saturating_sub(UnicodeWidthStr::width(short_path.as_str())));

        /* ----------  assemble  ---------- */
        format!("{status_col} │ {hash_col} │ {short_path}{pad}{suffix}")
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
    pub pending: u64,
}

pub static DASHBOARD: LazyLock<Arc<RwLock<Dashboard>>> =
    LazyLock::new(|| Arc::new(RwLock::new(Dashboard::new())));

impl Dashboard {
    pub fn new() -> Self {
        Self {
            tasks: vec![],
            completed: VecDeque::new(),
            handled: 0,
            pending: 0,
        }
    }

    pub fn add_task(&mut self, hash: ArrayString<64>, path: PathBuf, file_type: FileType) {
        if let Some(t) = self.tasks.iter_mut().find(|t| t.hash == hash) {
            // Restart task if it already exists
            t.path = path;
            t.file_type = file_type;
            t.state = TaskState::Indexing(Instant::now());
            t.progress = None;
        } else {
            // Create a new task, starting in the Indexing state.
            self.tasks.push(TaskRow {
                hash,
                path,
                file_type,
                state: TaskState::Indexing(Instant::now()),
                progress: None,
            });
        }
    }

    /// Advances a task to its next state and moves it to the completed list if it's done.
    pub fn advance_task_state(&mut self, hash: &ArrayString<64>) {
        if let Some(pos) = self.tasks.iter().position(|t| &t.hash == hash) {
            // Tell the task to update its own state.
            self.tasks[pos].advance_state();

            // Check if the update resulted in a finished state.
            if let TaskState::Done(_) = self.tasks[pos].state {
                let row = self.tasks.remove(pos);
                self.move_to_completed(row);
            }
        }
    }

    fn move_to_completed(&mut self, row: TaskRow) {
        self.completed.push_back(row);
        while self.completed.len() > *MAX_ROWS {
            self.completed.pop_front();
        }
        self.handled += 1;
    }

    pub fn update_progress(&mut self, hash: ArrayString<64>, percent: f64) {
        if let Some(row) = self.tasks.iter_mut().find(|t| t.hash == hash) {
            // Clamp to 0–100 just in case
            row.progress = Some(percent.clamp(0.0, 100.0));
        }
    }
    pub fn increase_pending(&mut self) {
        self.pending = self.pending.saturating_add(1);
    }

    pub fn decrease_pending(&mut self) {
        self.pending = self.pending.saturating_sub(1);
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

        let mut stats = format!(
            "• Processed: {:<6} │ Pending: {:<6}",
            self.handled, self.pending
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
        // The header consists of 3 lines (rule, stats, rule)
        let header_lines = 3;
        while lines.len() < max + header_lines {
            lines.push(vec![" ".repeat(cols)].try_into()?);
        }

        // Trim any excess lines if we have fewer than MAX_ROWS total tasks
        lines.truncate(max + header_lines);

        Ok(Lines(lines))
    }
}
