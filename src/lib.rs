#[macro_export]
macro_rules! api {
    ($($arg:tt)+) => {
        log::info!(target: "JacksSportsZoneApi", $($arg)+)
    };
}

#[macro_export]
macro_rules! updates {
    ($($arg:tt)+) => {
        log::debug!(target: "JacksSportsZoneApi", $($arg)+)
    };
}

#[macro_export]
macro_rules! update_error {
    ($($arg:tt)+) => {
        log::error!(target: "JacksSportsZoneApi", $($arg)+)
    };
}