#[macro_export]
macro_rules! log {
    ($var:expr) => {
        println!("{}: {:?}", stringify!($var), $var);
    };
}
