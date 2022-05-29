#![feature(assert_matches)]
#![feature(decl_macro)]

mod common;
use common::*;

cm_body_check!(
    /// Block structure takes precedence over inline structure
    /// <https://spec.commonmark.org/0.29/#example-61>
    #[cfg_attr(feature = "location", ignore)]
    code_block_breaks_inline_code,
    "`*Foo*\n```\nnot inline code`\n",
    paragraph!(<0, 7> plain!(<0, 1> "`"), emph!(plain!(<2, 5> "Foo"))),
    code_block!(<7, 28> "", "not inline code`\n")
);

cm_body_check!(
    heading_breaks_inline_html_tag,
    "<tag\n# headline\ndata='foo' >",
    paragraph!(<0, 5> plain!(<0, 4> "<tag")),
    heading!(<5, 16> 1, plain!(<7, 15> "headline")),
    paragraph!(<16, 28> plain!(<16, 28> "data='foo' >"))
);

cm_body_check!(
    heading_breaks_inline_html_proc_inst,
    "text <?php\n# headline\necho 42; ?>",
    paragraph!(<0, 11> plain!(<0, 10> "text <?php")),
    heading!(<11, 22> 1, plain!(<13, 21> "headline")),
    paragraph!(<22, 33> plain!(<22, 33> "echo 42; ?>"))
);
