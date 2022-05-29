#![feature(assert_matches)]
#![feature(decl_macro)]
#![cfg(not(feature = "location"))]

mod common;
use common::*;

par_check!(
    unquoted_not_code,
    "<img src=@url title=@(url) alt=@{url} />",
    html!("<img src=@url title=@(url) alt=@{url} />")
);

par_check!(
    value_with_code_simple_expr,
    "<img src='@url' title=\"@title\" />",
    html!("<img src='"), emb_expr!("url"),
    html!("' title=\""), emb_expr!("title"), html!("\" />")
);

par_check!(
    value_with_code_expr,
    r#"<img src="@(&"img.png"[..3])" title='@('0'.to_digit(10).unwrap() - 6)' />"#,
    html!("<img src=\""), emb_expr!("&\"img.png\"[..3]"),
    html!("\" title='"), emb_expr!("'0'.to_digit(10).unwrap() - 6"),
    html!("' />")
);

par_check!(
    single_value_with_code_stmt,
    r#"<img src="@url("img.png")" />"#,
    html!("<img src=\""), emb_expr!("url(\"img.png\")"), html!("\" />")
);

// TODO: { }
// TODO: newline
// TODO: if for
