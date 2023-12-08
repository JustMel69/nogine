use colored::Colorize;

#[macro_export]
macro_rules! log_info {
    ($($x:tt)*) => {
        $crate::logging::Logger::push(format!($($x)*), $crate::logging::LogType::Info, file!(), line!())
    };
}

#[macro_export]
macro_rules! log_warn {
    ($($x:tt)*) => {
        $crate::logging::Logger::push(format!($($x)*), $crate::logging::LogType::Warn, file!(), line!())
    };
}

#[macro_export]
macro_rules! log_error {
    ($($x:tt)*) => {
        $crate::logging::Logger::push(format!($($x)*), $crate::logging::LogType::Error, file!(), line!())
    };
}

#[macro_export]
macro_rules! unwrap_res {
    ($e:expr) => {
        match $e {
            Ok(x) => x,
            Err(e) => {
                $crate::log_error!("{}", e);
                std::process::exit(1);
            },
        }
    };
}

#[macro_export]
macro_rules! unwrap_opt {
    ($e:expr) => {
        $e.match {
            Some(x) => x,
            None => {
                $crate::log_error!("Unwrapped on 'None'.");
                std::process::exit(1);
            },
        }
    };
}

#[macro_export]
macro_rules! assert_expr {
    ($e:expr, $($tt:tt)+) => {
        if !$e {
            $crate::log_error!($($tt)+);
            std::process::exit(1);
        }
    };
}

#[derive(Debug, Clone, Copy)]
pub enum LogType {
    Info, Warn, Error
}

pub struct Logger;

impl Logger {
    pub(super) fn init() {
        colored::control::set_override(true);

        #[cfg(windows)]
        colored::control::set_virtual_terminal(true).unwrap();
    }

    pub fn push(msg: String, kind: LogType, file: &str, line: u32) {
        let metadata = format!("[{file}, Ln {line}]");
        match kind {
            LogType::Info => println!("[INFO] {}", msg),
            LogType::Warn => eprintln!("{} {}\n{}", "[WARN]".bright_yellow(), msg.bright_yellow(), metadata.bright_yellow()),
            LogType::Error => eprintln!("{} {}\n{}", "[ERR.]".red(), msg.red(), metadata.red()),
        }
    }
}