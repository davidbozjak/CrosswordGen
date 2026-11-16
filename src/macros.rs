#[macro_export]
macro_rules! debug_panic_warn {
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)]
        panic!($($arg)*);

        #[cfg(not(debug_assertions))]
        eprintln!($($arg)*);
    };
}
