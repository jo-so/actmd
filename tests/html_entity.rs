#![feature(assert_matches)]
#![feature(decl_macro)]

mod common;
use common::*;

cm_par_check!(
    with_empty_number, "&#;", plain!(<0, 3> "&#;")
);

cm_par_check!(
    with_long_number, "&#12345678;", plain!(<0, 11> "&#12345678;")
);

cm_par_check!(
    with_empty_hex, "&#x;", plain!(<0, 4> "&#x;")
);

cm_par_check!(
    with_long_hex, "&#x1234567;", plain!(<0, 11> "&#x1234567;")
);
