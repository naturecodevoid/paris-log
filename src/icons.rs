#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        $crate::__private_exports_do_not_use::__export_error!("{}", $crate::__private_exports_do_not_use::__export_colorize_string(format!("<red><cross></> {}", format!($($arg)*))))
    }
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
        $crate::__private_exports_do_not_use::__export_warn!("{}", $crate::__private_exports_do_not_use::__export_colorize_string(format!("<yellow><warn></> {}", format!($($arg)*))))
    }
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        $crate::__private_exports_do_not_use::__export_info!("{}", $crate::__private_exports_do_not_use::__export_colorize_string(format!("<cyan><info></> {}", format!($($arg)*))))
    }
}

#[macro_export]
macro_rules! success {
    ($($arg:tt)*) => {
        $crate::__private_exports_do_not_use::__export_info!("{}", $crate::__private_exports_do_not_use::__export_colorize_string(format!("<green><tick></> {}", format!($($arg)*))))
    }
}

#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {
        $crate::__private_exports_do_not_use::__export_debug!("{}", $crate::__private_exports_do_not_use::__export_colorize_string(format!($($arg)*)))
    }
}

#[macro_export]
macro_rules! trace {
    ($($arg:tt)*) => {
        $crate::__private_exports_do_not_use::__export_trace!("{}", $crate::__private_exports_do_not_use::__export_colorize_string(format!($($arg)*)))
    }
}
