#[macro_export]
/// Macro for prompting the user with a yes/no question.
macro_rules! prompt {
    (default yes, $($arg:tt)+) => {
        $crate::interact::Interact::interact($crate::interact::AmePrompt::new(format!($($arg)+)).default_yes())
    };
    (default no, $($arg:tt)+) => {
        $crate::interact::Interact::interact($crate::interact::AmePrompt::new(format!($($arg)+)).default_no())
    };
    (no default, $($arg:tt)+) => {
        $crate::interact::Interact::interact($crate::interact::AmePrompt::new(format!($($arg)+)))
    }
}

#[macro_export]
/// Macro for prompting the user with a multi select
macro_rules! multi_select {
    ($items:expr, $($arg:tt)+) => {
        $crate::interact::Interact::interact($crate::interact::AmeMultiSelect::new(format!($($arg)+)).items($items))
    }
}
