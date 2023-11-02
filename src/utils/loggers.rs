#![allow(unused)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

/*
    Macro that will print message with source code line number
*/
#[macro_export]
macro_rules! log_writer {
    ($($arg:tt)*) => {
        let (file, line) = (file!(), line!());
        println!("[{:?}:{:?}] {:?}", file, line, format_args!($($arg)*));
    };
}