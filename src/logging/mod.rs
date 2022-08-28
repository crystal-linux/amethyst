use std::sync::Arc;

use lazy_static::lazy_static;
use tracing::Level;
use tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt;
use tracing_subscriber::Registry;

mod fmt_layer;
use fmt_layer::AmeFormatLayer;

use crate::internal::uwu_enabled;

use self::handler::LogHandler;
pub mod handler;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Verbosity {
    #[allow(dead_code)]
    Error = 0,
    #[allow(dead_code)]
    Warning = 1,
    Info = 2,
    Debug = 3,
    Trace = 4,
}

impl From<usize> for Verbosity {
    fn from(num_verbosity: usize) -> Self {
        match num_verbosity {
            0 => Self::Info,
            1 => Self::Debug,
            2 => Self::Trace,
            _ => Self::Info,
        }
    }
}

impl Verbosity {
    fn from_level(l: &Level) -> Self {
        match *l {
            Level::ERROR => Self::Error,
            Level::WARN => Self::Warning,
            Level::INFO => Self::Info,
            Level::DEBUG => Self::Debug,
            Level::TRACE => Self::Trace,
        }
    }
}

/// Initializes the tracing logger
/// Can be used for debug purposes _or_ verbose output
pub fn init_logger(verbosity: Verbosity) {
    let logger = get_logger();
    logger.set_verbosity(verbosity);
    logger.set_uwu_enabled(uwu_enabled());
    let ame_layer = AmeFormatLayer::new(logger);

    let subscriber = Registry::default().with(ame_layer);
    tracing::subscriber::set_global_default(subscriber).unwrap();
}

/// Returns the global logger instance
pub fn get_logger() -> Arc<LogHandler> {
    lazy_static! {
        static ref LOGGER: Arc<LogHandler> = Arc::new(LogHandler::default());
    }

    Arc::clone(&LOGGER)
}
