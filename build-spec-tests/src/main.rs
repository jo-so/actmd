use anyhow::Context;

use html5ever::{
    parse_fragment,
    QualName,
    local_name,
    namespace_url,
    ns,
    LocalName,
    rcdom::RcDom,
    serialize::{
        AttrRef,
        Serialize,
        Serializer,
        TraversalScope,
    },
};

use percent_encoding::percent_decode_str as percent_decode;

use std::{
    collections::{
        HashMap,
        HashSet,
    },
    env,
    fs::File,
    io::{
        self,
        BufRead,
        BufReader,
        Lines
    },
    iter::Enumerate,
    num::IntErrorKind::Empty,
    ops::{
        RangeFrom,
        RangeInclusive,
    },
};

use tendril::TendrilSink;

struct TestOutput {
    kind: String,
    src: String,
    text: String,
}

#[derive(Default)]
struct TestSerializer {
    add_comma: bool,
    inside_code: bool,
    inside_div: bool,
    inside_li: bool,
    inside_pre: bool,
    inside_style: bool,
    space_to_tab: bool,
    eol: &'static str,
}

impl TestSerializer {
    fn handle_comma(&mut self, new_val: bool) {
        if self.add_comma {
            print!(", ");
        }
        self.add_comma = new_val;
    }
}

impl Serializer for TestSerializer {
    /// Serialize the start of an element, for example `<div class="test">`.
    fn start_elem<'a, AttrIter>(&mut self, name: QualName, attrs: AttrIter) -> io::Result<()>
    where
        AttrIter: Iterator<Item = AttrRef<'a>>
    {
        // println!("<{} {}>", &name.local, self.add_comma);

        let attrs = attrs.collect::<Vec<_>>();
        let attr = |name: &str| {
            attrs.iter().find(|(n, _)| &n.local == name)
                .map(|(_, val)| val)
                .unwrap_or(&"")
        };

        self.handle_comma(false);

        match name.local {
            local_name!("em") => print!("emph!("),
            local_name!("strong") => print!("strong!("),
            local_name!("h1") => { print!("heading!(1"); self.add_comma = true; self.inside_div = true; }
            local_name!("h2") => { print!("heading!(2"); self.add_comma = true; self.inside_div = true; }
            local_name!("h3") => { print!("heading!(3"); self.add_comma = true; self.inside_div = true; }
            local_name!("h4") => { print!("heading!(4"); self.add_comma = true; self.inside_div = true; }
            local_name!("h5") => { print!("heading!(5"); self.add_comma = true; self.inside_div = true; }
            local_name!("h6") => { print!("heading!(6"); self.add_comma = true; self.inside_div = true; }
            local_name!("blockquote") => print!("quote!("),

            local_name!("p") => {
                self.inside_div = true;
                print!("paragraph!(");
            }

            local_name!("hr") => {
                print!("Break");
                self.add_comma = true;
            }

            local_name!("br") => {
                print!("HardBreak");
                self.add_comma = true;
            }

            local_name!("a") => {
                let mut url = percent_decode(attr("href")).decode_utf8().unwrap();
                let mut alt = attr("title").to_string();
                if self.space_to_tab {
                    use std::borrow::Cow;
                    use std::ops::Deref;

                    url = Cow::from(url.deref().replace(' ', "\t"));
                    alt = alt.replace(' ', "\t");
                }
                print!(r#"link!("{}", "{}""#, url.escape_default(), alt.escape_default());
                self.add_comma = true;
            }

            local_name!("img") => {
                print!(r#"image!("{}", "{}""#,
                       attr("src").escape_default(),
                       attr("title").escape_default(),
                );
                let alt = attr("alt");
                if !alt.is_empty() {
                    let mut alt = alt.to_string();
                    if self.space_to_tab {
                        alt = alt.replace(' ', "\t");
                    }
                    print!(r#", plain!("{}")"#, alt.escape_default());
                }
                print!(")");
                self.add_comma = true;
            }

            local_name!("ol") => {
                print!("ordered_list!(");
                let start = attr("start");
                if !start.is_empty() {
                    print!("\"{start}\", ");
                }

            }

            local_name!("ul") => {
                print!("unordered_list!(");
            }

            local_name!("li") => {
                print!("vec![");
                self.inside_li = true;
            }

            local_name!("pre") => {
                self.inside_div = true;
                self.inside_pre = true;
            }

            local_name!("code") => {
                if self.inside_pre {
                    let mut info = attr("class").to_string();
                    if info.starts_with("language-") {
                        info = info[9..].to_string();
                    }

                    print!("code_block!(\"{}\"", info.escape_default());
                    self.add_comma = true;
                } else {
                    print!("code!(");
                }
                self.inside_code = true;
            }

            local_name!("style") => {
                print!("html_block!(\"<style>");
                self.inside_div = true;
                self.inside_style = true;
            }

            x => print!("»{}«(", x),
        }

        Ok(())
    }

    /// Serialize the end of an element, for example `</div>`.
    fn end_elem(&mut self, name: QualName) -> io::Result<()> {
        // println!("</{} {}>", &name.local, self.add_comma);

        match name.local {
            local_name!("p") | local_name!("pre") | local_name!("blockquote") |
            local_name!("h1") | local_name!("h2") | local_name!("h3") |
            local_name!("h4") | local_name!("h5") | local_name!("h6")
            => {
                self.inside_div = false;
            }

            _ => (),
        }

        match name.local {
            local_name!("code") => self.inside_code = false,
            _ => (),
        }

        match name.local {
            local_name!("hr") | local_name!("br") | local_name!("img") => (),

            local_name!("pre") => {
                self.inside_pre = false;
            }

            local_name!("li") => {
                print!("]");
                self.add_comma = true;
                self.inside_li = false;
            }

            local_name!("style") => {
                print!("</style>{}\")", self.eol.escape_debug());
                self.add_comma = true;
                self.inside_div = false;
                self.inside_style = false;
            }

            _ => {
                print!(")");
                self.add_comma = true;
            }
        }

        Ok(())
    }

    /// Serialize a plain text node.
    fn write_text(&mut self, text: &str) -> io::Result<()> {
        // println!("'{}'", text.escape_default());

        if !self.inside_div {
            if text == "\n" {
                // ignore
            } else if self.inside_li {
                let text = text.trim_end().escape_default().to_string();

                let text = if self.space_to_tab {
                    text.replace(' ', "\t").replace("\\n", r#""), SoftBreak, plain!(""#)
                } else {
                    text.replace("\\n", r#""), SoftBreak, plain!(""#)
                };

                print!(r#"paragraph!(plain!("{}"))"#, text);
                self.add_comma = true;
            } else {
                println!("text not insde of a block: <{}>", text.escape_default());
            }

            return Ok(());
        }

        self.handle_comma(true);

        if self.inside_code {
            let mut text = text.replace('\n', self.eol);
            if self.space_to_tab {
                text = text.replace(' ', "\t");
            }

            print!("\"{}\"", text.escape_debug());
            self.add_comma = false;
        } else if self.inside_style {
            let mut text = text.replace('\n', self.eol);
            if self.space_to_tab {
                text = text.replace(' ', "\t");
            }

            print!("{}", text.escape_debug());
        } else if text == "\n" {
            print!("SoftBreak");
        } else {
            let text = text.escape_debug().to_string();
            let mut text = text.as_str();
            let (mut before, mut after) = ("", "");

            if text.starts_with("\\n") {
                before = "SoftBreak, ";
                text = &text[2..];
            }
            if text.ends_with("\\n") {
                after = ", SoftBreak";
                text = &text[..text.len() - 2];
            }

            let text = if self.space_to_tab {
                text.replace(' ', "\t").replace("\\n", r#""), SoftBreak, plain!(""#)
            } else {
                text.replace("\\n", r#""), SoftBreak, plain!(""#)
            };

            print!("{}plain!(\"{}\"){}", before, text, after);
        }

        Ok(())
    }

    /// Serialize a comment node, for example `<!-- comment -->`.
    fn write_comment(&mut self, text: &str) -> io::Result<()> {
        self.handle_comma(true);
        let mut text = text.replace('\n', self.eol);
        if self.space_to_tab {
            text = text.replace(' ', "\t");
        }

        if !(text.starts_with('?') && text.ends_with('?')) {
            text.insert_str(0, "!--");
            text += "--";
        }

        if self.inside_div {
            print!("html!(\"<{}>\")", text.escape_default());
        } else {
            print!("html_block!(\"<{}>{}\")", text.escape_default(), self.eol.escape_debug());
        }
        Ok(())
    }

    /// Serialize a doctype node, for example `<!doctype html>`.
    fn write_doctype(&mut self, name: &str) -> io::Result<()> {
        self.handle_comma(true);
        print!("html_block!(\"<!DOCTYPE {}>\\n\")", name.escape_default());
        Ok(())
    }

    /// Serialize a processing instruction node, for example
    /// `<?xml-stylesheet type="text/xsl" href="style.xsl"?>`.
    fn write_processing_instruction(
        &mut self, _target: &str, _data: &str
    ) -> io::Result<()> {
        Ok(())
    }
}

fn print_test_prelude(
    ignore: bool, test_no: usize, name_suffix: &str, line: usize,
    expected_html: &str, input: &str
) {
    print!(
        r##"
#[test]{ignore}
/// Test case generated from Commonmark Spec line {line}
/// <https://github.com/commonmark/commonmark-spec/blob/master/spec.txt#L{line}>
/// <https://spec.commonmark.org/0.29/#example-{test_no}>
fn t{test_no}{name_suffix}() {{
    init!("{input}");

    assert_eq!(
        // from spec: {expected_html}
        [
            "##,
        ignore = if ignore { "\n#[ignore]" } else { "" },
        expected_html = expected_html.escape_debug(),
        input = input.escape_debug(),
    );
}

fn gen_test(
    ignore: bool, ex_no: usize, name_ext: &str, line_no: usize,
    input: &str, output: &str, divergent_output: Option<&String>,
    serializer: &mut TestSerializer
) -> io::Result<()> {
    print_test_prelude(ignore, ex_no, name_ext, line_no, output, input);

    if let Some(div) = divergent_output {
        print!("{}", div.replace('\n', "\n            "));
    } else {
        let output = output
            .replace('\u{2192}', "\t") // replace → by TAB
            .replace("<br />\n", "<br />");

        let dom = parse_fragment(
            RcDom::default(), // sink
            Default::default(), // opts
            QualName::new(None, ns!(html), LocalName::from("body")), // context_name
            vec![], // context_attrs
        ).one(output);

        // the first child is <html>
        dom.document.children.borrow()[0].serialize(
            serializer,
            TraversalScope::ChildrenOnly(None)
        )?;
    }

    if serializer.space_to_tab {
        println!(
            "
        ].as_slice(),
        space2tab(body(&mut StringData::new(\"{}\", ParserSettings::Html))),
    );\n}}",
            // replace → by TAB
            input.replace('\u{2192}', "\t").escape_default()
        );
    } else {
        println!(
            "
        ].as_slice(),
        body(&mut StringData::new(\"{}\", ParserSettings::Html)),
    );\n}}",
            // replace → by TAB
            input.replace('\u{2192}', "\t").escape_default()
        );
    }

    Ok(())
}

fn gen_html_test(
    ignore: bool, ex_no: usize, line_no: usize, input: &str, output: &str
) {
    println!(
        r#"
#[test]
{}/// Test case generated from Commonmark Spec line {}
/// <https://github.com/commonmark/commonmark-spec/blob/master/spec.txt#L{}>
/// <https://spec.commonmark.org/0.29/#example-{}>
fn t{}_html() {{
    let mut html = Vec::new();
    assert!(
        HtmlRenderer::render(
            &mut html,
            &[&Document::from_str("{}").unwrap()]
        ).is_ok()
    );
    assert_eq!(
        "{}{}",
        String::from_utf8(html).unwrap()
    );
}}"#,
        if ignore { "#[ignore]\n" } else { "" },
        line_no, line_no, ex_no, ex_no,

        // replace → by TAB
        input.replace('\u{2192}', "\t").escape_debug(),
        output.replace('\u{2192}', "\t")
            // revert some percent encodings, they aren't required by the HTML spec
            .replace("%5B", "[")
            .replace("%5C", "\\")
            .replace("%5D", "]")
            // remove empty *alt* attributes from *\<img>*, because HTML spec requires
            // they are not empty, hence no *alt* and an empty are a violation
            .replace(r#" alt="""#, "")
            .escape_debug(),
        if output.is_empty() { "" } else { "\n" },
    );
}

fn gen_exception(ex_no: usize, input: &str, output: &str) -> io::Result<()> {
    println!("
[Example {ex_no}](https://spec.commonmark.org/0.29/#example-{ex_no}):

```````````````````````````````` rust
{input}.");

    let output = output
        .replace('\u{2192}', "\t") // replace → by TAB
        .replace("<br />\n", "<br />");

    let dom = parse_fragment(
        RcDom::default(), // sink
        Default::default(), // opts
        QualName::new(None, ns!(html), LocalName::from("body")), // context_name
        vec![], // context_attrs
    ).one(output);

    // the first child is <html>
    dom.document.children.borrow()[0].serialize(
        &mut TestSerializer::default(),
        TraversalScope::ChildrenOnly(None)
    )?;

    println!("\n````````````````````````````````");

    Ok(())
}

fn build_skip_tester(arg: Option<&str>) -> impl Fn(&usize) -> bool {
    enum R<T> {
        R(RangeInclusive<T>),
        F(RangeFrom<T>),
    }

    let list = arg.map(|a| {
        if let Some(a) = a.strip_prefix('!') {
            if let Ok(x) = a.parse::<usize>() {
                return vec![
                    R::R(RangeInclusive::new(1, x - 1)),
                    R::F(RangeFrom { start: x + 1 }),
                ];
            } else {
                return Vec::new();
            }
        }

        a.split(|c| c == ' ' || c == ',')
            .flat_map(|e| {
                let mut it = e.splitn(2, '-')
                    .map(|num| num.parse::<usize>())
                    .fuse();

                match (it.next(), it.next()) {
                    (Some(Ok(start)), Some(Ok(end)))
                        => Some(R::R(RangeInclusive::new(start, end))),

                    (Some(Ok(start)), Some(Err(err))) => {
                        if err.kind() == &Empty {
                            Some(R::F(RangeFrom { start }))
                        } else {
                            None
                        }
                    }

                    (Some(Ok(start)), None)
                        => Some(R::R(RangeInclusive::new(start, start))),
                    (_, _) => None,
                }
            })
            .collect::<Vec<_>>()
    });

    move |x| {
        if let Some(l) = &list {
            fn sel_key<T>(e: &R<T>) -> &T {
                match e {
                    R::R(r) => r.start(),
                    R::F(f) => &f.start,
                }
            }

            match l.binary_search_by_key(&x, sel_key) {
                Ok(_) => true,
                Err(0) => false,
                Err(pos) => {
                    match &l[pos - 1] {
                        R::R(range) => range.contains(x),
                        R::F(from) => from.contains(x),
                    }
                }
            }
        } else {
            false
        }
    }
}

fn print_test_suite_prelude() {
    let mut args = env::args().into_iter();
    print!(
        "\
        #![feature(assert_matches)]\n\
        #![feature(decl_macro)]\n\
        #![cfg(not(feature = \"location\"))]\n\
        //\n\
        // This file was generated with\n\
        //   {}",
        args.next().unwrap()
    );

    for arg in args {
        print!(" {}", arg);
    }
    println!("\n//");

    print!("{}", r#"
mod common;
use common::*;

fn space2tab(mut body: Vec<Block>) -> Vec<Block> {
    body.iter_mut().for_each(|e| {
        match e {
            Block::Code(_, ref mut txt, _) | Block::Html(ref mut txt, _)
                | Block::LinkDef(ref mut txt, ..) =>
            {
                *txt = txt.replace(' ', "\t");
            }

            Block::Paragraph(par, _) => {
                par.iter_mut().for_each(|e| {
                    match e {
                        Inline::Code(txt, _) => *txt = txt.replace(' ', "\t"),
                        _ => (),
                    }
                })
            }

            _ => (),
        }
    });

    body
}
"#);
}

fn parse_file(
    fname: &str
) -> anyhow::Result<impl Iterator<Item = io::Result<(String, usize, String, String)>>> {
    struct MyIter(Enumerate<Lines<BufReader<File>>>);

    impl Iterator for MyIter {
        type Item = io::Result<(String, usize, String, String)>;

        fn next(&mut self) -> Option<Self::Item> {
            let (kind, line_no);
            loop {
                // TODO: capture the last block before the example and put in the
                // doc-comment of the test function

                if let Some((no, line)) = self.0.next() {
                    let line = match line {
                        Ok(val) => val,
                        Err(err) => return Some(Err(err)),
                    };

                    if let Some(tail) = line.strip_prefix("```````````````````````````````` ") {
                        line_no = no + 1;
                        kind = tail.to_owned();
                        break;
                    }
                } else {
                    return None;
                }
            }

            let mut input = String::new();
            while let Some((_, line)) = self.0.next() {
                let line = match line {
                    Ok(val) => val,
                    Err(err) => return Some(Err(err)),
                };

                if line == "." {
                    break;
                }

                input += &line;
                input.push('\n')
            }

            let mut output = String::new();
            while let Some((_, line)) = self.0.next() {
                let line = match line {
                    Ok(val) => val,
                    Err(err) => return Some(Err(err)),
                };

                if line == "````````````````````````````````" {
                    if !output.is_empty() {
                        assert_eq!(output.pop(), Some('\n'),
                                   "output doesn't end on \\n: \"{}\"", output);
                    }

                    break;
                }

                output += &line;
                output.push('\n');
            }

            Some(Ok((kind, line_no, input, output)))
        }
    }

    Ok(MyIter(
        BufReader::new(
            File::open(fname).with_context(|| format!("File::open({})", fname))?
        ).lines().enumerate()
    ))
}

fn main() -> anyhow::Result<()> {
    let args = clap::Command::new(clap::crate_name!())
        .version(clap::crate_version!())
        .author(clap::crate_authors!(", "))
        .about(clap::crate_description!())
        .arg(clap::Arg::new("skip")
             .short('s')
             .value_name("SKIP")
             .help("IDs of test cases not to build")
             .num_args(1)
        ).arg(clap::Arg::new("ignores")
              .short('i')
              .value_name("IGNORE")
              .help("List of tests to mark as ignore")
              .num_args(1)
        ).arg(clap::Arg::new("exception_mode")
              .short('x')
              .help("Print exception file format instead of tests")
              .action(clap::ArgAction::SetTrue)
        ).arg(clap::Arg::new("spec")
              .value_name("INPUT")
              .help("File with CommonMark Spec")
              .num_args(1)
        ).arg(clap::Arg::new("exceptions")
              .value_name("ALT")
              .help("File(s) with divergent output")
              .num_args(0..)
              .action(clap::ArgAction::Set)
        ).get_matches();

    let skip = build_skip_tester(args.get_one::<String>("skip").map(|x| x.as_str()));
    let ignore = args.get_one::<String>("ignores").map_or(
        HashSet::new(),
        |x| x.split(',').flat_map(|e| e.parse::<usize>()).collect()
    );

    let exception_mode = args.get_flag("exception_mode");
    if !exception_mode {
        print_test_suite_prelude();
    }

    let mut exceptions : HashMap<_, Vec<_>> = HashMap::new();

    if let Some(ex) = args.get_many::<String>("exceptions") {
        for fname in ex {
            for e in parse_file(fname)? {
                let (kind, line, input, text) = e?;
                exceptions.entry(input)
                    .or_default()
                    .push(TestOutput {
                        kind,
                        src: format!("{}:{}", fname, line),
                        text,
                    });
            }
        }
    }

    let mut test_no = 0;
    for e in parse_file(args.get_one::<String>("spec").unwrap())? {
        let (kind, line, input, mut output) = e?;
        assert_eq!(kind, "example");

        test_no += 1;

        if skip(&test_no) {
            continue;
        }

        if exception_mode {
            gen_exception(test_no, &input, &output)?;
            continue;
        }

        let ex_rust = if let Some(ex) = exceptions.get(&input) {
            if let Some(repl) = ex.iter().find(|y| y.kind == "replacement") {
                output = repl.text.clone();
            }
            ex.iter().find(|y| y.kind == "rust").map(|x| &x.text)
        } else {
            None
        };

        gen_test(
            ignore.contains(&test_no), test_no, "", line,
            &input, &output, ex_rust,
            &mut TestSerializer { eol: "\n", ..Default::default() }
        )?;

        if ![565, 567].contains(&test_no) {
            // these tests are not possible for HTML spec test
            // 565: classification of references diverts from spec
            // 567: classification of references diverts from spec
/* TODO
            gen_html_test(
                ignore.contains(&test_no), test_no, line, &input, &output
            );
*/
        }

        if test_no != 325 {
            let input_cr = input.replace('\n', "\r");
            let sp_cr = ex_rust.map(|x| x.replace("\\n", "\\r"));
            if input_cr != input || sp_cr.as_ref() != ex_rust {
                gen_test(
                    ignore.contains(&test_no), test_no, "_cr", line,
                    &input_cr, &output, sp_cr.as_ref(),
                    &mut TestSerializer { eol: "\r", ..Default::default() }
                )?;
            }

            let input_cr_nl = input.replace('\n', "\r\n");
            let sp_cr_nl = ex_rust.map(|x| x.replace("\\n", "\\r\\n"));
            if input_cr_nl != input || sp_cr_nl.as_ref() != ex_rust {
                gen_test(
                    ignore.contains(&test_no), test_no, "_cr_nl", line,
                    &input_cr_nl, &output, sp_cr_nl.as_ref(),
                    &mut TestSerializer { eol: "\r\n", ..Default::default() }
                )?;
            }
        }

        let input_no_nl = input.trim_end_matches('\n');
        if input_no_nl != input {
            gen_test(
                ignore.contains(&test_no), test_no, "_no_nl", line,
                &input_no_nl, &output, ex_rust,
                &mut TestSerializer { eol: "\n", ..Default::default() }
            )?;
        }

        // these tests aren't translatable to a TAB version
        if [116, 196, 330, 552, 583, 632, 639, 646].contains(&test_no) {
            continue;
        }

        let mut input_with_tabs = String::with_capacity(input.len());
        let mut pos = 0;
        for ch in input.chars() {
            match ch {
                ' ' => {
                    pos += 1;
                    input_with_tabs.push(if pos <= 4 { ' ' } else { '\t' });
                }

                '\n' => {
                    pos = 0;
                    input_with_tabs.push(ch);
                }

                _ => {
                    pos = 5;
                    input_with_tabs.push(ch);
                }
            }
        }
        input_with_tabs = input_with_tabs.replace("\t\t\n", "  \n");

        let mut revert_backticks = input_with_tabs.replacen("`\t", "` ", 1);
        if revert_backticks != input_with_tabs {
            if let Some(idx) = revert_backticks.rfind("\t`") {
                revert_backticks.replace_range(idx..idx + 1, " ");

                if input_with_tabs != revert_backticks {
                    input_with_tabs = revert_backticks;
                }
            }
        }

        if input_with_tabs != input {
            gen_test(
                ignore.contains(&test_no), test_no, "_tab", line,
                &input_with_tabs, &output,
                ex_rust.map(|x| x.replace(' ', "\t")).as_ref(),
                &mut TestSerializer { space_to_tab: true, eol: "\n", ..Default::default() }
            )?;
        }
    }

    Ok(())
}
