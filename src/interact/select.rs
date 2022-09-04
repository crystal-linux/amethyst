use std::mem;

use crate::with_suspended_output;

use super::{theme::AmeTheme, Interact, InteractOpt};

pub struct AmeFuzzySelect {
    prompt: String,
    items: Vec<String>,
}

impl AmeFuzzySelect {
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

    fn build(&mut self) -> dialoguer::FuzzySelect {
        let mut select = dialoguer::FuzzySelect::with_theme(AmeTheme::get());
        select
            .with_prompt(mem::take(&mut self.prompt))
            .items(&self.items)
            .default(0);

        select
    }
}

impl Interact for AmeFuzzySelect {
    type Result = usize;

    fn interact(&mut self) -> Self::Result {
        with_suspended_output!({ self.build().interact().unwrap() })
    }
}

impl InteractOpt for AmeFuzzySelect {
    fn interact_opt(&mut self) -> Option<Self::Result> {
        with_suspended_output!({ self.build().interact_opt().unwrap() })
    }
}
