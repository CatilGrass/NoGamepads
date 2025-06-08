#[macro_export]
macro_rules! entry_mutex {
    ($mutex:expr, |$guard:ident| $code:expr) => {
        if let Ok(mut $guard) = $mutex.lock() {
            let $guard = &mut $guard;
            $code
        }
    };
}