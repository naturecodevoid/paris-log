#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        $crate::__private_exports::error!("{}", $crate::__private_exports::colorize_string(format!("<red><cross></> {}", format!($($arg)*))))
    }
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
        $crate::__private_exports::warn!("{}", $crate::__private_exports::colorize_string(format!("<yellow><warn></> {}", format!($($arg)*))))
    }
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        $crate::__private_exports::info!("{}", $crate::__private_exports::colorize_string(format!("<cyan><info></> {}", format!($($arg)*))))
    }
}

#[macro_export]
macro_rules! success {
    ($($arg:tt)*) => {
        $crate::__private_exports::info!("{}", $crate::__private_exports::colorize_string(format!("<green><tick></> {}", format!($($arg)*))))
    }
}

#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {
        $crate::__private_exports::debug!("{}", $crate::__private_exports::colorize_string(format!($($arg)*)))
    }
}

#[macro_export]
macro_rules! trace {
    ($($arg:tt)*) => {
        $crate::__private_exports::trace!("{}", $crate::__private_exports::colorize_string(format!($($arg)*)))
    }
}
