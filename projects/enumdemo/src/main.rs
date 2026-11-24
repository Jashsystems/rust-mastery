enum LogLevel {
    Error,
    Warn,
    Info,
}

fn level_of(line: &str) -> LogLevel {
    if line.contains("ERROR") {
        LogLevel::Error
    } else if line.contains("WARN") {
        LogLevel::Warn
    } else {
        LogLevel::Info
    }
}

fn main() {
    let text = "ERROR: Something failed
WARN: Low memory
INFO: Application started
ERROR: Connection lost
WARN: Disk space low";

    let mut error_count = 0;
    let mut warn_count = 0;
    let mut info_count = 0;

    for line in text.lines() {
        let level = level_of(line);

        match level {
            LogLevel::Error => error_count += 1,
            LogLevel::Warn => warn_count += 1,
            LogLevel::Info => info_count += 1,
        }
    }

    println!("Errors: {}", error_count);
    println!("Warnings: {}", warn_count);
    println!("Info: {}", info_count);
}