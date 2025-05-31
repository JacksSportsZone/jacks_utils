#[macro_export]
macro_rules! collegehockeyzoneapi {
    ($($arg:tt)+) => {
        log::info!(target: "JacksSportsZoneApi", $($arg)+)
    };
}

#[macro_export]
macro_rules! collegehockeyzoneupdates {
    ($($arg:tt)+) => {
        log::debug!(target: "JacksSportsZoneApi", $($arg)+)
    };
}

#[macro_export]
macro_rules! collegehockeyzoneupdateerror {
    ($($arg:tt)+) => {
        log::error!(target: "JacksSportsZoneApi", $($arg)+)
    };
}