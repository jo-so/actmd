#![feature(assert_matches)]
#![feature(decl_macro)]

mod common;
use common::*;

par_check!(
    trim_space_bol,
    "text\n   text",
    plain!(<0, 4> "text"), SoftBreak, plain!(<8, 12> "text")
);

par_check!(
    trim_space_eol,
    "text   \ntext",
    plain!(<0, 4> "text"), SoftBreak, plain!(<8, 12> "text")
);

par_check!(
    hardbreak,
    "text\\\ntext",
    plain!(<0, 4> "text"), HardBreak, plain!(<6, 10> "text")
);

par_check!(
    keep_eol_escaped_backslash,
    "text\\\\\ntext",
    plain!(<0, 6> "text\\"), SoftBreak, plain!(<7, 11> "text")
);

par_check!(
    keep_eol_html_entity_backslash,
    "text&#92;\ntext",
    plain!(<0, 9> "text\\"), SoftBreak, plain!(<10, 14> "text")
);

par_check!(
    keep_eol_html_entity_space,
    "text  &#32;\ntext",
    plain!(<0, 11> "text   "), SoftBreak, plain!(<12, 16> "text")
);

par_check!(
    whitespace_before_hardbreak,
    "text   \\\ntext",
    plain!(<0, 7> "text   "), HardBreak, plain!(<9, 13> "text")
);
