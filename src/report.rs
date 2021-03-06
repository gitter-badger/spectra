#[macro_export]
macro_rules! now {
  () => {{
    use chrono::{Datelike, Local, Timelike};
    let t = Local::now();
    
    format!("{month:0>2}/{day:0>2}/{year} {hour:0>2}:{min:0>2}:{secs:0>2}:{nsecs:0>9}",
            month = t.month(),
            day = t.day(),
            year = t.year(),
            hour = t.hour(),
            min = t.minute(),
            secs = t.second(),
            nsecs = t.nanosecond())
  }}
}

#[macro_export]
macro_rules! deb {
  ( $e:expr ) => { println!(concat!("\x1b[90m{} \x1b[36m> ", $e, "\x1b[0m"), now!()); };
  ( $e:expr, $($arg:tt)+ ) => { println!(concat!("\x1b[90m{} \x1b[36m> ", $e, "\x1b[0m"), now!(), $($arg)+); };
}

#[macro_export]
macro_rules! info {
  ( $e:expr ) => { println!(concat!("\x1b[90m{} \x1b[34m> ", $e, "\x1b[0m"), now!()); };
  ( $e:expr, $($arg:tt)+ ) => { println!(concat!("\x1b[90m{} \x1b[34m> ", $e, "\x1b[0m"), now!(), $($arg)+); };
}

#[macro_export]
macro_rules! warn {
  ( $e:expr ) => { println!(concat!("\x1b[90m{} \x1b[33m> ", $e, "\x1b[0m"), now!()); };
  ( $e:expr, $($arg:tt)+ ) => { println!(concat!("\x1b[90m{} \x1b[33m> ", $e, "\x1b[0m"), now!(), $($arg)+); };
}

#[macro_export]
macro_rules! err {
  ( $e:expr ) => { println!(concat!("\x1b[90m{} \x1b[1;31m> ", $e, "\x1b[0;0m"), now!()); };
  ( $e:expr, $($arg:tt)+ ) => { println!(concat!("\x1b[90m{} \x1b[1;31m> ", $e, "\x1b[0;0m"), now!(), $($arg)+); };
}
