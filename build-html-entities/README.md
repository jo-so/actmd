# Builder for HTML named entities

This is a helper crate to build a table of *[HTML named character
references](https://html.spec.whatwg.org/#named-character-references)* as Rust
*[array](https://doc.rust-lang.org/std/primitive.array.html)*. This table is
used in *actmd::parser* when parsing [HTML character
references](https://html.spec.whatwg.org/#character-reference-state).

## Usage

```bash
(
  url=https://html.spec.whatwg.org/entities.json
  echo //
  echo "// This file was generated with build-html-entities"
  echo "// from $url"
  echo "// and is used in src/parser/content.rs"
  echo //
  echo
  curl -s $url |cargo run
) > ../src/parser/html_entities.rs
```
