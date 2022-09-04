pub mod macros;
mod multi_select;
mod prompt;
mod select;
mod theme;

pub use multi_select::AmeMultiSelect;
pub use prompt::AmePrompt;
pub use select::AmeFuzzySelect;

pub trait Interact {
    type Result;

    fn interact(&mut self) -> Self::Result;
}

pub trait InteractOpt: Interact {
    fn interact_opt(&mut self) -> Option<Self::Result>;
}
