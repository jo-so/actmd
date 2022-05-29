#![feature(assert_matches)]
#![feature(decl_macro)]

mod common;
use common::*;

mod image {
    use super::*;

    par_check!(
        url_with_space,
        r#"![Text](<local file.txt>)"#,
        image!("local file.txt", "", plain!(<2, 6> "Text"))
    );

    par_check!(
        empty_text,
        r#"![](/url)"#,
        image!("/url", "")
    );
}

mod imageref {
    use super::*;

    par_check!(
        empty_text,
        r#"![][ref]"#,
        imageref!("ref", )
    );

    par_check!(
        no_empty_text_and_label,
        r#"a ![][] b"#,
        plain!(<0, 9> "a ![][] b")
    );

    par_check!(
        short_on_empty_label,
        r#"a ![text][] b"#,
        plain!(<0, 2> "a "),
        imageref!("", plain!(<4, 8> "text")),
        plain!(<11, 13> " b")
    );

    par_check!(
        /// Derived from [Spec 542](https://spec.commonmark.org/0.29/#example-542)
        label_with_bracket,
        "![foo][ref[]",
        imageref!(plain!(<2, 5> "foo")), plain!(<6, 12> "[ref[]")

    );

    body_check!(
        /// Derrived from https://spec.commonmark.org/0.29/#example-586
        escaped_brackets,
        "![\\[foo\\]]\n\n\
         [\\[foo\\]]: /url \"title\"",
        paragraph!(<0, 11> imageref!(plain!(<2, 9> "[foo]"))),
        linkdef!("[foo]", "/url", "title")
    );
}

mod link {
    use super::*;

    par_check!(
        url_with_space,
        r#"[Text](<local file.txt>)"#,
        link!("local file.txt", "", plain!(<1, 5> "Text"))
    );

    par_check!(
        empty_text,
        r#"[](/url)"#,
        link!("/url", "")
    );

    par_check!(
        /// Derived from https://spec.commonmark.org/0.29/#example-170
        no_space_between_url_title,
        "[foo](<bar>(baz))",
        linkref!(plain!(<1, 4> "foo")),
        plain!(<5, 6> "("), html!(<6, 11> "<bar>"), plain!(<11, 17> "(baz))")
    );

    body_check!(
        unbalanced_braces_in_url,
        "[abc](/tes(t )\n\n[abc](/tes(t\n)",
        paragraph!(<0, 15> linkref!(plain!(<1, 4> "abc")), plain!(<5, 14> "(/tes(t )")),
        paragraph!(
            <16, 30>
            linkref!(plain!(<17, 20> "abc")),
            plain!(<21, 28> "(/tes(t"),
            SoftBreak,
            plain!(<29, 30> ")")
        )
    );
}

mod linkref {
    use super::*;

    par_check!(
        empty_text,
        r#"[][ref]"#,
        linkref!("ref", )
    );

    par_check!(
        no_empty_text_and_label,
        r#"a [][] b"#,
        plain!(<0, 8> "a [][] b")
    );

    body_check!(
        multiple_simple_linkdefs,
        "[foo]: /url1\n\
         [bar]: /url2",
        linkdef!("foo", "/url1"),
        linkdef!("bar", "/url2")
    );

    par_check!(
        short_on_empty_label,
        r#"a [text][] b"#,
        plain!(<0, 2> "a "),
        linkref!("", plain!(<3, 7> "text")),
        plain!(<10, 12> " b")
    );

    par_check!(
        /// This is a violation of the spec, because it parses this to `plain!("[text]"),
        /// link!("/url", "", plain!("reflabel"))` due to the absence of a link reference
        /// definition of *reflabel*.
        linkref_non_link,
        "[text][reflabel](/url)",
        linkref!("reflabel", plain!(<1, 5> "text")), plain!(<16, 22> "(/url)")
    );
}
