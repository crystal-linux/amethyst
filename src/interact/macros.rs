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

#[macro_export]
/// Returns a singular or plural expression depending on the given len
macro_rules! numeric {
    ($len:expr, $sin:literal[$plu:literal]) => {
        if $len == 1 {
            format!("{} {}", $len, $sin)
        } else {
            format!("{} {}{}", $len, $sin, $plu)
        }
    };
    ($len:expr, $sin:literal or $plu:literal) => {
        if $len == 1 {
            format!("{} {}", $len, $sin)
        } else {
            format!("{} {}", $len, plu)
        }
    };
}
