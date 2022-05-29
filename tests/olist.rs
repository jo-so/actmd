#![feature(assert_matches)]
#![feature(decl_macro)]

mod common;
use common::*;

body_check!(
    parenthesis_delimiter, "1) foo\n2) foo",
    ordered_list!(
        vec![paragraph!(<3, 7> plain!(<3, 6> "foo"))],
        vec![paragraph!(<10, 13> plain!(<10, 13> "foo"))]
    )
);

body_check!(
    mixed_delimiters, "1. text\n2) text\n3. text",
    ordered_list!(
        vec![paragraph!(<3, 8> plain!(<3, 7> "text"))],
        vec![paragraph!(<11, 16> plain!(<11, 15> "text"))],
        vec![paragraph!(<19, 23> plain!(<19, 23> "text"))]
    )
);

body_check!(
    big_numbers,
    "1. text\n22. text\n333. text\n4444. text\n1234567890. text",
    ordered_list!(
        vec![paragraph!(<3, 8> plain!(<3, 7> "text"))],
        vec![paragraph!(<12, 17> plain!(<12, 16> "text"))],
        vec![paragraph!(<22, 27> plain!(<22, 26> "text"))],
        vec![paragraph!(<33, 38> plain!(<33, 37> "text"))],
        vec![paragraph!(<50, 54> plain!(<50, 54> "text"))]
    )
);

body_check!(
    #[ignore]
    number_at_line_start,
    "1. paragraph with\n  3 lines\n  end\n2. next list item",
    ordered_list!(
        "1",
        vec![paragraph!(
            <3, 31>
            plain!(<3, 17> "paragraph with"), SoftBreak,
            plain!(<18, 26> "3 lines"), SoftBreak,
            plain!(<27, 30> "end"), SoftBreak
        )],

        vec![paragraph!(<3, 31> plain!(<3, 17> "next list item"))]
    )
);
