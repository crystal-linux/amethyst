pub mod macros;
mod multi_select;
mod prompt;
mod theme;

pub use multi_select::AmeMultiSelect;
pub use prompt::AmePrompt;

pub trait Interact {
    type Result;

    fn interact(&mut self) -> Self::Result;
}
