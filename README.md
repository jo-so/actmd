# Actmd: a parser and output generator for Markdown-like documents

## @

### Parsing

* `@{…}` ⇒ `Block(…)`
* `@(…)` ⇒ `Expr(…)`
* `@%{…}%` ⇒ `Block(%%…)` block comment
* `@%…` ⇒ `Block(%…)` line comment
* `@%\n *` ⇒ `Block(%"\\n ")` comment to remove newline
* `@[ASCII punctuation]?…(…)` ⇒ `Expr([]…(…))`

### Rendering

* `Block(%%…)` ⇒ `/*…*/\n`
* `Block(%…)` ⇒ `//…\n`
* `Block(…)` ⇒ `…\n`

* `Expr(#…)` ⇒ `(…).write_to(out)?;`
* `Expr(=…)` ⇒ `(…).write_attr_val(out)?;`
* `Expr(?…(…))` ⇒ `…(ctx, out, …)?;`
* `Expr(…)` ⇒ `(…).write_html_esc(out)?;`

#### Examples

* `@?deb("nginx")` ⇒ `deb(ctx, out, "nginx")?;`
* `@?content()` ⇒ `content(ctx, out)?;`

## Missing

* no indented code blocks
* no hard line-break with spaces
* no short links `[…]`; use `[…][]`
* no `#` at the end of ATX headings
* no setext headings
* links in image description
* deletion `~~…~~`
* LaTeX `@math("…")`, ` ```math `
* tables
* checklists `* [_]`, `* [X]`

* [How to move ahead with extending
  CommonMark](https://talk.commonmark.org/t/how-to-move-ahead-with-extending-commonmark/3706)
* [Beyond Markdown](https://talk.commonmark.org/t/beyond-markdown/2787)

* [Thoughts on changing supported html syntax to include the dangling closing
  angle bracket? - Spec - CommonMark
  Discussion](https://talk.commonmark.org/t/thoughts-on-changing-supported-html-syntax-to-include-the-dangling-closing-angle-bracket/3827)

### No indented code blocks, leading spaces get removed

* [We should abandon indented code blocks](https://talk.commonmark.org/t/we-should-abandon-indented-code-blocks/182)
* [Tabs and indentation](https://spec.commonmark.org/0.29/#example-6)

`> \t  CODE`

``` markdown
Some text
   # and a head line

Some text
    # neighter a head line nor a code block
```

``` markdown
<table>
  <tbody>
    <tr>
      <td>col 1</td>
      <td>col 2</td>
    </tr>

    <tr>
      <td>col 1</td>
      <td>col 2</td>
    </tr>
  </tbody>
</table>
```

### No hard line break with trailing spaces

### No shortcut reference linkstart

* <https://spec.commonmark.org/0.29/#shortcut-reference-link>

### No setext headings

* <https://spec.commonmark.org/0.29/#setext-headings>

### No ordered list start in paragraph

* <https://talk.commonmark.org/t/ordered-lists-shouldnt-trigger-for-any-number/3913/9>

## TODO

### Bessere Strings

Gefühlt müssen ständig Strings kopiert werden, um die Eigentümerschaft zu
sichern, damit der Code compiliert. Bearbeitet werden die Strings auch selten
und meist sind es kurze Strings; zum Beispiel bei den Attributen für Tags. Daher
wäre ein anderer Datentyp als `str`/`String` besser.

Möglichkeiten:

* `Rc<str>`
* [ArcStr](https://lib.rs/crates/arcstr): String mit eingebautem `Arc`
* [smartstring](https://docs.rs/smartstring/0.2.9/smartstring/): speichert kurze
  Strings (&lt; 23 Zeichen) inplace
* [beef](https://lib.rs/crates/beef): `Cow` mit geringerem Speicherbedarf
* [dairy](https://lib.rs/crates/dairy): ebenfalls `Cow` mit geringerem
  Speicherbedarf
* [Tendril](https://lib.rs/crates/tendril): ein String mit `Cow` und `Arc`, der
  aber nicht alle String-Methoden unterstützt; [`struct
  Tendril`](https://doc.servo.org/tendril/struct.Tendril.html)
* [KString — data structures in Rust](https://lib.rs/crates/kstring)

``` rust
fn main() {
    // cargo add arcstr smartstring smol_str beef tendril dairy kstring thin_str
    macro_rules! size {
        ($type:ty) => (
            println!(
                "On stack size of {:32}: {}", stringify!($type), std::mem::size_of::<$type>()
            );
        );
    }

    // https://stackoverflow.com/questions/30869007/how-to-benchmark-memory-usage-of-a-function
    size!(String);
    size!(&str);
    size!(Box<str>);
    size!(std::borrow::Cow<str>);
    size!(std::rc::Rc<str>);
    size!(std::sync::Arc<str>);
    size!(arcstr::ArcStr);
    size!(smartstring::alias::String);
    size!(smol_str::SmolStr);
    size!(dairy::Cow<str>);
    size!(tendril::StrTendril);
    size!(kstring::KString);
    size!(kstring::KStringCow);
    size!(thin_str::ThinStr);
}
```

``` text
On stack size of String                          : 24
On stack size of &str                            : 16
On stack size of Box<str>                        : 16
On stack size of std::borrow::Cow<str>           : 32
On stack size of std::rc::Rc<str>                : 16
On stack size of std::sync::Arc<str>             : 16
On stack size of arcstr::ArcStr                  : 8
On stack size of smartstring::alias::String      : 24
On stack size of smol_str::SmolStr               : 24
On stack size of dairy::Cow<str>                 : 16
On stack size of tendril::StrTendril             : 16
On stack size of kstring::KString                : 24
On stack size of kstring::KStringCow             : 32
On stack size of thin_str::ThinStr               : 8
```

### Über lazy_regex nachdenken

Prüfen, ob der Einsatz von [lazy-regex](https://lib.rs/crates/lazy-regex)
sinnvoll ist.
