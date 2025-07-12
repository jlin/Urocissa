//! tui.rs — lock-free dashboard (Rust 1.88, std::LazyLock / OnceLock)

use arrayvec::ArrayString;
use atomic_float::AtomicF64;
use crossbeam_queue::ArrayQueue;
use dashmap::DashMap;
use std::{
    mem,
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc, LazyLock, OnceLock,
    },
    time::Instant,
};
use superconsole::{Component, Dimensions, DrawMode, Line, Lines, SuperConsole};
use terminal_size::{terminal_size, Width};
use tokio::{
    select,
    sync::mpsc::{UnboundedReceiver, UnboundedSender},
    time::{interval, Duration},
};
use unicode_width::{UnicodeWidthChar, UnicodeWidthStr};

use crate::structure::database_struct::database::definition::Database;

/// ---------- async driver ----------
pub async fn tui_task(
    mut sc: SuperConsole,
    dashboard: Arc<Dashboard>,
    mut rx: UnboundedReceiver<String>,
) -> anyhow::Result<()> {
    let mut tick = interval(Duration::from_millis(200));
    loop {
        select! {
            Some(line) = rx.recv() => sc.emit(Lines(vec![superconsole::content::Line::unstyled(&line)?])),
            _ = tick.tick() => sc.render(&*dashboard)?,
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
            _       => Err(anyhow::anyhow!("Unknown file type: {s}")),
        }
    }
}

pub enum TaskState {
    Indexing(Instant),
    Transcoding(Instant),
    Done(f64),
}
impl Clone for TaskState {                // 手動實作，Instant 是 Copy
    fn clone(&self) -> Self {
        match self {
            TaskState::Indexing(t)   => TaskState::Indexing(*t),
            TaskState::Transcoding(t)=> TaskState::Transcoding(*t),
            TaskState::Done(d)       => TaskState::Done(*d),
        }
    }
}

#[derive(Clone)]
pub struct TaskRow {
    pub hash: ArrayString<64>,
    pub path: String,
    pub file_type: FileType,
    pub state: TaskState,
    pub progress: Option<f64>,
}

impl TaskRow {
    pub fn advance_state(&mut self) {
        let old = mem::replace(&mut self.state, TaskState::Done(0.0));
        self.state = match old {
            TaskState::Indexing(t0) => match self.file_type {
                FileType::Image => TaskState::Done(t0.elapsed().as_secs_f64()),
                FileType::Video => TaskState::Transcoding(Instant::now()),
            },
            TaskState::Transcoding(t0) => TaskState::Done(t0.elapsed().as_secs_f64()),
            TaskState::Done(d)         => TaskState::Done(d),
        };
        if matches!(self.state, TaskState::Transcoding(_)) {
            self.progress = None;
        }
    }

    pub fn fmt(&self) -> String {
        const COL_STATUS: usize = 6;
        const COL_HASH:   usize = 5;
        const DEFAULT_COLS: usize = 120;

        let margin = std::env::var("UROCISSA_TERM_MARGIN")
            .ok().and_then(|v| v.parse().ok()).unwrap_or(4);
        let cols   = terminal_size().map(|(Width(w), _)| w as usize).unwrap_or(DEFAULT_COLS);

        let status = match (&self.state, self.progress) {
            (TaskState::Transcoding(_), Some(p)) => format!("{:>5.1}%", p.min(100.0)),
            (TaskState::Done(_), _)               => "✓".into(),
            _                                     => "•".into(),
        };
        let status_col = format!("{:<COL_STATUS$}", status);

        let short_hash = &self.hash.as_str()[..COL_HASH.min(self.hash.len())];
        let hash_col   = format!("{:>COL_HASH$}", short_hash);

        let secs = match self.state {
            TaskState::Indexing(t0) | TaskState::Transcoding(t0) => t0.elapsed().as_secs_f64(),
            TaskState::Done(d)                                   => d,
        };
        let suffix = format!(" │ {:>6.1}s", secs);

        let prefix_w    = COL_STATUS + 3 + COL_HASH + 3;
        let path_budget = cols
            .saturating_sub(prefix_w + UnicodeWidthStr::width(suffix.as_str()) + margin)
            .max(5);

        let short_path = Self::tail_ellipsis(&self.path, path_budget);
        let pad        = " ".repeat(path_budget.saturating_sub(
                                   UnicodeWidthStr::width(short_path.as_str())));

        format!("{status_col} │ {hash_col} │ {short_path}{pad}{suffix}")
    }

    fn tail_ellipsis(s: &str, max: usize) -> String {
        if UnicodeWidthStr::width(s) <= max { return s.to_owned(); }
        let tail_len = max.saturating_sub(1);
        let mut acc  = 0;
        let mut rev  = String::new();
        for c in s.chars().rev() {
            let w = c.width().unwrap_or(0);
            if acc + w > tail_len { break; }
            acc += w; rev.push(c);
        }
        format!("…{}", rev.chars().rev().collect::<String>())
    }
}

/// ---------- dashboard ----------
pub struct Dashboard {
    tasks:          DashMap<ArrayString<64>, TaskRow>,
    completed:      ArrayQueue<TaskRow>,     // lock-free ring
    handled:        AtomicU64,
    pending:        AtomicU64,
    total_duration: AtomicF64,
}

pub static MAX_ROWS:  LazyLock<usize>                 = LazyLock::new(|| rayon::current_num_threads());
pub static LOGGER_TX: OnceLock<UnboundedSender<String>> = OnceLock::new();
pub static DASHBOARD: LazyLock<Arc<Dashboard>>        = LazyLock::new(|| Arc::new(Dashboard::new()));

impl Dashboard {
    pub fn new() -> Self {
        Self {
            tasks:          DashMap::new(),
            completed:      ArrayQueue::new(*MAX_ROWS * 4),
            handled:        AtomicU64::new(0),
            pending:        AtomicU64::new(0),
            total_duration: AtomicF64::new(0.0),
        }
    }

    /* ---------- mutation API ---------- */
    pub fn add_task(&self, hash: ArrayString<64>, path: String, file_type: FileType) {
        self.tasks
            .entry(hash.clone())
            .and_modify(|t| {
                t.path  = path.clone();
                t.file_type = file_type.clone();
                t.state = TaskState::Indexing(Instant::now());
                t.progress = None;
            })
            .or_insert_with(|| TaskRow {
                hash,
                path,
                file_type,
                state: TaskState::Indexing(Instant::now()),
                progress: None,
            });
    }

    pub fn advance_task_state(&self, hash: &ArrayString<64>) {
        if let Some(mut view) = self.tasks.get_mut(hash) {
            view.advance_state();
            if let TaskState::Done(duration) = view.state {
                let row_done = view.clone();
                drop(view);
                self.tasks.remove(hash);
                self.move_to_completed(row_done, duration);
            }
        }
    }

    fn move_to_completed(&self, row: TaskRow, duration: f64) {
        self.total_duration.fetch_add(duration, Ordering::Relaxed);

        match self.completed.push(row) {          // 不再重複使用已 move 的值
            Ok(()) => {}
            Err(r) => {                            // r 回傳值仍擁有權
                let _ = self.completed.pop();
                let _ = self.completed.push(r);
            }
        }
        self.handled.fetch_add(1, Ordering::Relaxed);
    }

    pub fn update_progress(&self, hash: ArrayString<64>, percent: f64) {
        if let Some(mut view) = self.tasks.get_mut(&hash) {
            view.progress = Some(percent.clamp(0.0, 100.0));
        }
    }
    pub fn increase_pending(&self) { self.pending.fetch_add(1, Ordering::Relaxed); }
    pub fn decrease_pending(&self) { self.pending.fetch_sub(1, Ordering::Relaxed); }

    #[inline] fn handled(&self)        -> u64 { self.handled.load(Ordering::Relaxed) }
    #[inline] fn pending(&self)        -> u64 { self.pending.load(Ordering::Relaxed) }
    #[inline] fn total_duration(&self) -> f64 { self.total_duration.load(Ordering::Relaxed) }
}

/// ---------- renderer ----------
impl Component for Dashboard {
    fn draw_unchecked(&self, _: Dimensions, _: DrawMode) -> anyhow::Result<Lines> {
        let cols = terminal_size().map(|(Width(w), _)| w as usize).unwrap_or(120);
        let sep  = "─".repeat(cols);
        let mut lines: Vec<Line> = Vec::new();

        lines.push(vec![sep.clone()].try_into()?);

        let avg = if self.handled() > 0 {
            format!("│ Avg: {:.2}s", self.total_duration() / self.handled() as f64)
        } else { String::new() };

        let mut stats = format!("• Processed: {:<6} │ Pending: {:<6} {}",
                                self.handled(), self.pending(), avg);
        stats.push_str(&" ".repeat(cols.saturating_sub(UnicodeWidthStr::width(stats.as_str()))));
        lines.push(vec![stats].try_into()?);

        lines.push(vec![sep.clone()].try_into()?);

        /* snapshot rows */
        let running   : Vec<_> = self.tasks.iter().map(|kv| kv.value().clone()).collect();
        let completed : Vec<_> = {
            let mut v = Vec::with_capacity(self.completed.len());
            while let Some(item) = self.completed.pop() {
                v.push(item);
            }
            for item in &v {
                // This might fail if other threads are pushing, which is acceptable.
                let _ = self.completed.push(item.clone());
            }
            v
        };

        let max         = *MAX_ROWS;
        let running_len = running.len();

        if running_len >= max {
            let mut slice = running;
            slice.sort_by_key(|r| match r.state {
                TaskState::Indexing(t0) | TaskState::Transcoding(t0) => t0,
                TaskState::Done(_) => Instant::now(),
            });
            for t in slice.into_iter().rev().take(max).rev() {
                lines.push(vec![t.fmt()].try_into()?);
            }
        } else {
            let need  = max - running_len;
            let start = completed.len().saturating_sub(need);
            for t in completed.iter().skip(start) {
                lines.push(vec![t.fmt()].try_into()?);
            }
            for t in running {
                lines.push(vec![t.fmt()].try_into()?);
            }
        }

        while lines.len() < max + 3 {
            lines.push(vec![" ".repeat(cols)].try_into()?);
        }
        lines.truncate(max + 3);

        Ok(Lines(lines))
    }
}
