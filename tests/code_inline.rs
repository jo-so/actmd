#![feature(assert_matches)]
#![feature(decl_macro)]

mod common;
use common::*;

par_check!(
    /// https://spec.commonmark.org/0.29/#insecure-characters
    with_0, "`abc\0def`", code!("abc\u{fffd}def")
);
