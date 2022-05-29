#![feature(assert_matches)]
#![feature(decl_macro)]

mod common;
use common::*;

body_check!(
    trailing_space_in_fenced_code,
    "~~~\nline 1   \n~~~", code_block!(<0, 17> "", "line 1   \n")
);
