use i18n_embed::{
    fluent::{fluent_language_loader, FluentLanguageLoader},
    DesktopLanguageRequester,
};
use lazy_static::lazy_static;
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "i18n"]
struct Localizations;

fn read() -> FluentLanguageLoader {
    let loader: FluentLanguageLoader = fluent_language_loader!();
    let req_langs = DesktopLanguageRequester::requested_languages();
    i18n_embed::select(&loader, &Localizations, &req_langs).unwrap();
    loader
}

lazy_static! {
    pub static ref LANG_LOADER: FluentLanguageLoader = read();
}

#[macro_export]
macro_rules! fl {
    ($message_id:literal) => {{
        i18n_embed_fl::fl!($crate::internal::i18n::LANG_LOADER, $message_id)
    }};

    ($message_id:literal, $($args:expr),*) => {{
        i18n_embed_fl::fl!($crate::internal::i18n::LANG_LOADER, $message_id, $($args), *)
    }};
}

#[macro_export]
macro_rules! fl_debug {
    ($message_id:literal) =>  {
        tracing::debug!("{}", $crate::fl!($message_id))
    };

    ($message_id:literal, $($arg:tt)*) => {
        tracing::debug!("{}", $crate::fl!($message_id, $($args)*))
    };
}

#[macro_export]
macro_rules! fl_trace {
    ($message_id:literal) =>  {
        tracing::trace!("{}", $crate::fl!($message_id))
    };

    ($message_id:literal, $($arg:tt)*) => {
        tracing::trace!("{}", $crate::fl!($message_id, $($args)*))
    };
}

#[macro_export]
macro_rules! fl_info {
    ($message_id:literal) =>  {
        tracing::info!("{}", $crate::fl!($message_id))
    };

    ($message_id:literal, $($arg:tt)*) => {
        tracing::info!("{}", $crate::fl!($message_id, $($arg)*))
    };
}

#[macro_export]
macro_rules! fl_warn {
    ($message_id:literal) =>  {
        tracing::warn!("{}", $crate::fl!($message_id))
    };

    ($message_id:literal, $($arg:tt)*) => {
        tracing::warn!("{}", $crate::fl!($message_id, $($arg)*))
    };
}

#[macro_export]
macro_rules! fl_error {
    ($message_id:literal) =>  {
        tracing::error!("{}", $crate::fl!($message_id))
    };

    ($message_id:literal, $($arg:tt)*) => {
        tracing::error!("{}", $crate::fl!($message_id, $($args)*))
    };
}
