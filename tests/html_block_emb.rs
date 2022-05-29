#![feature(assert_matches)]
#![feature(decl_macro)]

mod common;
use common::*;

body_check!(
    plain_at_no_emb,
    "<div> @ @test </div>",
    html_block!(<0, 20> "<div> @ @test </div>\n")
);

body_check!(
    not_in_emails,
    "<meta name=author content=\"Jon Doe <jon@example.org>\" />",
    html_block!(<0, 56> "<meta name=author content=\"Jon Doe <jon@example.org>\" />\n")
);

body_check!(
    escaped_expr,
    "<meta content='@@(user)'>",
    html_block!(<0, 25> "<meta content='@(user)'>\n")
);

body_check!(
    escaped_block,
    "<meta content='@@{user}'>",
    html_block!(<0, 25> "<meta content='@{user}'>\n")
);

body_check!(
    #[cfg_attr(feature = "location", ignore)]
    embedded_expr,
    "<title>@(title) @ @(site_name)</title>",
    html_block!(<0, 8> "<title>"),
    emb_expr_block!(<9, 14> "title"),
    html_block!(<15, 18> " @ "),
    emb_expr_block!(<20, 29> "site_name"),
    html_block!(<30, 38> "</title>")
);

body_check!(
    #[cfg_attr(feature = "location", ignore)]
    expr_with_parenthesis,
    r#"<meta name="description" content="@(attr_value(desc))" />"#,
    html_block!(<0, 34> r#"<meta name="description" content=""#),
    emb_expr_block!(<36, 52> "attr_value(desc)"),
    html_block!(<53, 58> "\" />")
);

body_check!(
    #[cfg_attr(feature = "location", ignore)]
    expr_with_braces,
    r#"<meta name="description" content="@(if val { 1 } else { 2 })" />"#,
    html_block!(<0, 34> r#"<meta name="description" content=""#),
    emb_expr_block!(<36, 52> "attr_value(desc)"),
    html_block!(<53, 58> "\" />")
);

body_check!(
    #[cfg_attr(feature = "location", ignore)]
    block_with_parenthesis,
    r#"<meta name="description" content="@{attr_value(desc)}" />"#,
    html_block!(<0, 34> r#"<meta name="description" content=""#),
    emb_expr_block!(<36, 52> "attr_value(desc)"),
    html_block!(<53, 58> "\" />")
);

body_check!(
    #[cfg_attr(feature = "location", ignore)]
    block_with_braces,
    r#"<meta name="description" content="@{if val { 1 } else { 2 }}" />"#,
    html_block!(<0, 34> r#"<meta name="description" content=""#),
    emb_expr_block!(<36, 52> "attr_value(desc)"),
    html_block!(<53, 58> "\" />")
);

body_check!(
    #[cfg_attr(feature = "location", ignore)]
    html_after_code,
    "@if let Some(auth) = author {\n  <meta name=author content=\"@attr_value(auth)\" />\n}",
    emb_block_block!(<1, 30> "if let Some(auth) = author {\n"),
    html_block!(<30, 59> "  <meta name=author content=\""),
    emb_expr_block!(<60, 76> "attr_value(auth)"),
    html_block!(<76, 81> "\" />\n"),
    emb_block_block!(<81, 82> "}")
);
