pub mod macros;
mod multi_select;
mod prompt;
mod theme;

pub use multi_select::MultiSelect;
pub use prompt::Prompt;

pub trait Interact {
    type Result;

    fn interact(&mut self) -> Self::Result;
}
