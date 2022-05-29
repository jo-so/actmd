#![feature(assert_matches)]
#![feature(decl_macro)]

mod common;
use common::*;

body_check!(
    empty,
    "> ",
    quote!(<0, 2>)
);

body_check!(
    simple,
    "> quoted line\n\nnext paragraph",
    quote!(<0, 14> paragraph!(<2, 14> plain!(<2, 13> "quoted line"))),
    paragraph!(<15, 29> plain!(<15, 29> "next paragraph"))
);

body_check!(
    continued,
    "> line 1\n> line 2",
    quote!(<0, 17> paragraph!(
        <2, 17>
        plain!(<2, 8> "line 1"),
        SoftBreak,
        plain!(<11, 17> "line 2")
    ))
);

body_check!(
    continued_leading_space,
    ">line 1\n  >line 2",
    quote!(<0, 17> paragraph!(
        <1, 17>
        plain!(<1, 7> "line 1"),
        SoftBreak,
        plain!(<11, 17> "line 2")
    ))
);

body_check!(
    continued_hanging,
    "> line 1\nline 2",
    quote!(<0, 15> paragraph!(
        <2, 15>
        plain!(<2, 8> "line 1"),
        SoftBreak,
        plain!(<9, 15> "line 2")
    ))
);

body_check!(
    indented_second_level,
    "> line 1\n  >> line 2\ninside\n\nnext paragraph",
    quote!(
        <0, 28>
        paragraph!(<2, 12> plain!(<2, 8> "line 1")),

        quote!(
            <12, 28>
            paragraph!(
                <14, 28>
                plain!(<14, 20> "line 2"),
                SoftBreak,
                plain!(<21, 27> "inside")
            )
        )
    ),

    paragraph!(<29, 43> plain!(<29, 43> "next paragraph"))
);

body_check!(
    inner_code_block,
    "> ```text\n>   bla\n>  ```",
    quote!(<0, 24> code_block!(<2, 24> "text", "  bla\n"))
);

body_check!(
    nested_heading,
    "> # headline",
    quote!(<0, 12> heading!(<2, 12> 1, plain!(<4, 12> "headline")))
);

body_check!(
    heading_stops_quote,
    "> quote\n# headline",
    quote!(<0, 8> paragraph!(<2, 8> plain!(<2, 7> "quote"))),
    heading!(<8, 18> 1, plain!(<10, 18> "headline"))
);
