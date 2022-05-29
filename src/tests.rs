#![cfg(not(feature = "location"))]

use crate::*;
use crate::test_utils::*;
use std::assert_matches::assert_matches;

macro_rules! par_check {
    (
        $(#[$attr:meta])*
        $name:ident, $inp:literal, $( $val:expr ),*
    ) => {
        $(#[$attr])*
        #[test]
        fn $name() {
            match paragraph(&mut StringData::new($inp, ParserSettings::Html), false) {
                Block::Paragraph(ct, _)
                    => assert_eq!(vec![ $( $val ),* ] as Vec<Inline>, ct),

                x => assert_matches!(x, Block::Paragraph(..)),
            }
        }
    };

    (
        $(#[$attr:meta])*
        $name:ident, $opts:expr, $inp:literal, $( $val:expr ),*
    ) => {
        $(#[$attr:meta])*
        par_check!($name, $opts, ( $inp ), $( $val ),* );
    };

    (
        $(#[$attr:meta])*
        $name:ident, $opts:expr, ( $( $inp:literal ),+ $(,)? ), $( $val:expr ),*
    ) => {
        $(#[$attr])*
        #[test]
        fn $name() {
            let par = paragraph(
                &mut StringData::new(concat!($( $inp, "\n" ),+), $opts),
                false
            );

            match par {
                Block::Paragraph(ct, _)
                    => assert_eq!(vec![ $( $val ),* ] as Vec<Inline>, ct),

                x => assert_matches!(x, Block::Paragraph(..)),
            }
        }
    };
}

macro_rules! epar_check {
    (
        $(#[$attr:meta])*
        $name:ident, $inp:literal, $( $val:expr ),*
    ) => {
        $(#[$attr])*
        epar_check!($name, ( $inp ), $( $val ),*);
    };

    (
        $(#[$attr:meta])*
        $name:ident, ( $( $inp:literal ),+ $(,)? ), $( $val:expr ),*
    ) => {
        $(#[$attr])*
        par_check!($name, ParserSettings::Embedded, ( $( $inp ),+ ), $( $val ),*);
    };
}

macro_rules! body_check {
    (
        $(#[$attr:meta])*
        $name:ident, $inp:literal, $( $val:expr ),*
    ) => {
        $(#[$attr])*
        #[test]
        fn $name() {
            assert_eq!(
                [ $( $val ),* ].as_slice(),
                body(&mut StringData::new($inp, ParserSettings::Html))
            );
        }
    };

    (
        $(#[$attr:meta])*
        $name:ident, $opts:expr, ( $inp_first:literal $( , $inp:literal )* $(,)? ), $( $val:expr ),*
    ) => {
        $(#[$attr])*
        #[test]
        fn $name() {
            assert_eq!(
                [ $( $val ),* ].as_slice(),
                body(&mut StringData::new(
                    concat!($inp_first $( ,"\n", $inp )* ), $opts
                ))
            );
        }
    };
}

macro_rules! ebody_check {
    (
        $(#[$attr:meta])*
        $name:ident, $inp:literal, $( $val:expr ),*
    ) => {
        $(#[$attr])*
        ebody_check!($name, ( $inp ), $( $val ),*);
    };

    (
        $(#[$attr:meta])*
        $name:ident, ( $( $inp:literal ),+ $(,)? ), $( $val:expr ),*
    ) => {
        $(#[$attr])*
        body_check!($name, ParserSettings::Embedded, ( $( $inp ),+ ), $( $val ),*);
    };
}

mod embedded_block {
    use super::*;

    ebody_check!(
        single_line,
        r#"@let site_name = "My Site";"#,
        emb_block_block!(r#"let site_name = "My Site";"#)
    );

    ebody_check!(
        single_line_newline,
        r#"@let site_name = "My Site";\n"#,
        emb_block_block!(r#"let site_name = "My Site";\n"#)
    );

    ebody_check!(
        multiple_lines,
        "@{\n  use html_tools::*;\n  init_toc();\n  let count = 42;\n@}",
        emb_block_block!(
            "  use html_tools::*;\n  init_toc();\n  let count = 42;\n"
        )
    );

    ebody_check!(
        code_with_empty_lines,
        "@{\n  use html_tools::*;\n\n  init_toc();\n\n  let count = 42;\n@}\n",
        emb_block_block!(
            "  use html_tools::*;\n\n  init_toc();\n\n  let count = 42;\n"
        )
    );

    ebody_check!(simple_stmt, "@content()", emb_expr_block!("content()"));
    // TODO: with args

    ebody_check!(
        code_if,
        (
            "@if let Some(desc) = description {",
            r#"  <meta name="description" content="@attr_value(desc)" />"#,
            "}"
        ),
        emb_block_block!("if let Some(desc) = description {\n"),
        html_block!(r#"  <meta name="description" content=""#),
        emb_expr_block!("attr_value(desc)"),
        html_block!("\" />\n"),
        emb_block_block!("}\n")
    );

    /* TODO:
@if !tags.is_empty() {
  <nav class=tags>Schlagworte:
  @{
    for (i, t) in tags.into_iter().enumerate() {
        if i > 0 {
            ", ".to_html(out)?;
        }
        ctx.simple_link(out, &t.1, "", &t.0)?;
    }
  @}
  </nav>
}
    */
    ebody_check!(
        todo,
        "<html>", html_block!("<html>")
    );
}

mod embedded_inline {
    use super::*;

    epar_check!(simple_expr, "@var", emb_expr!("var"));
    epar_check!(expr_lowdash, "@_var", emb_expr!("_var"));
    epar_check!(expr_amp, "@&var", emb_expr!("&var"));
    epar_check!(expr_paren, "@(var)", emb_expr!("var"));
    epar_check!(expr_expr, "@(7 + 4)", emb_expr!("7 + 4"));
    epar_check!(expr_with_stmt, "@(\"abc\".len())", emb_expr!("\"abc\".len()"));

    epar_check!(simple_stmt, "@fun()", emb_expr!("fun()"));
    epar_check!(stmt_lowdash, "@_fun()", emb_expr!("_fun()"));
    epar_check!(stmt_amp, "@&fun()", emb_expr!("&fun()"));
    epar_check!(method_stmt, "@var.fun()", emb_expr!("var.fun()"));
    epar_check!(module_stmt, "@mod::fun()", emb_expr!("mod::fun()"));
    epar_check!(macro_stmt, "@tag!()", emb_expr!("tag!()"));
    epar_check!(
        stmt_with_args,
        "@fun(x, 42, \"text\")",
        emb_expr!("fun(x, 42, \"text\")")
    );
    epar_check!(
        stmt_with_arg_paren,
        "@fun(x, (40 + 2), \"t(ex)t\")",
        emb_expr!("fun(x, (40 + 2), \"t(ex)t\")")
    );
    epar_check!(
        stmt_map_or,
        "Hello @name.map_or(\"stranger\", |x| x.as_str())",
        plain!("Hello "),
        emb_expr!("name.map_or(\"stranger\", |x| x.as_str())")
    );
    epar_check!(
        stmt_map_unwrap,
        "Hello @(name.map(|x| x.as_str()).unwrap_or(\"stranger\"))",
        plain!("Hello "),
        emb_expr!("name.map(|x| x.as_str()).unwrap_or(\"stranger\")")
    );

    epar_check!(
        line_comment,
        "text @// comment\n  next line",
        plain!("text "), emb_block!("// comment\n"), plain!("next line")
    );
    epar_check!(
        block_comment,
        "before @/* comment */ after",
        plain!("before "), emb_block!("/* comment */"), plain!(" after")
    );

    epar_check!(simple_code, "@{let x = 7;}", emb_block!("let x = 7;"));

    epar_check!(
        code_if,
        "@if !msg.is_empty() { @msg }",
        emb_block!("if !msg.is_empty() {"),
        plain!(" "), emb_expr!("msg"), plain!(" "),
        emb_block!("}")
    );

    epar_check!(
        code_if_emph,
        "@if !msg.is_empty() { *@msg* }",
        emb_block!("if !msg.is_empty() {"),
        plain!(" "), emph!(emb_expr!("msg")), plain!(" "),
        emb_block!("}")
    );

    epar_check!(
        code_if_else,
        "Hello @if name.is_empty() {stranger} else {@name.as_str()},",
        plain!("Hello "),
        emb_block!("if name.is_empty() {"),
        plain!("stranger"),
        emb_block!("} else {"),
        emb_expr!("name.as_str()"),
        emb_block!("}")
    );

    epar_check!(
        code_for,
        "@for e in list {@e}",
        emb_block!("for e in list {"), emb_expr!("e"), emb_block!("}")
    );

    epar_check!(
        code_for_if,
        "@for e in list {@if first {@{first = false;}} else {, }@e}",
        emb_block!("for e in list {"),
        emb_block!("if first {"), emb_block!("first = false;"),
        emb_block!("} else {"), plain!(", "), emb_block!("}"),
        emb_expr!("e"),
        emb_block!("}")
    );

    epar_check!(
        code_for_if_short,
        "@for e in list {\n    \
         if first { @{first = false;} } else {, }@e\n  \
         }",
        emb_block!("for e in list {\n    if first {"),
        emb_block!("first = false;"),
        emb_block!("} else {"), plain!(", "), emb_block!("}"),
        emb_expr!("e"),
        emb_block!("}")
    );
}

body_check!(
    linkdef_not_inside_paragraph,
    "text [foo] text\n\n\
     x[foo]: /wrong\n\n\
     [foo]: /right",
    paragraph!(plain!("text "), linkref!(plain!("foo")), plain!(" text")),
    paragraph!(plain!("x"), linkref!(plain!("foo")), plain!(": /wrong")),
    linkdef!("foo", "/right")
);

body_check!(
    simple_linkdef_and_text,
    "[foo]: /url\ntest",
    linkdef!("foo", "/url"),
    paragraph!(plain!("test"))
);

body_check!(
    simple_linkdef_and_trailing_space,
    "[foo]: /url1  \n[bar]: /url2  ",
    linkdef!("foo", "/url1"),
    linkdef!("bar", "/url2")
);

body_check!(
    full_linkdef_and_trailing_space,
    "[foo]: /url1  \"title1\"  \n[bar]: /url2  'title2'  \n[baz]: /url3\t(title3)  ",
    linkdef!("foo", "/url1", "title1"),
    linkdef!("bar", "/url2", "title2"),
    linkdef!("baz", "/url3", "title3")
);

body_check!(
    /// This is a violation of the spec. But parsing the shortcut inside a
    /// link is difficult, because the link definition comes afterwards and
    /// can't be used by the decision, if the link reference is valid.
    shortref_in_link_is_plain,
    "[[foo]](/img)\n\n\
     [[foo][]](/img)\n\n\
     [foo]: /url",
    paragraph!(link!("/img", "", plain!("[foo]"))),
    paragraph!(plain!("["), linkref!(plain!("foo")), plain!("](/img)")),
    linkdef!("foo", "/url")
);

