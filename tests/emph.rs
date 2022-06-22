#![feature(assert_matches)]
#![feature(decl_macro)]

mod common;
use common::*;

par_check!(
    end_between_content_punct,
    "*foo `x`*.",
    emph!(plain!(<1, 5> "foo "), code!(<6, 7> "x")),
    plain!(<9, 10> ".")
);

par_check!(
    across_link,
    "[*foo]()*",
    link!(<0, 0> "", "", plain!(<1, 5> "*foo")),
    plain!(<8, 9> "*")
);

par_check!(
    xacross_ref_label,
    "[foo][*]*",
    linkref!(<0, 0> "*", plain!(<1, 4> "foo")),
    plain!(<8, 9> "*")
);

cm_par_check!(
    cacross_ref_label,
    "[foo][*]*",
    linkref!(<0, 0> "*", plain!(<1, 4> "foo")),
    plain!(<8, 9> "*")
);

par_check!(
    cleanup_open_emph_at_par_end,
    "*a **b [ c** d* x",
    emph!(plain!(<1, 3> "a "), strong!(plain!(<5, 10> "b [ c")), plain!(<12, 14> " d")),
    plain!(<15, 17> " x")
);
