#[macro_export]
#[allow(unused_macros)]
macro_rules! dbg {
    ($first_val:expr, $($val:expr),+ $(,)?) => {
        if cfg!(debug_assertions) {
            eprint!("[{}:{}] {} = {:?}",
                        file!(), line!(), stringify!($first_val), &$first_val);
            ($(eprint!(", {} = {:?}", stringify!($val), &$val)),+,);
            eprintln!();
        }
    };
    ($first_val:expr) => {
        if cfg!(debug_assertions) {
            eprintln!("[{}:{}] {} = {:?}",
                        file!(), line!(), stringify!($first_val), &$first_val)
        }
    };
}
