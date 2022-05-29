#![feature(assert_matches)]
#![feature(decl_macro)]

mod common;
use common::*;

macro_rules! test {
    ($name:ident, $val:literal, $foo:ident, $len:literal) => {
        cm_par_check!($name, $val, $foo!(<0, $len> $val));
    };
}

mod cdata {
    use super::*;

    test!(minimal, "<![CDATA[]]>", html, 12);
    test!(with_data, "<![CDATA[ data ]]>", html, 18);
    test!(with_1_bracket, "<![CDATA[ data]more ]]>", html, 23);
    test!(with_2_brackets, "<![CDATA[ data]] more ]]>", html, 25);
    test!(with_bracket_angle, "<![CDATA[ data]> more ]]>", html, 25);

    cm_par_check!(
        invalid_start_1,
        "<![ data ]]>",
        plain!(<0, 1> "<"),
        imageref!(plain!(<3, 9> " data ")),
        plain!(<10, 12> "]>")
    );

    cm_par_check!(
        invalid_start_2,
        "<![CDATA data ]]>",
        plain!(<0, 1> "<"),
        imageref!(plain!(<3, 14> "CDATA data ")),
        plain!(<15, 17> "]>")
    );


    cm_par_check!(
        invalid_start_3,
        "<![cdata[ data ]]>",
        plain!(<0, 1> "<"),
        imageref!(plain!(<3, 8> "cdata"), linkref!(plain!(<9, 15> " data "))),
        plain!(<17, 18> ">")
    );
}

mod comment {
    use super::*;

    test!(minimal, "<!---->", html, 7);
    test!(with_text, "<!--text-->", html, 11);
    test!(with_dash, "<!-- - -->", html, 10);

    test!(invalid_start_1, "<!- comment -->", plain, 15);
    test!(invalid_start_2, "<!--", plain, 4);
    test!(invalid_start_3, "<!-->", plain, 5);
    test!(invalid_start_4, "<!--->", plain, 6);

    test!(invalid_end_1, "<!-- comment ->", plain, 15);
    test!(invalid_end_2, "<!-- comment --", plain, 15);

    cm_body_check!(
        no_par_break_in_comment,
        "x <!--\n\n-->",
        paragraph!(<0, 7> plain!(<0, 6> "x <!--")),
        paragraph!(<8, 11> plain!(<8, 11> "-->"))
    );
}

mod declaration {
    use super::*;

    test!(minimal, "<!A b>", html, 6);
    test!(doctype_html5, "<!DOCTYPE html>", html, 15);
    test!(with_newline, "<!DOCTYPE html\nPUBLIC>", html, 22);
    test!(
        doctype_html4,
        r#"<!DOCTYPE HTML PUBLIC "-//W3C//DTD HTML 4.01 Transitional//EN" "http://www.w3.org/TR/html4/loose.dtd">"#,
        html, 102
    );
    test!(
        doctype_xhtml,
        r#"<!DOCTYPE html PUBLIC "-//W3C//DTD XHTML 1.1//EN" "http://www.w3.org/TR/xhtml11/DTD/xhtml11.dtd">"#,
        html, 97
    );

    test!(invalid_1, "<!A>", plain, 4);
    test!(invalid_2, "<!1 b>", plain, 6);
    test!(invalid_3, "<!_ b>", plain, 6);
}

mod processing_instruction {
    use super::*;

    test!(bad, "<?>", plain, 3);
    test!(unclosed, "<??", plain, 3);
    test!(short, "<??>", html, 4);
    test!(valid, "<? text ?>", html, 10);

    cm_body_check!(
        with_newline, "Start par <? text\ntext ?>",
        paragraph!(<0, 25>
            plain!(<0, 10> "Start par "),
            html!(<10, 25> "<? text\ntext ?>")
        )
    );

    cm_body_check!(
        with_2_newlines, "Start par <? text\n\ntext ?>",
        paragraph!(<0, 18> plain!(<0, 17> "Start par <? text")),
        paragraph!(<19, 26> plain!(<19, 26> "text ?>"))
    );
}

mod tags {
    use super::*;

    test!(minimal_single, "<b/>", html, 4);
    test!(minimal_opening, "<a>", html, 3);
    test!(minimal_closing, "</a>", html, 4);

    test!(name_digits, "<a123>", html, 6);
    test!(name_dash, "<a1-23->", html, 8);
    test!(name_uppercase, "<BR/>", html, 5);

    test!(simple_attribute, "<a a>", html, 5);
    test!(simple_attribute_tab, "<a\ta>", html, 5);
    test!(simple_attribute_multiple_spaces, "<a \t a>", html, 7);
    test!(simple_attribute_closing, "<a a/>", html, 6);
    test!(simple_attribute_space_closing, "<a a />", html, 7);
    test!(simple_attribute_tab_closing, "<a a\t/>", html, 7);
    test!(simple_attribute_multiple_spaces_closing, "<a a \t />", html, 9);

    test!(attribute_value, "<a a=a>", html, 7);
    test!(attribute_value_closing, "<a a=a/>", html, 8);
    test!(attribute_value_space_closing, "<a a=a />", html, 9);
    test!(attribute_value_around_before, "<a a = a>", html, 9);

    test!(attribute_value_single_empty, "<a a=''>", html, 8);
    test!(attribute_value_double_empty, "<a a=\"\">", html, 8);
    test!(attribute_value_specials_in_single, "<a a='>\" &amp;'>", html, 16);
    test!(attribute_value_specials_in_double, "<a a=\">' &amp;\">", html, 16);

    test!(attribute_colon_start, "<a :attr>", html, 9);
    test!(attribute_colon_start_value, "<a :attr=val>", html, 13);
    test!(attribute_lowdash_start, "<a _attr>", html, 9);
    test!(attribute_lowdash_start_value, "<a _attr=val>", html, 13);
    test!(attribute_uppercase_start, "<a ATTR>", html, 8);
    test!(attribute_uppercase_start_value, "<a ATTR=val>", html, 12);

    test!(attribute_mixed, "<a :_A.t-1-t._R0_:>", html, 19);
    test!(attribute_mixed_value, "<a :_A.t-1-t._R0_:=val>", html, 23);

    test!(only_left_angle, "<", plain, 1);
    test!(empty_angles, "<>", plain, 2);
    test!(empty_closing_angles, "</>", plain, 3);
    test!(empty_closing_angles_space, "</ >", plain, 4);

    test!(space_before_name, "< a>", plain, 4);
    test!(space_before_closing_name, "< /a>", plain, 5);
    test!(space_after_closing, "<a/ >", plain, 5);

    test!(missing_right_angle, "<a", plain, 2);
    test!(missing_right_closing_angle, "<a/", plain, 3);

    test!(invalid_name_1, "<1a>", plain, 4);
    test!(invalid_name_2, "<_a>", plain, 4);
    test!(invalid_name_3, "<:a>", plain, 4);

    test!(missing_value, "<a a=>", plain, 6);
    test!(missing_value_space, "<a a= \t >", plain, 9);
    test!(missing_right_delimiter_single, "<a a='val>", plain, 10);
    test!(missing_right_delimiter_double, "<a a=\"val>", plain, 10);

    test!(incomplete_attr, "<a a", plain, 4);
    test!(incomplete_value, "<a a=val", plain, 8);
    test!(incomplete_value_single, "<a a='val", plain, 9);
    test!(incomplete_value_double, "<a a=\"val", plain, 9);

    test!(invalid_undelimited_value_1, "<a a=va\"lue>", plain, 12);
    test!(invalid_undelimited_value_2, "<a a=va'lue>", plain, 12);
    test!(invalid_undelimited_value_3, "<a a=va`lue>", plain, 12);
    test!(invalid_undelimited_value_4, "<a a=va=lue>", plain, 12);

    cm_par_check!(
        invalid_undelimited_value_5,
        "<a a=va<lue>",
        plain!(<0, 7> "<a a=va"), html!(<7, 12> "<lue>")
    );
}

cm_par_check!(
    with_text,
    "<a href=xxx> Text </a>",
    html!(<0, 12> "<a href=xxx>"), plain!(<12, 18> " Text "), html!(<18, 22> "</a>")
);
