#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        $crate::__private::error!("{}", $crate::__private::colorize_string(format!($($arg)*)))
    }
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
        $crate::__private::warn!("{}", $crate::__private::colorize_string(format!($($arg)*)))
    }
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        $crate::__private::info!("{}", $crate::__private::colorize_string(format!($($arg)*)))
    }
}

#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {
        $crate::__private::debug!("{}", $crate::__private::colorize_string(format!($($arg)*)))
    }
}

#[macro_export]
macro_rules! trace {
    ($($arg:tt)*) => {
        $crate::__private::trace!("{}", $crate::__private::colorize_string(format!($($arg)*)))
    }
}
