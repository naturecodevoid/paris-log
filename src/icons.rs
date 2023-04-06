#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        ::log::error!("{}", ::paris::formatter::colorize_string(format!("<red><cross></> {}", format!($($arg)*))))
    }
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
        ::log::warn!("{}", ::paris::formatter::colorize_string(format!("<yellow><warn></> {}", format!($($arg)*))))
    }
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        ::log::info!("{}", ::paris::formatter::colorize_string(format!("<cyan><info></> {}", format!($($arg)*))))
    }
}

#[macro_export]
macro_rules! success {
    ($($arg:tt)*) => {
        ::log::info!("{}", ::paris::formatter::colorize_string(format!("<green><tick></> {}", format!($($arg)*))))
    }
}

#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {
        ::log::debug!("{}", ::paris::formatter::colorize_string(format!($($arg)*)))
    }
}

#[macro_export]
macro_rules! trace {
    ($($arg:tt)*) => {
        ::log::trace!("{}", ::paris::formatter::colorize_string(format!($($arg)*)))
    }
}
