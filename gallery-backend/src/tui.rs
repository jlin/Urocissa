use arrayvec::ArrayString;
use bytesize::ByteSize;
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

use superconsole::{Component, Dimensions, DrawMode, Line, Lines, SuperConsole};

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
    pub tasks: Vec<TaskRow>,
    pub handled: u64,  // ✔ 已完成計數
    pub db_bytes: u64, // 💾 目前 DB 佔用
}

pub static DASHBOARD: LazyLock<Arc<RwLock<Dashboard>>> =
    LazyLock::new(|| Arc::new(RwLock::new(Dashboard::new())));

impl Component for Dashboard {
    fn draw_unchecked(&self, _: Dimensions, _: DrawMode) -> anyhow::Result<Lines> {
        let cols = terminal_size()
            .map(|(Width(w), _)| w as usize)
            .unwrap_or(120);
        let sep = "─".repeat(cols);

        let mut lines = Vec::<Line>::new();

        // 第一條線
        lines.push(vec![sep.clone()].try_into()?);

        // 📊 統計列 —─ 動態欄寬
        let human = ByteSize(self.db_bytes).to_string();
        let stats = format!(
            "📊 已處理：{:<6} │  💾 DB 使用： {:>8}",
            self.handled, human
        );
        lines.push(vec![stats].try_into()?);

        // 第二條線
        lines.push(vec![sep].try_into()?);

        // 任務清單（同前，最多五筆）
        for t in self.tasks.iter().take(5) {
            lines.push(vec![t.fmt()].try_into()?);
        }
        let remain = self.tasks.len().saturating_sub(5);
        if remain > 0 {
            lines.push(vec![format!("… 其餘 {remain} 筆任務")].try_into()?);
        }
        Ok(Lines(lines))
    }
}

impl Dashboard {
    /// 建構空 Dashboard
    pub fn new() -> Self {
        Dashboard {
            tasks: Vec::new(),
            handled: 0,
            db_bytes: 0,
        }
    }

    /// 新增/覆寫同雜湊任務
    pub fn add_task(&mut self, hash: ArrayString<64>, path: PathBuf) {
        // 若雜湊已存在就覆寫路徑並重置計時
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

    /// 處理完畢後移除
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
