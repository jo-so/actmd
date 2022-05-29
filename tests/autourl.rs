#![feature(assert_matches)]
#![feature(decl_macro)]

mod common;
use common::*;

par_check!(
    invalid_start, "<+http://example.org>", plain!(<0, 21> "<+http://example.org>")
);

cm_par_check!(
    email_without_name, "<@example.org>", plain!(<0, 14> "<@example.org>")
);

par_check!(
    email_without_domain, "<user@>", plain!(<0, 7> "<user@>")
);

par_check!(
    too_short_scheme, "<x://example.org>", plain!(<0, 17> "<x://example.org>")
);

par_check!(
    short_scheme, "<xy://example.org>",
    link!("xy://example.org", "", plain!(<1, 17> "xy://example.org"))
);

par_check!(
    long_scheme,
    "<abcdefghijklmnopqrstuvwxyzABCDEF://example.org>",
    link!(
        "abcdefghijklmnopqrstuvwxyzABCDEF://example.org",
        "",
        plain!(<1, 47> "abcdefghijklmnopqrstuvwxyzABCDEF://example.org")
    )
);

par_check!(
    too_long_scheme,
    "<abcdefghijklmnopqrstuvwxyzABCDEFG://example.org>",
    plain!(<0, 49> "<abcdefghijklmnopqrstuvwxyzABCDEFG://example.org>")
);
