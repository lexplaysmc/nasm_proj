use std::{fmt::Display, process::exit};

use colored::Colorize;

pub trait Expect<T, R, I> {
    fn expect_np(self: Self, msg: &str) -> I;
}
// impl<T, E> Expect<Result<T, E>, E, T> for Result<T, E> {
//     default fn expect_np(self: Self, msg: &str) -> T
//     {
//         match self {
//             Ok(t) => {
//                 return t;
//             }
//             Err(_) => {
//                 error(msg)
//             }
//         }
//     }
// }
impl<T, E: Display> Expect<Result<T, E>, E, T> for Result<T, E> {
    fn expect_np(self, msg: &str) -> T
    {
        match self {
            Ok(t) => {
                return t;
            }
            Err(t) => {
                error(&format!("{msg}\n{t}"))
            }
        }
    }
}
impl<T> Expect<Option<T>, (), T> for Option<T> {
    fn expect_np(self, msg: &str) -> T
    {
        match self {
            Some(t) => {
                return t;
            }
            None=> {
                error(&format!("{msg}"))
            }
        }
    }
}


pub fn error(msg: &str) -> ! {
    eprintln!("{}{}", "ERROR: ".color("red"), msg.color("red"));
    exit(1);
}