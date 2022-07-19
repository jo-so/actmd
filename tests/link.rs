#![feature(assert_matches)]
#![feature(decl_macro)]

mod common;
use common::*;

mod footnote_inline {
    use super::*;

    par_check!(
        empty, "[][^]",
        linkref!(<0, 0> "^", )
    );

    par_check!(
        simple, "[Text][^]",
        linkref!(<0, 0> "^", plain!(<1, 5> "Text"))
    );

    par_check!(
        with_image, "[Text and ![an image](img.jpg)][^]",
        linkref!(
            <0, 0> "^",
            plain!(<1, 5> "Text and "),
            image!(<0, 0> "img.jpg", "", plain!(<0, 0> "an image"))
        )
    );

    par_check!(
        with_link, "[Text and [a link](/url)][^]",
        linkref!(
            <0, 0> "^",
            plain!(<1, 5> "Text and "),
            link!(<0, 0> "/url", "", plain!(<0, 0> "a link"))
        )
    );

    par_check!(
        with_footnote, "[A footnote[in a footnote][^]][^]",
        linkref!(
            <0, 0> "^",
            plain!(<1, 5> "A footnote"),
            linkref!(<0, 0> "^", plain!(<0, 0> "in a footnote"))
        )
    );
}

mod footnote_ref {
    use super::*;

    par_check!(
        without_def, "[^fn]",
        linkref!(<0, 0> "^fn",)
    );

    body_check!(
        empty,
        "Text[^fn]\n\n\
         [^fn]:",

        paragraph!(<0, 0> plain!(<0, 4> "Text"), linkref!(<0, 0> "^fn",)),
        linkdef!(<0, 0> "^fn", "")
    );

    body_check!(
        simple,
        "Text[^fn]\n\n\
         [^fn]: Footnote content",

        paragraph!(<0, 0> plain!(<0, 4> "Text"), linkref!(<0, 0> "^fn",)),
        linkdef!(
            <0, 0> "^fn",
            "" // paragraph!(<0, 0> plain!(<0, 0> "Footnote content"))
        )
    );
}

mod image {
    use super::*;

    par_check!(
        url_with_space,
        r#"![Text](<local file.txt>)"#,
        image!(<0, 0> "local file.txt", "", plain!(<2, 6> "Text"))
    );

    par_check!(
        empty_text,
        r#"![](/url)"#,
        image!(<0, 0> "/url", "")
    );
}

mod imageref {
    use super::*;

    par_check!(
        empty_text,
        r#"![][ref]"#,
        imageref!(<0, 0> "ref", )
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
        imageref!(<0, 0> "", plain!(<4, 8> "text")),
        plain!(<11, 13> " b")
    );

    par_check!(
        /// Derived from [Spec 542](https://spec.commonmark.org/0.29/#example-542)
        label_with_bracket,
        "![foo][ref[]",
        imageref!(<0, 0> plain!(<2, 5> "foo")), plain!(<6, 12> "[ref[]")

    );

    body_check!(
        /// Derrived from https://spec.commonmark.org/0.29/#example-586
        escaped_brackets,
        "![\\[foo\\]]\n\n\
         [\\[foo\\]]: /url \"title\"",
        paragraph!(<0, 11> imageref!(<0, 0> plain!(<2, 9> "[foo]"))),
        linkdef!(<0, 0> "[foo]", "/url", "title")
    );
}

mod link {
    use super::*;

    par_check!(
        url_with_space,
        r#"[Text](<local file.txt>)"#,
        link!(<0, 0> "local file.txt", "", plain!(<1, 5> "Text"))
    );

    par_check!(
        empty_text,
        r#"[](/url)"#,
        link!(<0, 0> "/url", "")
    );

    par_check!(
        /// Derived from https://spec.commonmark.org/0.29/#example-170
        no_space_between_url_title,
        "[foo](<bar>(baz))",
        linkref!(<0, 0> plain!(<1, 4> "foo")),
        plain!(<5, 6> "("), html!(<6, 11> "<bar>"), plain!(<11, 17> "(baz))")
    );

    body_check!(
        unbalanced_braces_in_url,
        "[abc](/tes(t )\n\n[abc](/tes(t\n)",
        paragraph!(
            <0, 15>
            linkref!(<0, 0> plain!(<1, 4> "abc")),
            plain!(<5, 14> "(/tes(t )")
        ),
        paragraph!(
            <16, 30>
            linkref!(<0, 0> plain!(<17, 20> "abc")),
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
        linkref!(<0, 0> "ref", )
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
        linkdef!(<0, 0> "foo", "/url1"),
        linkdef!(<0, 0> "bar", "/url2")
    );

    par_check!(
        short_on_empty_label,
        r#"a [text][] b"#,
        plain!(<0, 2> "a "),
        linkref!(<0, 0> "", plain!(<3, 7> "text")),
        plain!(<10, 12> " b")
    );

    par_check!(
        /// This is a violation of the spec, because it parses this to `plain!("[text]"),
        /// link!("/url", "", plain!("reflabel"))` due to the absence of a link reference
        /// definition of *reflabel*.
        linkref_non_link,
        "[text][reflabel](/url)",
        linkref!(<0, 0> "reflabel", plain!(<1, 5> "text")), plain!(<16, 22> "(/url)")
    );
}
