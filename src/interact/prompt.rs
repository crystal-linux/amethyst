use std::mem;

use crate::with_suspended_output;

use super::{theme::AmeTheme, Interact};

pub struct AmePrompt {
    question: String,
    default_yes: Option<bool>,
}

impl AmePrompt {
    /// Creates a new prompt
    pub fn new<Q: ToString>(question: Q) -> Self {
        Self {
            question: question.to_string(),
            default_yes: None,
        }
    }

    /// Sets the prompt to default to yes
    pub fn default_yes(&mut self) -> &mut Self {
        self.default_yes = Some(true);

        self
    }

    /// Sets the prompt to default to yes
    pub fn default_no(&mut self) -> &mut Self {
        self.default_yes = Some(false);

        self
    }
}

impl Interact for AmePrompt {
    type Result = bool;

    fn interact(&mut self) -> Self::Result {
        let mut dialog = dialoguer::Confirm::with_theme(AmeTheme::get());

        if let Some(def) = self.default_yes.take() {
            dialog.default(def);
        }

        dialog
            .with_prompt(mem::take(&mut self.question))
            .wait_for_newline(true);
        with_suspended_output!({ dialog.interact().unwrap() })
    }
}
