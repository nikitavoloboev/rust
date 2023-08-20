// nicer print macro, just print variable name and value (stripped from release builds)
#[macro_export]
macro_rules! log {
    ($var:expr) => {
        #[cfg(debug_assertions)]
        {
            println!("{}: {:?}", stringify!($var), $var);
        }
    };
}
