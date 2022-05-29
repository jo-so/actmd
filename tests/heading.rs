#![feature(assert_matches)]
#![feature(decl_macro)]

mod common;
use common::*;

body_check!(
    /// https://spec.commonmark.org/0.29/#insecure-characters
    #[cfg_attr(feature = "location", ignore)]
    headline_with_0, "# \0text", heading!(<0, 7> 1, plain!(<2, 7> "\u{fffd}text"))
);

body_check!(
    with_open_emph, "# test *bla\nfoo*",
    heading!(<0, 12> 1, plain!(<2, 11> "test *bla")),
    paragraph!(<12, 16> plain!(<12, 16> "foo*"))
);

body_check!(
    with_open_code, "# test `bla\nfoo`",
    heading!(<0, 12> 1, plain!(<2, 11> "test `bla")),
    paragraph!(<12, 16> plain!(<12, 16> "foo`"))
);

body_check!(
    after_hard_break,
    "Foo\\\n# 12\ntext",
    paragraph!(<0, 5> plain!(<0, 4> "Foo\\")),
    heading!(<5, 10> 1, plain!(<7, 9> "12")),
    paragraph!(<10, 14> plain!(<10, 14> "text"))
);

cm_body_check!(
    /// Block structure takes precedence over inline structure
    /// <https://spec.commonmark.org/0.29/#example-61>
    #[cfg_attr(feature = "location", ignore)]
    heading_breaks_inline_code,
    "`*Foo*\n# Heading\nnot inline code`",
    paragraph!(<0, 7> plain!(<0, 1> "`"), emph!(plain!(<2, 5> "Foo"))),
    heading!(<7, 17> 1, plain!(<9, 16> "Heading")),
    paragraph!(<17, 33> plain!(<17, 33> "not inline code`"))
);

cm_body_check!(
    /// Block structure takes precedence over inline structure
    /// <https://spec.commonmark.org/0.29/#example-61>
    #[cfg_attr(feature = "location", ignore)]
    headline_breaks_link,
    "[*Foo*\n# 12\n](https://example.org/)",
    paragraph!(<0, 7> plain!(<0, 1> "["), emph!(plain!(<2, 5> "Foo"))),
    heading!(<7, 12> 1, plain!(<9, 11> "12")),
    paragraph!(<12, 35> plain!(<12, 35> "](https://example.org/)"))
);
