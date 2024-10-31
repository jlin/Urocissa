use anstyle::Color;
use env_logger::fmt::style::Style;
use env_logger::Builder;
use log::kv::Key;
use log::LevelFilter;
use std::env;
use std::io::Write;
use std::path::PathBuf;
pub fn initialize_logger() {
    env::set_var("RUST_LOG", "INFO");
    Builder::new()
        .format(|buf, record| {
            let custom_value = record
                .key_values()
                .get(Key::from("duration"))
                .map(|v| {
                    // If custom_value contains both numbers and units, add a space between them
                    let value_str = format!("{}", v);
                    if let Some(index) = value_str.find(|c: char| c.is_alphabetic()) {
                        format!("{} {}", &value_str[..index], &value_str[index..])
                    } else {
                        value_str
                    }
                })
                .unwrap_or_default(); // Return an empty string if "duration" is not found

            // Set style: bold blue for custom_value
            let custom_value_style = Style::new()
                .bold() // Set bold style
                .fg_color(Some(Color::Ansi(anstyle::AnsiColor::Blue))); // Set blue foreground color

            // Set style: gray for timestamp and target
            let grey_style =
                Style::new().fg_color(Some(Color::Ansi(anstyle::AnsiColor::BrightBlack))); // Set gray foreground color

            // Begin writing to buffer, combining all content into one write!
            writeln!(
                buf,
                "{}{}{}{} {}{}{} {}{}{}\n{:>10}{} {}",
                // Render timestamp in gray style
                grey_style.render(),
                buf.timestamp(),           // Directly use the timestamp method
                grey_style.render_reset(), // Reset gray style
                // Render level color
                buf.default_level_style(record.level()).render(), // Begin rendering Level style
                record.level(),                                   // Actual Level content
                buf.default_level_style(record.level()).render_reset(), // End Level style rendering
                // Render target in gray style
                grey_style.render(),
                record.target(),           // Target is displayed after Level
                grey_style.render_reset(), // Reset gray style
                // Render custom_value in bold blue style
                custom_value_style.render(),
                custom_value,                      // Write custom_value
                custom_value_style.render_reset(), // Reset custom_value style
                // Write log message
                record.args()
            )?;

            Ok(())
        })
        .filter(None, LevelFilter::Info) // Set minimum Level to Warn for all modules
        .filter(Some("rocket"), LevelFilter::Warn)
        .init();
}

pub fn initialize_folder() {
    std::fs::create_dir_all(PathBuf::from("./db")).unwrap();
    std::fs::create_dir_all(PathBuf::from("./object/imported")).unwrap();
    std::fs::create_dir_all(PathBuf::from("./object/compressed")).unwrap();
    std::fs::create_dir_all(PathBuf::from("upload")).unwrap();
}
