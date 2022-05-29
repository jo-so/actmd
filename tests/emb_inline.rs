#![feature(assert_matches)]
#![feature(decl_macro)]

mod common;
use common::*;

par_check!(
    #[cfg_attr(feature = "location", ignore)]
    line_comment, "abc @// TEST\ndef",
    plain!(<0, 4> "abc "),
    emb_block!(<5, 13> "// TEST\n"),
    plain!(<13, 6> "def")
);

par_check!(
    #[cfg_attr(feature = "location", ignore)]
    line_comment_end, "abc @// TEST",
    plain!(<0, 4> "abc "), emb_block!(<5, 12> "// TEST")
);

par_check!(
    #[cfg_attr(feature = "location", ignore)]
    line_comment_end_nl, "abc @// TEST\n",
    plain!(<0, 4> "abc "), emb_block!(<5, 13> "// TEST\n")
);
