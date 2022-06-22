#![feature(assert_matches)]
#![feature(decl_macro)]

mod common;
use common::*;

par_check!(
    empty, "",
);

par_check!(
    only_spaces, "    ",
);

par_check!(
    /// https://spec.commonmark.org/0.29/#insecure-characters
    #[cfg_attr(feature = "location", ignore)]
    text_with_0_begin, "\0x", plain!(<0, 2> "\u{fffd}x")
);

par_check!(
    /// https://spec.commonmark.org/0.29/#insecure-characters
    #[cfg_attr(feature = "location", ignore)]
    text_with_0_middle, "x\0y", plain!(<0, 3> "x\u{fffd}y")
);

par_check!(
    /// https://spec.commonmark.org/0.29/#insecure-characters
    #[cfg_attr(feature = "location", ignore)]
    text_with_0_end, "x\0", plain!(<0, 2> "x\u{fffd}")
);

par_check!(
    whitespace_at_line_begin_end,
    "x\n    \x0b\x0c    Foo  \t  \x0b\x0c   \ny",
    plain!(<0, 1> "x"), SoftBreak,
    plain!(<6, 22> "\x0b\x0c    Foo  \t  \x0b\x0c"), SoftBreak,
    plain!(<26, 27> "y")
);

par_check!(
    whitespace_before_softbreak,
    "x  \t \ny", plain!(<0, 1> "x"), SoftBreak, plain!(<6, 7> "y")
);

par_check!(
    whitespace_before_hardbreak,
    "x  \t  \\\ny", plain!(<0, 6> "x  \t  "), HardBreak, plain!(<8, 9> "y")
);

body_check!(
    paragraph_end_after_hard_break,
    "hard break\\\n\ntext",
    paragraph!(<0, 12> plain!(<0, 11> "hard break\\")),
    paragraph!(<13, 17> plain!(<13, 17> "text"))
);

cm_par_check!(
    /// Derived from https://spec.commonmark.org/0.29/#example-196
    t196b, "foo   &#32;   ", plain!(<0, 11> "foo    ")
);

cm_par_check!(
    /// Derived from https://spec.commonmark.org/0.29/#example-333
    t333b, "`\tb\t`", code!(<1, 4> "\tb\t")
);

cm_par_check!(
    /// Derived from https://spec.commonmark.org/0.29/#example-335
    t335b,
    "``\r\nfoo\r\nbar  \r\nbaz\r\n``", code!(<2, 21> "foo bar   baz")
);

cm_par_check!(
    /// Derived from https://spec.commonmark.org/0.29/#example-336
    t336b, "``\r\nfoo \r\n``", code!(<2, 10> "foo ")
);

cm_par_check!(
    /// Derived from https://spec.commonmark.org/0.29/#example-337
    t337b, "`foo   bar \r\nbaz`", code!(<1, 16> "foo   bar  baz")
);

par_check!(
    no_html_in_paragraph, ParserSettings::None,
    "text <a> or <b/>", plain!(<0, 16> "text <a> or <b/>")
);
