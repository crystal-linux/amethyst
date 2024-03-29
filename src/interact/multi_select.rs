use std::mem;

use crate::with_suspended_output;

use super::{theme::AmeTheme, Interact};

pub struct AmeMultiSelect {
    prompt: String,
    items: Vec<String>,
}

impl AmeMultiSelect {
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

impl Interact for AmeMultiSelect {
    type Result = Vec<usize>;

    fn interact(&mut self) -> Self::Result {
        with_suspended_output!({
            dialoguer::MultiSelect::with_theme(AmeTheme::get())
                .with_prompt(mem::take(&mut self.prompt))
                .items(&self.items)
                .interact()
                .unwrap()
        })
    }
}
