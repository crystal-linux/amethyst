use colored::Colorize;
use std::collections::HashMap;
use std::sync::Arc;
use tracing_subscriber::registry::LookupSpan;

use tracing::field::Visit;
use tracing::{span, Level, Metadata, Subscriber};
use tracing_subscriber::Layer;

use super::handler::LogHandler;
use super::Verbosity;

const ENABLED_MODULES: &[&str] = &["ame"];

pub struct AmeFormatLayer {
    logger: Arc<LogHandler>,
}

impl AmeFormatLayer {
    pub fn new(logger: Arc<LogHandler>) -> Self {
        Self { logger }
    }

    fn is_level_loggable(&self, level: &Level) -> bool {
        self.logger.is_loggable(Verbosity::from_level(level))
    }

    fn is_enabled(&self, metadata: &Metadata) -> bool {
        let level = metadata.level();
        if !self.is_level_loggable(level) {
            false
        } else if let Some(module_path) = metadata.module_path() {
            ENABLED_MODULES.iter().any(|m| module_path.starts_with(m))
        } else {
            false
        }
    }

    fn log(&self, msg: String, level: &Level) {
        match Verbosity::from_level(level) {
            Verbosity::Error => self.logger.log_error(msg),
            Verbosity::Warning => self.logger.log_warning(msg),
            Verbosity::Info => self.logger.log_info(msg),
            Verbosity::Debug => self.logger.log_debug(msg),
            Verbosity::Trace => self.logger.log_trace(msg),
        }
    }
}

impl<S: Subscriber + for<'a> LookupSpan<'a>> Layer<S> for AmeFormatLayer {
    /// When entering a span
    fn on_new_span(
        &self,
        attrs: &span::Attributes<'_>,
        _id: &span::Id,
        _ctx: tracing_subscriber::layer::Context<'_, S>,
    ) {
        let metadata = attrs.metadata();
        if self.is_enabled(metadata) {
            let mut visitor = ValueDebugStorage::default();
            attrs.record(&mut visitor);
            let fields: Vec<String> = visitor
                .values
                .into_iter()
                .map(|(k, v)| format!("{k} = {v}"))
                .collect();
            let mut fields_str = fields.join("\n  ");

            if !fields_str.is_empty() {
                fields_str = format!("\n  {fields_str}");
            }

            if let Some(module) = metadata.module_path() {
                self.log(
                    format!(
                        "{} {}::{} {}",
                        "ENTER".italic(),
                        module,
                        metadata.name(),
                        fields_str.dimmed()
                    ),
                    metadata.level(),
                )
            } else {
                self.log(
                    format!(
                        "{} {} {}",
                        "ENTER".italic(),
                        metadata.name(),
                        fields_str.dimmed()
                    ),
                    metadata.level(),
                )
            }
        }
    }

    fn on_close(&self, id: span::Id, ctx: tracing_subscriber::layer::Context<'_, S>) {
        let span = ctx.span(&id).unwrap();
        let metadata = span.metadata();

        if self.is_enabled(metadata) {
            if let Some(module) = metadata.module_path() {
                self.log(
                    format!("{} {}::{}", "EXIT".italic(), module, metadata.name(),),
                    metadata.level(),
                );
            } else {
                self.log(
                    format!("{} {}", "EXIT".italic(), metadata.name()),
                    metadata.level(),
                );
            }
        }
    }

    fn on_event(
        &self,
        event: &tracing::Event<'_>,
        _ctx: tracing_subscriber::layer::Context<'_, S>,
    ) {
        let metadata = event.metadata();

        if self.is_enabled(metadata) {
            let mut visitor = ValueDebugStorage::default();
            event.record(&mut visitor);
            let mut values = visitor.values;

            if let Some(msg) = values.remove("message") {
                self.log(msg, metadata.level())
            }
        }
    }
}

#[derive(Default)]
pub struct ValueDebugStorage {
    pub values: HashMap<String, String>,
}

impl Visit for ValueDebugStorage {
    fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn std::fmt::Debug) {
        self.values
            .insert(field.name().to_string(), format!("{:?}", value));
    }
}
