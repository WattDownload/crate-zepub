pub(crate) static mut IS_LOG: bool = false;

pub(crate) fn set_enable_log(value: bool) {
    unsafe { IS_LOG = value };
}
#[macro_export]
macro_rules! msg {
    ( $($arg:tt)+) => {
        unsafe{
            if $crate::log::IS_LOG {
                println!($($arg)+)
            }
        }

    };
}

#[macro_export]
macro_rules! exec_err {
    ($($arg:tt)*) => {{
        #[cfg(not(test))]
        {
            eprintln!($($arg)*);
            std::process::exit(1);
        }
        #[cfg(test)]
        panic!($($arg)*);

    }};

}
