/* -*- mode:rust; coding:utf-8-unix; -*- */

//! unwrap.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/05/06
//  @date 2016/05/28

/* ////////////////////////////////////////////////////////////////////////// */
/* ========================================================================== */
/// unwrap!
#[cfg(not(debug_assertions))]
#[macro_export]
macro_rules! unwrap {
    ($e:expr)                           => (($e).unwrap());
    ($e:expr, $msg:expr)                => (($e).unwrap());
    ($e:expr, $fmt:expr, $($args:tt)+)  => (($e).unwrap());
}
/* -------------------------------------------------------------------------- */
/// unwrap!
#[cfg(debug_assertions)]
#[macro_export]
macro_rules! unwrap {
    ($e:expr)                           => {
        ($e).expect(concat!(module_path!(), ": ", file!(), "(", line!(), "): "))
    };
    ($e:expr, $msg:expr)                => {
        unwrap!($e, "{}", $msg)
    };
    ($e:expr, $fmt:expr, $($args:tt)+)  => {
        ($e).expect(&format!(concat!(module_path!(), ": ", file!(),
                                     "(", line!(), "): ", $fmt), $($args)+))
    };
}
