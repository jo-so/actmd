#![feature(assert_matches)]
#![feature(decl_macro)]

mod common;
use common::*;

body_check!(
    /// An invalid HTML comment, but it satisfies the Commonmark Spec
    invalid_html_comment, "<!-->*foo*\nLine",
    html_block!(<0, 11> "<!-->*foo*\n"),
    paragraph!(<11, 15> plain!(<11, 15> "Line"))
);

body_check!(
    /// An invalid HTML processing instruction, but it satisfies the Commonmark Spec
    invalid_proc_inst, "<?>*foo*\nLine",
    html_block!(<0, 9> "<?>*foo*\n"),
    paragraph!(<9, 13> plain!(<9, 13> "Line"))
);

body_check!(
    html_block_starts_with_simple_tag,
    "  <meta />\n",
    html_block!(<0, 11> "  <meta />\n")
);

body_check!(
    html_block_starts_with_tag,
    "  <meta name=license content=\"CC BY-SA 4.0\" />\n",
    html_block!(<0, 47> "  <meta name=license content=\"CC BY-SA 4.0\" />\n")
);

body_check!(
    tag_with_space_end_block,
    "  <meta attr=val >\n",
    html_block!(<0, 19> "  <meta attr=val >\n")
);

body_check!(
    html_block_tag_with_boolean_attribute,
    "  <tag attr=\"val\" disabled attr=val>\n",
    html_block!(<0, 37> "  <tag attr=\"val\" disabled attr=val>\n")
);

body_check!(
    script, "<script>\nwindow.alert('hello');\n</script>",
    html_block!(<0, 41> "<script>\nwindow.alert('hello');\n</script>\n")
);

body_check!(
    style, "<style>\nbody {\n  color: #fff;\n}\n</style>",
    html_block!(<0, 40> "<style>\nbody {\n  color: #fff;\n}\n</style>\n")
);

body_check!(
    blockquote_not_ends_html,
    "<div>\n> This is not a blockquote\n</div>",
    html_block!(<0, 39> "<div>\n> This is not a blockquote\n</div>\n")
);

mod nohtml {
    use super::*;

    body_check!(
        div, ParserSettings::None,
        "<div>\ntext\n</div>",
        paragraph!(<0, 17>
            plain!(<0, 5> "<div>"), SoftBreak,
            plain!(<6, 10> "text"), SoftBreak,
            plain!(<11, 17> "</div>")
        )
    );

    body_check!(
        script, ParserSettings::None,
        "<script>\nwindow.alert('hello');\n</script>",
        paragraph!(<0, 41>
            plain!(<0, 8> "<script>"), SoftBreak,
            plain!(<9, 31> "window.alert('hello');"), SoftBreak,
            plain!(<32, 41> "</script>")
        )
    );

    body_check!(
        style, ParserSettings::None,
        "<style>\nbody {\n  color: #fff;\n}\n</style>",
        paragraph!(<0, 40>
            plain!(<0, 7> "<style>"), SoftBreak,
            plain!(<8, 14> "body {"), SoftBreak,
            plain!(<17, 29> "color: #fff;"), SoftBreak,
            plain!(<30, 31> "}"), SoftBreak,
            plain!(<32, 40> "</style>")
        )
    );
}
