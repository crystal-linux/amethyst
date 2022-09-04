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
/// Usage:
/// ```rust
/// let some_list = vec!["a", "b", "c"];
/// format!("The list has {}", numeric!(some_list.len(), "element"["s"]));
/// // result: The list has 3 elements
///
/// let some_other_list = vec!["a"];
/// format!("The list has {}", numeric!(some_other_list.len(), "element"["s"]));
/// // result: The list has 1 element
/// ```
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

#[macro_export]
/// Creates a new multiprogress bar
macro_rules! multi_progress {
    () => {
        $crate::logging::get_logger().new_multi_progress();
    };
}

#[macro_export]
/// Creates a new progress spinner
macro_rules! spinner {
    () => {
        $crate::logging::get_logger().new_progress_spinner()
    };
    ($($arg:tt)+) => {
        {
            let spinner = $crate::spinner!();
            spinner.set_message(format!($($arg)+));
            spinner
        }
    }
}

#[macro_export]
/// Resets the output to normal text output (erases all progress bars and spinners)
macro_rules! normal_output {
    () => {
        $crate::logging::get_logger().reset_output_type();
    };
}

#[macro_export]
/// Suspends the output so that nothing is being written to stdout/stderr
macro_rules! supend_output {
    () => {
        $crate::loggign::get_logger().suspend();
    };
}

#[macro_export]
/// Unsuspends the output and writes everything buffered to stdout/stderr
macro_rules! unsupend_output {
    () => {
        $crate::loggign::get_logger().unsuspend();
    };
}
