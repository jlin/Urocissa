use env_logger::{Builder, WriteStyle};
use log::kv::Key;
use superconsole::style::Stylize;

use std::io::Write;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

use crate::public::tui::LOGGER_TX;

/// A `Write` adapter that sends each incoming line over a Tokio channel.
pub struct TokioPipe(pub UnboundedSender<String>);

impl std::io::Write for TokioPipe {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        // Decode bytes into a UTF-8 string, replacing invalid sequences
        let s = String::from_utf8_lossy(buf);
        // Split on newline, replace tabs, and send non-empty lines
        for line in s.split_terminator('\n') {
            let clean = line.replace('\t', "    ");
            if !clean.is_empty() {
                let _ = self.0.send(clean.to_string());
            }
        }
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        // No buffering, so nothing to flush
        Ok(())
    }
}

/// Initialize the logger and return a receiver for formatted log lines.
pub fn initialize_logger() -> UnboundedReceiver<String> {
    // Create a channel and save the sender globally
    let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
    LOGGER_TX.set(tx).unwrap();

    Builder::new()
        // Always include ANSI codes so StyledContent can reset itself
        .write_style(WriteStyle::Always)
        .format(|buf, record| {
            // Colorize timestamp in dark grey
            let ts = buf.timestamp().to_string().dark_grey();

            // Colorize level with default style (includes reset)
            let level_style = buf.default_level_style(record.level());
            let lvl = format!(
                "{}{}{}",
                level_style.render(),
                record.level(),
                level_style.render_reset()
            );

            // Colorize module target in dark grey
            let tgt = record.target().dark_grey();

            // Extract raw duration and format to 2 decimal places
            let dur_raw = record
                .key_values()
                .get(Key::from("duration"))
                .map(|v| {
                    let s = format!("{}", v);
                    if let Some(idx) = s.find(|c: char| c.is_alphabetic()) {
                        let (num, unit) = (&s[..idx], &s[idx..]);
                        if let Ok(val) = num.parse::<f32>() {
                            // Insert space between number and unit
                            return format!("{:.2} {}", val, unit);
                        }
                    }
                    s
                })
                .unwrap_or_default();

            // Right-align or pad the duration field to width 10, then color it cyan
            let dur = if dur_raw.is_empty() {
                " ".repeat(10)
            } else {
                format!("{:>10}", dur_raw).cyan().to_string()
            };

            // Write two lines: first is prefix, second is duration + message
            writeln!(buf, "{} {} {}\n{} {}", ts, lvl, tgt, dur, record.args())?;

            Ok(())
        })
        // Send formatted output through our custom pipe
        .target(env_logger::Target::Pipe(Box::new(TokioPipe(
            LOGGER_TX.get().unwrap().clone(),
        ))))
        // Only show INFO+ globally, WARN+ for Rocket
        .filter(None, log::LevelFilter::Info)
        .filter(Some("rocket"), log::LevelFilter::Warn)
        .init();

    rx
}
