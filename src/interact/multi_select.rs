use std::mem;

use crate::logging::get_logger;

use super::{theme::AmeTheme, Interact};

pub struct MultiSelect {
    prompt: String,
    items: Vec<String>,
}

impl MultiSelect {
    /// Creates a new multi select prompt
    pub fn new<S: ToString>(prompt: S) -> Self {
        Self {
            prompt: prompt.to_string(),
            items: Vec::new(),
        }
    }

    /// Adds/replaces the items of this multi select
    pub fn items<I: IntoIterator<Item = S>, S: ToString>(&mut self, items: I) -> &mut Self {
        self.items = items.into_iter().map(|i| i.to_string()).collect();

        self
    }
}

impl Interact for MultiSelect {
    type Result = Vec<usize>;

    fn interact(&mut self) -> Self::Result {
        get_logger().suspend();
        let selection = dialoguer::MultiSelect::with_theme(AmeTheme::get())
            .with_prompt(mem::take(&mut self.prompt))
            .items(&self.items)
            .interact()
            .unwrap();
        get_logger().unsuspend();

        selection
    }
}
