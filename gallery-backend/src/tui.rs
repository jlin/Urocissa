use arrayvec::ArrayString;
use std::path::PathBuf;
use std::sync::{Arc, LazyLock, OnceLock, RwLock};
use std::time::Instant;
use terminal_size::Width;
use terminal_size::terminal_size;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tokio::{
    select,
    time::{Duration, interval},
};
use unicode_width::{UnicodeWidthChar, UnicodeWidthStr};
pub static LOGGER_TX: OnceLock<UnboundedSender<String>> = OnceLock::new();

use superconsole::{Component, Dimensions, DrawMode, Lines, SuperConsole};

pub struct TokioPipe(pub UnboundedSender<String>);
impl std::io::Write for TokioPipe {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let s = String::from_utf8_lossy(buf);
        for line in s.split_terminator('\n') {
            // ← 切掉最後的 \n
            let clean = line.replace('\t', "    "); // ← 如有 Tab, 換空格
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
pub async fn tui_task(
    mut sc: SuperConsole,
    dashboard: Arc<RwLock<Dashboard>>, // ❷ 共享讀寫鎖
    mut rx: UnboundedReceiver<String>,
) -> anyhow::Result<()> {
    let mut tick = interval(Duration::from_millis(200));

    loop {
        select! {
            //── A. 收到 logger 行：emit 到上方 ────────────────────────────
            Some(line) = rx.recv() => {
                sc.emit(Lines(vec![
                    superconsole::content::Line::unstyled(&line)?
                ]));
            }

            //── B. 每 200 ms 重新渲染 Scratch 區域 ───────────────────────
            _ = tick.tick() => {
                // 只讀鎖：允許多個渲染迴圈同時取用
                let guard = dashboard.read().unwrap(); // ❸
                sc.render(&*guard)?;    // Dashboard 已實作 Component
            }
        }
    }
}

struct TaskRow {
    hash: ArrayString<64>,
    path: PathBuf,
    started: Instant,
}
impl TaskRow {
    fn fmt(&self) -> String {
        /* ---------- 0. 終端欄寬 + 安全邊界 ---------- */
        const DEFAULT_COLS: usize = 120;
        // 允許用環境變數临时調大保險欄
        let safety_env = std::env::var("UROCISSA_TERM_MARGIN")
            .ok()
            .and_then(|v| v.parse::<usize>().ok())
            .unwrap_or(4);
        let cols = terminal_size()
            .map(|(Width(w), _)| w as usize)
            .unwrap_or(DEFAULT_COLS);

        /* ---------- 1. 前綴 + 後綴動態計算 ---------- */
        let short_hash = &self.hash.as_str()[..5.min(self.hash.len())];
        let prefix = format!("🔑 {:<5} 📂 ", short_hash);
        let prefix_w = UnicodeWidthStr::width(prefix.as_str());

        let secs = self.started.elapsed().as_secs();
        let suffix = format!(" ⏱️ {:>4}s", secs);
        let suffix_w = UnicodeWidthStr::width(suffix.as_str());

        /* ---------- 2. 可分配給路徑的欄位 ---------- */
        let path_budget = cols.saturating_sub(prefix_w + suffix_w + safety_env).max(5); // 至少留 5 欄給路徑

        /* ---------- 3. 路徑尾端裁切 ---------- */
        let raw_path = self.path.display().to_string();
        let short_path = Self::tail_ellipsis(&raw_path, path_budget);

        /* ---------- 4. 組合輸出 ---------- */
        format!(
            "{prefix}{:<width$}{suffix}",
            short_path,
            width = path_budget
        )
    }

    fn tail_ellipsis(s: &str, max: usize) -> String {
        if UnicodeWidthStr::width(s) <= max {
            return s.to_owned();
        }

        let tail_len = max.saturating_sub(1); // 1 格留給 ‘…’
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
pub struct Dashboard {
    tasks: Vec<TaskRow>,
}

pub static DASHBOARD: LazyLock<Arc<RwLock<Dashboard>>> =
    LazyLock::new(|| Arc::new(RwLock::new(Dashboard::new())));

impl Component for Dashboard {
    fn draw_unchecked(&self, _: Dimensions, _: DrawMode) -> anyhow::Result<Lines> {
        let mut lines = Vec::new();
        for t in &self.tasks {
            lines.push(vec![t.fmt()].try_into()?);
        }
        Ok(Lines(lines))
    }
}

impl Dashboard {
    // 建構一個空的 Dashboard
    pub fn new() -> Self {
        Dashboard { tasks: Vec::new() }
    }

    // 新增一個任務
    pub fn add_task(&mut self, hash: ArrayString<64>, path: PathBuf) {
        self.tasks.push(TaskRow {
            hash,
            path,
            started: Instant::now(),
        });
    }
}
