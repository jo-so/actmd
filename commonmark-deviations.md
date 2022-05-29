# No `#` at the end of ATX headings

[Example 41](https://spec.commonmark.org/0.29/#example-41)

```````````````````````````````` replacement
## foo ##
  ###   bar    ###
.
<h2>foo ##</h2>
<h3>bar    ###</h3>
````````````````````````````````

[Example 42](https://spec.commonmark.org/0.29/#example-42)

```````````````````````````````` replacement
# foo ##################################
##### foo ##
.
<h1>foo ##################################</h1>
<h5>foo ##</h5>
````````````````````````````````

[Example 43](https://spec.commonmark.org/0.29/#example-43)

```````````````````````````````` replacement
### foo ###     
.
<h3>foo ###</h3>
````````````````````````````````

[Example 49](https://spec.commonmark.org/0.29/#example-49)

```````````````````````````````` replacement
## 
#
### ###
.
<h2></h2>
<h1></h1>
<h3>###</h3>
````````````````````````````````

# No setext headings

[Example 29](https://spec.commonmark.org/0.29/#example-29)

```````````````````````````````` replacement
Foo
---
bar
.
<p>Foo</p>
<hr />
<p>bar</p>
````````````````````````````````

[Example 50](https://spec.commonmark.org/0.29/#example-50)

```````````````````````````````` replacement
Foo *bar*
=========

Foo *bar*
---------
.
<p>Foo <em>bar</em>
=========</p>
<p>Foo <em>bar</em></p>
<hr />
````````````````````````````````

[Example 51](https://spec.commonmark.org/0.29/#example-51)

```````````````````````````````` replacement
Foo *bar
baz*
====
.
<p>Foo <em>bar
baz</em>
====</p>
````````````````````````````````

[Example 52](https://spec.commonmark.org/0.29/#example-52)

```````````````````````````````` replacement
  Foo *bar
baz*→
====
.
<p>Foo <em>bar
baz</em>
====</p>
````````````````````````````````

[Example 53](https://spec.commonmark.org/0.29/#example-53)

```````````````````````````````` replacement
Foo
-------------------------

Foo
=
.
<p>Foo</p>
<hr />
<p>Foo
=</p>
````````````````````````````````

[Example 54](https://spec.commonmark.org/0.29/#example-54)

```````````````````````````````` replacement
   Foo
---

  Foo
-----

  Foo
  ===
.
<p>Foo</p>
<hr />
<p>Foo</p>
<hr />
<p>Foo
===</p>
````````````````````````````````

[Example 56](https://spec.commonmark.org/0.29/#example-56)

```````````````````````````````` replacement
Foo
   ----      
.
<p>Foo</p>
<hr />
````````````````````````````````

[Example 59](https://spec.commonmark.org/0.29/#example-59)

```````````````````````````````` replacement
Foo  
-----
.
<p>Foo</p>
<hr />
````````````````````````````````

[Example 60](https://spec.commonmark.org/0.29/#example-60)

```````````````````````````````` replacement
Foo\
----
.
<p>Foo\</p>
<hr />
````````````````````````````````

[Example 61](https://spec.commonmark.org/0.29/#example-61)

```````````````````````````````` replacement
`Foo
----
`

<a title="a lot
---
of dashes"/>
.
<p>`Foo</p>
<hr />
<p>`</p>
<p>&lt;a title=&quot;a lot</p>
<hr />
<p>of dashes&quot;/&gt;</p>
````````````````````````````````

[Example 65](https://spec.commonmark.org/0.29/#example-65)

```````````````````````````````` replacement
Foo
Bar
---
.
<p>Foo
Bar</p>
<hr />
````````````````````````````````

[Example 66](https://spec.commonmark.org/0.29/#example-66)

```````````````````````````````` replacement
---
Foo
---
Bar
---
Baz
.
<hr />
<p>Foo</p>
<hr />
<p>Bar</p>
<hr />
<p>Baz</p>
````````````````````````````````

[Example 72](https://spec.commonmark.org/0.29/#example-72)

```````````````````````````````` replacement
\> foo
------
.
<p>&gt; foo</p>
<hr />
````````````````````````````````

[Example 73](https://spec.commonmark.org/0.29/#example-73)

```````````````````````````````` replacement
Foo

bar
---
baz
.
<p>Foo</p>
<p>bar</p>
<hr />
<p>baz</p>
````````````````````````````````

[Example 111](https://spec.commonmark.org/0.29/#example-111)

```````````````````````````````` replacement
foo
---
~~~
bar
~~~
# baz
.
<p>foo</p>
<hr />
<pre><code>bar
</code></pre>
<h1>baz</h1>
````````````````````````````````

[Example 184](https://spec.commonmark.org/0.29/#example-184)

```````````````````````````````` replacement
[foo]: /url
bar
===
[foo]
.
<p>bar
===
<a href="/url">foo</a></p>
````````````````````````````````

[Example 270](https://spec.commonmark.org/0.29/#example-270):

```````````````````````````````` replacement
- # Foo
- Bar
  ---
  baz
.
<ul>
<li>
<h1>Foo</h1>
</li>
<li>
<p>Bar</p>
<hr />
<p>baz</p>
</li>
</ul>
````````````````````````````````

# No hard line breaks with spaces

[Example 196](https://spec.commonmark.org/0.29/#example-196)

```````````````````````````````` replacement
aaa     
bbb     
.
<p>aaa
bbb</p>
````````````````````````````````

[Example 630](https://spec.commonmark.org/0.29/#example-630)

```````````````````````````````` replacement
foo  
baz
.
<p>foo
baz</p>
````````````````````````````````

[Example 632](https://spec.commonmark.org/0.29/#example-632)

```````````````````````````````` replacement
foo       
baz
.
<p>foo
baz</p>
````````````````````````````````

[Example 633](https://spec.commonmark.org/0.29/#example-633)

```````````````````````````````` replacement
foo  
     bar
.
<p>foo
bar</p>
````````````````````````````````

[Example 635](https://spec.commonmark.org/0.29/#example-635)

```````````````````````````````` replacement
*foo  
bar*
.
<p><em>foo
bar</em></p>
````````````````````````````````

# No indented code blocks

[Example 1](https://spec.commonmark.org/0.29/#example-1):

```````````````````````````````` replacement
→foo→baz→→bim
.
<p>foo→baz→→bim</p>
````````````````````````````````

[Example 2](https://spec.commonmark.org/0.29/#example-2):

```````````````````````````````` replacement
  →foo→baz→→bim
.
<p>foo→baz→→bim</p>
````````````````````````````````

[Example 3](https://spec.commonmark.org/0.29/#example-3):

```````````````````````````````` replacement
    a→a
    ὐ→a
.
<p>a→a
ὐ→a</p>
````````````````````````````````

[Example 5](https://spec.commonmark.org/0.29/#example-5):

```````````````````````````````` replacement
- foo

→→bar
.
<ul>
<li>
<p>foo</p>
<p>bar</p>
</li>
</ul>
````````````````````````````````

[Example 6](https://spec.commonmark.org/0.29/#example-6):

```````````````````````````````` replacement
>→→foo
.
<blockquote><p>foo</p></blockquote>
````````````````````````````````

[Example 7](https://spec.commonmark.org/0.29/#example-7):

```````````````````````````````` replacement
-→→foo
.
<ul><li>foo</li></ul>
````````````````````````````````

[Example 8](https://spec.commonmark.org/0.29/#example-8):

```````````````````````````````` replacement
    foo
→bar
.
<p>foo
bar</p>
````````````````````````````````

[Example 18](https://spec.commonmark.org/0.29/#example-18):

```````````````````````````````` replacement
    ***
.
<hr />
````````````````````````````````

[Example 19](https://spec.commonmark.org/0.29/#example-19):

```````````````````````````````` replacement
Foo
    ***
.
<p>Foo</p>
<hr />
````````````````````````````````

[Example 39](https://spec.commonmark.org/0.29/#example-39):

```````````````````````````````` replacement
    # foo
.
<h1>foo</h1>
````````````````````````````````

[Example 40](https://spec.commonmark.org/0.29/#example-40):

```````````````````````````````` replacement
foo
    # bar
.
<p>foo</p>
<h1>bar</h1>
````````````````````````````````

[Example 55](https://spec.commonmark.org/0.29/#example-55):

```````````````````````````````` replacement
    Foo
    ---

    Foo
---
.
<p>Foo</p>
<hr />
<p>Foo</p>
<hr />
````````````````````````````````

[Example 57](https://spec.commonmark.org/0.29/#example-57):

```````````````````````````````` replacement
Foo
    ---
.
<p>Foo</p>
<hr />
````````````````````````````````

[Example 70](https://spec.commonmark.org/0.29/#example-70):

```````````````````````````````` replacement
    foo
---
.
<p>foo</p>
<hr />
````````````````````````````````

[Example 77](https://spec.commonmark.org/0.29/#example-77):

```````````````````````````````` replacement
    a simple
      indented code block
.
<p>a simple
indented code block</p>
````````````````````````````````

[Example 80](https://spec.commonmark.org/0.29/#example-80):

```````````````````````````````` rust
    <a/>
    *hi*

    - one
.
html_block!("    <a/>\n    *hi*\n"),
unordered_list!(vec![ paragraph!(plain!("one")) ])
````````````````````````````````

[Example 81](https://spec.commonmark.org/0.29/#example-81):

```````````````````````````````` replacement
    chunk1

    chunk2
  
 
 
    chunk3
.
<p>chunk1</p>
<p>chunk2</p>
<p>chunk3</p>
````````````````````````````````

[Example 82](https://spec.commonmark.org/0.29/#example-82):

```````````````````````````````` replacement
    chunk1
      
      chunk2
.
<p>chunk1</p>
<p>chunk2</p>
````````````````````````````````

[Example 84](https://spec.commonmark.org/0.29/#example-84):

```````````````````````````````` replacement
    foo
bar
.
<p>foo
bar</p>
````````````````````````````````

[Example 85](https://spec.commonmark.org/0.29/#example-85):

```````````````````````````````` replacement
# Heading
    foo
Heading
------
    foo
----
.
<h1>Heading</h1>
<p>foo
Heading</p>
<hr />
<p>foo</p>
<hr />
````````````````````````````````

[Example 86](https://spec.commonmark.org/0.29/#example-86):

```````````````````````````````` replacement
        foo
    bar
.
<p>foo
bar</p>
````````````````````````````````

[Example 87](https://spec.commonmark.org/0.29/#example-87):

```````````````````````````````` replacement

    
    foo
    

.
<p>foo</p>
````````````````````````````````

[Example 88](https://spec.commonmark.org/0.29/#example-88):

```````````````````````````````` replacement
    foo  
.
<p>foo</p>
````````````````````````````````

[Example 104](https://spec.commonmark.org/0.29/#example-104):

```````````````````````````````` replacement
    ```
    aaa
    ```
.
<pre><code>aaa
</code></pre>
````````````````````````````````

[Example 195](https://spec.commonmark.org/0.29/#example-195):

```````````````````````````````` replacement
    aaa
bbb
.
<p>aaa
bbb</p>
````````````````````````````````

[Example 201](https://spec.commonmark.org/0.29/#example-201):

```````````````````````````````` replacement
    > # Foo
    > bar
    > baz
.
<blockquote>
<h1>Foo</h1>
<p>bar
baz</p>
</blockquote>
````````````````````````````````

[Example 206](https://spec.commonmark.org/0.29/#example-201):

```````````````````````````````` replacement
>     foo
    bar
.
<blockquote>
<p>foo
bar</p>
</blockquote>
````````````````````````````````

[Example 222](https://spec.commonmark.org/0.29/#example-222):

```````````````````````````````` replacement
>     code

>    not code
.
<blockquote>
<p>code</p>
</blockquote>
<blockquote>
<p>not code</p>
</blockquote>
````````````````````````````````

[Example 223](https://spec.commonmark.org/0.29/#example-223):

```````````````````````````````` replacement
A paragraph
with two lines.

    indented code

> A block quote.
.
<p>A paragraph
with two lines.</p>
<p>indented code</p>
<blockquote>
<p>A block quote.</p>
</blockquote>
````````````````````````````````

[Example 224](https://spec.commonmark.org/0.29/#example-224):

```````````````````````````````` replacement
1.  A paragraph
    with two lines.

        indented code

    > A block quote.
.
<ol>
<li>
<p>A paragraph
with two lines.</p>
<p>indented code</p>
<blockquote>
<p>A block quote.</p>
</blockquote>
</li>
</ol>
````````````````````````````````

[Example 234](https://spec.commonmark.org/0.29/#example-234):

```````````````````````````````` replacement
- Foo

      bar


      baz
.
<ul>
<li>
<p>Foo</p>
<p>bar</p>
<p>baz</p>
</li>
</ul>
````````````````````````````````

[Example 240](https://spec.commonmark.org/0.29/#example-240):

```````````````````````````````` replacement
- foo

      bar
.
<ul>
<li>
<p>foo</p>
<p>bar</p>
</li>
</ul>
````````````````````````````````

[Example 241](https://spec.commonmark.org/0.29/#example-241):

```````````````````````````````` replacement
  10.  foo

           bar
.
<ol start="10">
<li>
<p>foo</p>
<p>bar</p>
</li>
</ol>
````````````````````````````````

[Example 242](https://spec.commonmark.org/0.29/#example-242):

```````````````````````````````` replacement
    indented code

paragraph

    more code
.
<p>indented code</p>
<p>paragraph</p>
<p>more code</p>
````````````````````````````````

[Example 243](https://spec.commonmark.org/0.29/#example-243):

```````````````````````````````` replacement
1.     indented code

   paragraph

       more code
.
<ol>
<li>
<p>indented code</p>
<p>paragraph</p>
<p>more code</p>
</li>
</ol>
````````````````````````````````

[Example 244](https://spec.commonmark.org/0.29/#example-244):

```````````````````````````````` replacement
1.      indented code

   paragraph

       more code
.
<ol>
<li>
<p>indented code</p>
<p>paragraph</p>
<p>more code</p>
</li>
</ol>
````````````````````````````````

[Example 248](https://spec.commonmark.org/0.29/#example-248):

```````````````````````````````` replacement
-
  foo
-
  ```
  bar
  ```
-
      baz
.
<ul>
<li>foo</li>
<li>
<pre><code>bar
</code></pre>
</li>
<li>
<p>baz</p>
</li>
</ul>
````````````````````````````````

[Example 256](https://spec.commonmark.org/0.29/#example-256):

```````````````````````````````` replacement
 1.  A paragraph
     with two lines.

         indented code

     > A block quote.
.
<ol>
<li>
<p>A paragraph
with two lines.</p>
<p>indented code</p>
<blockquote>
<p>A block quote.</p>
</blockquote>
</li>
</ol>
````````````````````````````````

[Example 257](https://spec.commonmark.org/0.29/#example-257):

```````````````````````````````` replacement
  1.  A paragraph
      with two lines.

          indented code

      > A block quote.
.
<ol>
<li>
<p>A paragraph
with two lines.</p>
<p>indented code</p>
<blockquote>
<p>A block quote.</p>
</blockquote>
</li>
</ol>
````````````````````````````````

[Example 258](https://spec.commonmark.org/0.29/#example-258):

```````````````````````````````` replacement
   1.  A paragraph
       with two lines.

           indented code

       > A block quote.
.
<ol>
<li>
<p>A paragraph
with two lines.</p>
<p>indented code</p>
<blockquote>
<p>A block quote.</p>
</blockquote>
</li>
</ol>
````````````````````````````````

[Example 259](https://spec.commonmark.org/0.29/#example-259):

```````````````````````````````` replacement
    1.  A paragraph
        with two lines.

            indented code

        > A block quote.
.
<ol>
<li>
<p>A paragraph
with two lines.</p>
<p>indented code</p>
<blockquote>
<p>A block quote.</p>
</blockquote>
</li>
</ol>
````````````````````````````````

[Example 279](https://spec.commonmark.org/0.29/#example-279):

```````````````````````````````` rust
-   foo

    notcode

-   foo

<!-- -->

    code
.
unordered_list!(
  vec![
    paragraph!(plain!("foo")),
    paragraph!(plain!("notcode"))
  ],
  vec![paragraph!(plain!("foo"))]
),
html_block!("<!-- -->\n"),
paragraph!(plain!("code"))
````````````````````````````````

[Example 304](https://spec.commonmark.org/0.29/#example-304):

```````````````````````````````` replacement
    \[\]
.
<p>[]</p>
````````````````````````````````

[Example 322](https://spec.commonmark.org/0.29/#example-322):

```````````````````````````````` replacement
    f&ouml;f&ouml;
.
<p>föfö</p>
````````````````````````````````

# No short link references

[Example 309](https://spec.commonmark.org/0.29/#example-309)

```````````````````````````````` rust
[foo]

[foo]: /bar\* "ti\*tle"
.
paragraph!(linkref!(plain!("foo"))),
linkdef!("foo", "/bar*", "ti*tle")
````````````````````````````````

[Example 319](https://spec.commonmark.org/0.29/#example-319)

```````````````````````````````` rust
[foo]

[foo]: /f&ouml;&ouml; "f&ouml;&ouml;"
.
paragraph!(linkref!(plain!("foo"))),
linkdef!("foo", "/f\u{f6}\u{f6}", "f\u{f6}\u{f6}")
````````````````````````````````

# Non parse-able HTML

Not all HTML output of the examples in the [CommonMark
Spec](commonmark-spec.txt) can dynamicly remapped to a Rust expression.

[Example 113](https://spec.commonmark.org/0.29/#example-113)

The class attribute doesn't contain all characters from the input

```````````````````````````````` rust
~~~~    ruby startline=3 $%@#$
def foo(x)
  return 3
end
~~~~~~~
.
code_block!("ruby startline=3 $%@#$", "def foo(x)\n  return 3\nend\n")
````````````````````````````````

[Example 116](https://spec.commonmark.org/0.29/#example-116)

The class attribute doesn't contain all characters from the input

```````````````````````````````` rust
~~~ aa ``` ~~~
foo
~~~
.
code_block!("aa ``` ~~~", "foo\n")
````````````````````````````````

[Example 307](https://spec.commonmark.org/0.29/#example-307)

```````````````````````````````` rust
<a href="/bar\/)">
.
html_block!("<a href=\"/bar\\/)\">\n")
````````````````````````````````

[Example 317](https://spec.commonmark.org/0.29/#example-317)

```````````````````````````````` rust
<a href="&ouml;&ouml;.html">
.
html_block!("<a href=\"&ouml;&ouml;.html\">\n")
````````````````````````````````

[Example 325](https://spec.commonmark.org/0.29/#example-325)

```````````````````````````````` rust
foo&#10;&#10;bar
.
paragraph!(plain!("foo\n\nbar"))
````````````````````````````````

[Example 344](https://spec.commonmark.org/0.29/#example-344)

```````````````````````````````` rust
<a href="`">`
.
paragraph!(html!("<a href=\"`\">"), plain!("`"))
````````````````````````````````

[Example 520](https://spec.commonmark.org/0.29/#example-520)

```````````````````````````````` rust
[foo <bar attr="](baz)">
.
paragraph!(plain!("[foo "), html!("<bar attr=\"](baz)\">"))
````````````````````````````````

[Example 532](https://spec.commonmark.org/0.29/#example-532)

```````````````````````````````` rust
[foo <bar attr="][ref]">

[ref]: /uri
.
paragraph!(plain!("[foo "), html!("<bar attr=\"][ref]\">")),
linkdef!("ref", "/uri")
````````````````````````````````

## HTML block

[Example 118](https://spec.commonmark.org/0.29/#example-118)

```````````````````````````````` rust
<table><tr><td>
<pre>
**Hello**,

_world_.
</pre>
</td></tr></table>
.
html_block!("<table><tr><td>\n<pre>\n**Hello**,\n"),
paragraph!(
  emph!(plain!("world")), plain!("."), SoftBreak,
  html!("</pre>")
),
html_block!("</td></tr></table>\n")
````````````````````````````````

[Example 119](https://spec.commonmark.org/0.29/#example-119)

```````````````````````````````` rust
<table>
  <tr>
    <td>
           hi
    </td>
  </tr>
</table>

okay.
.
html_block!("<table>\n  <tr>\n    <td>\n           hi\n    </td>\n  </tr>\n</table>\n"),
paragraph!(plain!("okay."))
````````````````````````````````

[Example 120](https://spec.commonmark.org/0.29/#example-120)

```````````````````````````````` rust
 <div>
  *hello*
         <foo><a>
.
html_block!(" <div>\n  *hello*\n         <foo><a>\n")
````````````````````````````````

[Example 121](https://spec.commonmark.org/0.29/#example-121)

```````````````````````````````` rust
</div>
*foo*
.
html_block!("</div>\n*foo*\n")
````````````````````````````````

[Example 122](https://spec.commonmark.org/0.29/#example-122)

```````````````````````````````` rust
<DIV CLASS="foo">

*Markdown*

</DIV>
.
html_block!("<DIV CLASS=\"foo\">\n"),
paragraph!(emph!(plain!("Markdown"))),
html_block!("</DIV>\n"),
````````````````````````````````

[Example 123](https://spec.commonmark.org/0.29/#example-123)

```````````````````````````````` rust
<div id="foo"
  class="bar">
</div>
.
html_block!("<div id=\"foo\"\n  class=\"bar\">\n</div>\n")
````````````````````````````````

[Example 124](https://spec.commonmark.org/0.29/#example-124)

```````````````````````````````` rust
<div id="foo" class="bar
  baz">
</div>
.
html_block!("<div id=\"foo\" class=\"bar\n  baz\">\n</div>\n")
````````````````````````````````

[Example 125](https://spec.commonmark.org/0.29/#example-125)

```````````````````````````````` rust
<div>
*foo*

*bar*
.
html_block!("<div>\n*foo*\n"), paragraph!(emph!(plain!("bar")))
````````````````````````````````

[Example 126](https://spec.commonmark.org/0.29/#example-126)

```````````````````````````````` rust
<div id="foo"
*hi*
.
html_block!("<div id=\"foo\"\n*hi*\n")
````````````````````````````````

[Example 127](https://spec.commonmark.org/0.29/#example-127)

```````````````````````````````` rust
<div class
foo
.
html_block!("<div class\nfoo\n")
````````````````````````````````

[Example 128](https://spec.commonmark.org/0.29/#example-128)

```````````````````````````````` rust
<div *???-&&&-<---
*foo*
.
html_block!("<div *???-&&&-<---\n*foo*\n")
````````````````````````````````

[Example 129](https://spec.commonmark.org/0.29/#example-129)

```````````````````````````````` rust
<div><a href="bar">*foo*</a></div>
.
html_block!("<div><a href=\"bar\">*foo*</a></div>\n")
````````````````````````````````

[Example 130](https://spec.commonmark.org/0.29/#example-130)

```````````````````````````````` rust
<table><tr><td>
foo
</td></tr></table>
.
html_block!("<table><tr><td>\nfoo\n</td></tr></table>\n")
````````````````````````````````

[Example 131](https://spec.commonmark.org/0.29/#example-131)

```````````````````````````````` rust
<div></div>
``` c
int x = 33;
```
.
html_block!("<div></div>\n``` c\nint x = 33;\n```\n")
````````````````````````````````

[Example 132](https://spec.commonmark.org/0.29/#example-132)

```````````````````````````````` rust
<a href="foo">
*bar*
</a>
.
html_block!("<a href=\"foo\">\n*bar*\n</a>\n")
````````````````````````````````

[Example 133](https://spec.commonmark.org/0.29/#example-133)

```````````````````````````````` rust
<Warning>
*bar*
</Warning>
.
html_block!("<Warning>\n*bar*\n</Warning>\n")
````````````````````````````````

[Example 134](https://spec.commonmark.org/0.29/#example-134)

```````````````````````````````` rust
<i class="foo">
*bar*
</i>
.
html_block!("<i class=\"foo\">\n*bar*\n</i>\n")
````````````````````````````````

[Example 135](https://spec.commonmark.org/0.29/#example-135)

```````````````````````````````` rust
</ins>
*bar*
.
html_block!("</ins>\n*bar*\n")
````````````````````````````````

[Example 136](https://spec.commonmark.org/0.29/#example-136)

```````````````````````````````` rust
<del>
*foo*
</del>
.
html_block!("<del>\n*foo*\n</del>\n")
````````````````````````````````

[Example 137](https://spec.commonmark.org/0.29/#example-137)

```````````````````````````````` rust
<del>

*foo*

</del>
.
html_block!("<del>\n"), paragraph!(emph!(plain!("foo"))), html_block!("</del>\n")
````````````````````````````````

[Example 138](https://spec.commonmark.org/0.29/#example-138)

```````````````````````````````` rust
<del>*foo*</del>
.
paragraph!(html!("<del>"), emph!(plain!("foo")), html!("</del>"))
````````````````````````````````

[Example 139](https://spec.commonmark.org/0.29/#example-139)

```````````````````````````````` rust
<pre language="haskell"><code>
import Text.HTML.TagSoup

main :: IO ()
main = print $ parseTags tags
</code></pre>
okay
.
html_block!("<pre language=\"haskell\"><code>\nimport Text.HTML.TagSoup\n\nmain :: IO ()\nmain = print $ parseTags tags\n</code></pre>\n"),
paragraph!(plain!("okay"))
````````````````````````````````

[Example 140](https://spec.commonmark.org/0.29/#example-140)

```````````````````````````````` rust
<script type="text/javascript">
// JavaScript example

document.getElementById("demo").innerHTML = "Hello JavaScript!";
</script>
okay
.
html_block!("<script type=\"text/javascript\">\n// JavaScript example\n\ndocument.getElementById(\"demo\").innerHTML = \"Hello JavaScript!\";\n</script>\n"),
paragraph!(plain!("okay"))
````````````````````````````````

[Example 141](https://spec.commonmark.org/0.29/#example-141)

```````````````````````````````` rust
<style
  type="text/css">
h1 {color:red;}

p {color:blue;}
</style>
okay
.
html_block!("<style\n  type=\"text/css\">\nh1 {color:red;}\n\np {color:blue;}\n</style>\n"),
paragraph!(plain!("okay"))
````````````````````````````````

[Example 142](https://spec.commonmark.org/0.29/#example-142)

```````````````````````````````` rust
<style
  type="text/css">

foo
.
html_block!("<style\n  type=\"text/css\">\n\nfoo\n")
````````````````````````````````

[Example 143](https://spec.commonmark.org/0.29/#example-143)

```````````````````````````````` rust
> <div>
> foo

bar
.
quote!(html_block!("<div>\nfoo\n")),
paragraph!(plain!("bar"))
````````````````````````````````

[Example 144](https://spec.commonmark.org/0.29/#example-144):

```````````````````````````````` rust
- <div>
- foo
.
unordered_list!(
  vec![ html_block!("<div>\n") ],
  vec![ paragraph!(plain!("foo")) ]
)
````````````````````````````````

[Example 146](https://spec.commonmark.org/0.29/#example-146)

```````````````````````````````` rust
<!-- foo -->*bar*
*baz*
.
html_block!("<!-- foo -->*bar*\n"),
paragraph!(emph!(plain!("baz")))
````````````````````````````````

[Example 147](https://spec.commonmark.org/0.29/#example-147)

```````````````````````````````` rust
<script>
foo
</script>1. *bar*
.
html_block!("<script>\nfoo\n</script>1. *bar*\n")
````````````````````````````````

[Example 149](https://spec.commonmark.org/0.29/#example-149)

```````````````````````````````` rust
<?php

  echo '>';

?>
okay
.
html_block!("<?php\n\n  echo '>';\n\n?>\n"),
paragraph!(plain!("okay"))
````````````````````````````````

[Example 150](https://spec.commonmark.org/0.29/#example-150)

```````````````````````````````` rust
<!DOCTYPE html>
.
html_block!("<!DOCTYPE html>\n")
````````````````````````````````

[Example 151](https://spec.commonmark.org/0.29/#example-151)

```````````````````````````````` rust
<![CDATA[
function matchwo(a,b)
{
  if (a < b && a < 0) then {
    return 1;

  } else {

    return 0;
  }
}
]]>
okay
.
html_block!("<![CDATA[\nfunction matchwo(a,b)\n{\n  if (a < b && a < 0) then {\n    return 1;\n\n  } else {\n\n    return 0;\n  }\n}\n]]>\n"),
paragraph!(plain!("okay"))
````````````````````````````````

[Example 152](https://spec.commonmark.org/0.29/#example-152)

```````````````````````````````` rust
  <!-- foo -->

    <!-- foo -->
.
html_block!("  <!-- foo -->\n"),
html_block!("    <!-- foo -->\n")
````````````````````````````````

[Example 153](https://spec.commonmark.org/0.29/#example-153)

```````````````````````````````` rust
  <div>

    <div>
.
html_block!("  <div>\n"),
html_block!("    <div>\n"),
````````````````````````````````

[Example 154](https://spec.commonmark.org/0.29/#example-154)

```````````````````````````````` rust
Foo
<div>
bar
</div>
.
paragraph!(plain!("Foo")),
html_block!("<div>\nbar\n</div>\n")
````````````````````````````````

[Example 155](https://spec.commonmark.org/0.29/#example-155)

```````````````````````````````` rust
<div>
bar
</div>
*foo*
.
html_block!("<div>\nbar\n</div>\n*foo*\n")
````````````````````````````````

[Example 156](https://spec.commonmark.org/0.29/#example-156)

```````````````````````````````` rust
Foo
<a href="bar">
baz
.
paragraph!(
  plain!("Foo"),
  SoftBreak,
  html!("<a href=\"bar\">"),
  SoftBreak,
  plain!("baz")
)
````````````````````````````````

[Example 157](https://spec.commonmark.org/0.29/#example-157)

```````````````````````````````` rust
<div>

*Emphasized* text.

</div>
.
html_block!("<div>\n"),
paragraph!(emph!(plain!("Emphasized")), plain!(" text.")),
html_block!("</div>\n"),
````````````````````````````````

[Example 158](https://spec.commonmark.org/0.29/#example-158)

```````````````````````````````` rust
<div>
*Emphasized* text.
</div>
.
html_block!("<div>\n*Emphasized* text.\n</div>\n")
````````````````````````````````

[Example 159](https://spec.commonmark.org/0.29/#example-159)

```````````````````````````````` rust
<table>

<tr>

<td>
Hi
</td>

</tr>

</table>
.
html_block!("<table>\n"),
html_block!("<tr>\n"),
html_block!("<td>\nHi\n</td>\n"),
html_block!("</tr>\n"),
html_block!("</table>\n")
````````````````````````````````

[Example 160](https://spec.commonmark.org/0.29/#example-160)

```````````````````````````````` rust
<table>

  <tr>

    <td>
      Hi
    </td>

  </tr>

</table>
.
html_block!("<table>\n"),
html_block!("  <tr>\n"),
html_block!("    <td>\n      Hi\n    </td>\n"),
html_block!("  </tr>\n"),
html_block!("</table>\n")
````````````````````````````````

[Example 474](https://spec.commonmark.org/0.29/#example-474)

```````````````````````````````` rust
*<img src="foo" title="*"/>
.
paragraph!(plain!("*"), html!("<img src=\"foo\" title=\"*\"/>"))
````````````````````````````````

[Example 475](https://spec.commonmark.org/0.29/#example-475)

```````````````````````````````` rust
**<a href="**">
.
paragraph!(plain!("**"), html!("<a href=\"**\">"))
````````````````````````````````

[Example 476](https://spec.commonmark.org/0.29/#example-476)

```````````````````````````````` rust
__<a href="__">
.
paragraph!(plain!("__"), html!("<a href=\"__\">"))
````````````````````````````````

## Links

[Example 161](https://spec.commonmark.org/0.29/#example-161)

```````````````````````````````` rust
[foo]: /url "title"

[foo]
.
linkdef!("foo", "/url", "title"),
paragraph!(linkref!(plain!("foo")))
````````````````````````````````

[Example 162](https://spec.commonmark.org/0.29/#example-162)

```````````````````````````````` rust
   [foo]: 
      /url  
           'the title'  

[foo]
.
linkdef!("foo", "/url", "the title"),
paragraph!(linkref!(plain!("foo")))
````````````````````````````````

[Example 163](https://spec.commonmark.org/0.29/#example-163)

That's the opposite of
[Example 519](https://spec.commonmark.org/0.29/#example-519) and can't be
decided during parsing. If the user really want a reference, he should write
`[Foo*bar\]][]`. See also Example 560.

```````````````````````````````` rust
[Foo*bar\]]:my_(url) 'title (with parens)'

[Foo*bar\]]
.
linkdef!("Foo*bar]", "my_(url)", "title (with parens)"),
paragraph!(plain!("[Foo*bar]]"))
````````````````````````````````

[Example 164](https://spec.commonmark.org/0.29/#example-164)

```````````````````````````````` rust
[Foo bar]:
<my url>
'title'

[Foo bar]
.
linkdef!("Foo bar", "my url", "title"),
paragraph!(linkref!(plain!("Foo bar")))
````````````````````````````````

[Example 165](https://spec.commonmark.org/0.29/#example-165)

```````````````````````````````` rust
[foo]: /url '
title
line1
line2
'

[foo]
.
linkdef!("foo", "/url", "\ntitle\nline1\nline2\n"),
paragraph!(linkref!(plain!("foo")))
````````````````````````````````

[Example 166](https://spec.commonmark.org/0.29/#example-166)

```````````````````````````````` rust
[foo]: /url 'title

with blank line'

[foo]
.
paragraph!(linkref!(plain!("foo")), plain!(": /url 'title")),
paragraph!(plain!("with blank line'")),
paragraph!(linkref!(plain!("foo")))
````````````````````````````````

[Example 167](https://spec.commonmark.org/0.29/#example-167)

```````````````````````````````` rust
[foo]:
/url

[foo]
.
linkdef!("foo", "/url"),
paragraph!(linkref!(plain!("foo")))
````````````````````````````````

[Example 168](https://spec.commonmark.org/0.29/#example-168)

```````````````````````````````` rust
[foo]:

[foo]
.
paragraph!(linkref!(plain!("foo")), plain!(":")),
paragraph!(linkref!(plain!("foo")))
````````````````````````````````

[Example 169](https://spec.commonmark.org/0.29/#example-169)

```````````````````````````````` rust
[foo]: <>

[foo]
.
linkdef!("foo", ""),
paragraph!(linkref!(plain!("foo")))
````````````````````````````````

[Example 170](https://spec.commonmark.org/0.29/#example-170)

```````````````````````````````` rust
[foo]: <bar>(baz)

[foo]
.
paragraph!(linkref!(plain!("foo")), plain!(": "), html!("<bar>"), plain!("(baz)")),
paragraph!(linkref!(plain!("foo")))
````````````````````````````````

[Example 171](https://spec.commonmark.org/0.29/#example-171)

```````````````````````````````` rust
[foo]: /url\bar\*baz "foo\"bar\baz"

[foo]
.
linkdef!("foo", "/url\\bar*baz", "foo\"bar\\baz"),
paragraph!(linkref!(plain!("foo")))
````````````````````````````````

[Example 172](https://spec.commonmark.org/0.29/#example-172)

```````````````````````````````` rust
[foo]

[foo]: url
.
paragraph!(linkref!(plain!("foo"))),
linkdef!("foo", "url")
````````````````````````````````

[Example 173](https://spec.commonmark.org/0.29/#example-173)

```````````````````````````````` rust
[foo]

[foo]: first
[foo]: second
.
paragraph!(linkref!(plain!("foo"))),
linkdef!("foo", "first"),
linkdef!("foo", "second")
````````````````````````````````

[Example 174](https://spec.commonmark.org/0.29/#example-174)

```````````````````````````````` rust
[FOO]: /url

[Foo]
.
linkdef!("FOO", "/url"),
paragraph!(linkref!(plain!("Foo")))
````````````````````````````````

[Example 175](https://spec.commonmark.org/0.29/#example-175)

```````````````````````````````` rust
[ΑΓΩ]: /φου

[αγω]
.
linkdef!("ΑΓΩ", "/\u{3c6}\u{3bf}\u{3c5}"),
paragraph!(linkref!(plain!("αγω")))
````````````````````````````````

[Example 176](https://spec.commonmark.org/0.29/#example-176)
and [Example 188](https://spec.commonmark.org/0.29/#example-188)

```````````````````````````````` rust
[foo]: /url
.
linkdef!("foo", "/url")
````````````````````````````````

[Example 177](https://spec.commonmark.org/0.29/#example-177)

```````````````````````````````` rust
[
foo
]: /url
bar
.
linkdef!("\nfoo\n", "/url"),
paragraph!(plain!("bar"))
````````````````````````````````

[Example 178](https://spec.commonmark.org/0.29/#example-178)

```````````````````````````````` rust
[foo]: /url "title" ok
.
paragraph!(linkref!(plain!("foo")), plain!(": /url \"title\" ok"))
````````````````````````````````

[Example 179](https://spec.commonmark.org/0.29/#example-179)

```````````````````````````````` rust
[foo]: /url
"title" ok
.
linkdef!("foo", "/url"),
paragraph!(plain!("\"title\" ok"))
````````````````````````````````

[Example 180](https://spec.commonmark.org/0.29/#example-180)

```````````````````````````````` rust
    [foo]: /url "title"

[foo]
.
linkdef!("foo", "/url", "title"),
paragraph!(linkref!(plain!("foo")))
````````````````````````````````

[Example 181](https://spec.commonmark.org/0.29/#example-181)

```````````````````````````````` rust
```
[foo]: /url
```

[foo]
.
code_block!("", "[foo]: /url\n"),
paragraph!(linkref!(plain!("foo")))
````````````````````````````````

[Example 182](https://spec.commonmark.org/0.29/#example-182)

```````````````````````````````` rust
Foo
[bar]: /baz

[bar]
.
paragraph!(plain!("Foo"), SoftBreak, linkref!(plain!("bar")), plain!(": /baz")),
paragraph!(linkref!(plain!("bar")))
````````````````````````````````

[Example 183](https://spec.commonmark.org/0.29/#example-183):

```````````````````````````````` rust
# [Foo]
[foo]: /url
> bar
.
heading!(1, linkref!(plain!("Foo"))),
linkdef!("foo", "/url"),
quote!(paragraph!(plain!("bar")))
````````````````````````````````

[Example 184](https://spec.commonmark.org/0.29/#example-184)

```````````````````````````````` rust
[foo]: /url
bar
===
[foo]
.
linkdef!("foo", "/url"),
paragraph!(plain!("bar"), SoftBreak, plain!("==="), SoftBreak, linkref!(plain!("foo")))
````````````````````````````````

[Example 185](https://spec.commonmark.org/0.29/#example-185)

```````````````````````````````` rust
[foo]: /url
===
[foo]
.
linkdef!("foo", "/url"),
paragraph!(plain!("==="), SoftBreak, linkref!(plain!("foo")))
````````````````````````````````

[Example 186](https://spec.commonmark.org/0.29/#example-186)

```````````````````````````````` rust
[foo]: /foo-url "foo"
[bar]: /bar-url
  "bar"
[baz]: /baz-url

[foo],
[bar],
[baz]
.
linkdef!("foo", "/foo-url", "foo"),
linkdef!("bar", "/bar-url", "bar"),
linkdef!("baz", "/baz-url"),
paragraph!(
    linkref!(plain!("foo")),
    plain!(","), SoftBreak,
    linkref!(plain!("bar")),
    plain!(","), SoftBreak,
    linkref!(plain!("baz"))
)
````````````````````````````````

[Example 187](https://spec.commonmark.org/0.29/#example-187):

```````````````````````````````` rust
[foo]

> [foo]: /url
.
paragraph!(linkref!(plain!("foo"))),
quote!(linkdef!("foo", "/url"))
````````````````````````````````

[Example 287](https://spec.commonmark.org/0.29/#example-287):

```````````````````````````````` rust
- a
- b

  [ref]: /url
- d
.
unordered_list!(
  vec![paragraph!(plain!("a"))],
  vec![
    paragraph!(plain!("b")),
    linkdef!("ref", "/url"),
  ],
  vec![paragraph!(plain!("d"))]
)
````````````````````````````````

[Example 327](https://spec.commonmark.org/0.29/#example-327)

```````````````````````````````` rust
[a](url &quot;tit&quot;)
.
paragraph!(linkref!(plain!("a")), plain!("(url \"tit\")"))
````````````````````````````````

[Example 485](https://spec.commonmark.org/0.29/#example-485)

```````````````````````````````` rust
[link](/my uri)
.
paragraph!(linkref!(plain!("link")), plain!("(/my uri)"))
````````````````````````````````

[Example 487](https://spec.commonmark.org/0.29/#example-487)

```````````````````````````````` rust
[link](foo
bar)
.
paragraph!(linkref!(plain!("link")), plain!("(foo"), SoftBreak, plain!("bar)"))
````````````````````````````````

[Example 488](https://spec.commonmark.org/0.29/#example-488)

```````````````````````````````` rust
[link](<foo
bar>)
.
paragraph!(
  linkref!(plain!("link")), plain!("("), html!("<foo\nbar>"), plain!(")")
)
````````````````````````````````

[Example 490](https://spec.commonmark.org/0.29/#example-490)

```````````````````````````````` rust
[link](<foo\>)
.
paragraph!(linkref!(plain!("link")), plain!("(<foo>)"))
````````````````````````````````

[Example 491](https://spec.commonmark.org/0.29/#example-491)

```````````````````````````````` rust
[a](<b)c
[a](<b)c>
[a](<b>c)
.
paragraph!(
  linkref!(plain!("a")), plain!("(<b)c"), SoftBreak,
  linkref!(plain!("a")), plain!("(<b)c>"), SoftBreak,
  linkref!(plain!("a")), plain!("("), html!("<b>"), plain!("c)")
)
````````````````````````````````

[Example 499](https://spec.commonmark.org/0.29/#example-499)

```````````````````````````````` rust
[link](foo%20b&auml;)
.
paragraph!(link!("foo%20b\u{e4}", "", plain!("link")))
````````````````````````````````

[Example 504](https://spec.commonmark.org/0.29/#example-504)

```````````````````````````````` rust
[link](/url "title "and" title")
.
paragraph!(linkref!(plain!("link")), plain!("(/url \"title \"and\" title\")"))
````````````````````````````````

[Example 507](https://spec.commonmark.org/0.29/#example-507)

```````````````````````````````` rust
[link] (/uri)
.
paragraph!(linkref!(plain!("link")), plain!(" (/uri)"))
````````````````````````````````

[Example 508](https://spec.commonmark.org/0.29/#example-508):

```````````````````````````````` rust
[link [foo [bar]]](/uri)
.
paragraph!(link!(
  "/uri", "",
  plain!("link "),
  linkref!(plain!("foo "), linkref!(plain!("bar")))
))
````````````````````````````````

[Example 509](https://spec.commonmark.org/0.29/#example-509)

```````````````````````````````` rust
[link] bar](/uri)
.
paragraph!(linkref!(plain!("link")), plain!(" bar](/uri)"))
````````````````````````````````

[Example 516](https://spec.commonmark.org/0.29/#example-516)

```````````````````````````````` rust
![[[foo](uri1)](uri2)](uri3)
.
paragraph!(image!("uri3", "", plain!("["), link!("uri1", "", plain!("foo")), plain!("](uri2)")))
````````````````````````````````

[Example 523](https://spec.commonmark.org/0.29/#example-523)

```````````````````````````````` rust
[foo][bar]

[bar]: /url "title"
.
paragraph!(linkref!("bar", plain!("foo"))),
linkdef!("bar", "/url", "title")
````````````````````````````````

[Example 524](https://spec.commonmark.org/0.29/#example-524)

```````````````````````````````` rust
[link [foo [bar]]][ref]

[ref]: /uri
.
paragraph!(linkref!(
  "ref",
  plain!("link "),
  linkref!(plain!("foo "), linkref!(plain!("bar")))
)),
linkdef!("ref", "/uri")
````````````````````````````````

[Example 525](https://spec.commonmark.org/0.29/#example-525)

```````````````````````````````` rust
[link \[bar][ref]

[ref]: /uri
.
paragraph!(linkref!("ref", plain!("link [bar"))),
linkdef!("ref", "/uri")
````````````````````````````````

[Example 526](https://spec.commonmark.org/0.29/#example-526)

```````````````````````````````` rust
[link *foo **bar** `#`*][ref]

[ref]: /uri
.
paragraph!(linkref!(
  "ref",
  plain!("link "),
  emph!(plain!("foo "), strong!(plain!("bar")), plain!(" "), code!("#"))
)),
linkdef!("ref", "/uri")
````````````````````````````````

[Example 527](https://spec.commonmark.org/0.29/#example-527)

```````````````````````````````` rust
[![moon](moon.jpg)][ref]

[ref]: /uri
.
paragraph!(linkref!("ref", image!("moon.jpg", "", plain!("moon")))),
linkdef!("ref", "/uri")
````````````````````````````````

[Example 528](https://spec.commonmark.org/0.29/#example-528)

```````````````````````````````` rust
[foo [bar](/uri)][ref]

[ref]: /uri
.
paragraph!(
  plain!("[foo "), link!("/uri", "", plain!("bar")), plain!("]"),
  linkref!(plain!("ref"))
),
linkdef!("ref", "/uri")
````````````````````````````````

[Example 529](https://spec.commonmark.org/0.29/#example-529)

```````````````````````````````` rust
[foo *bar [baz][ref]*][ref]

[ref]: /uri
.
paragraph!(
  linkref!(
    "ref",
    plain!("foo "),
    emph!(plain!("bar "), linkref!("ref", plain!("baz")))
  )
),
linkdef!("ref", "/uri")
````````````````````````````````

[Example 530](https://spec.commonmark.org/0.29/#example-530)

```````````````````````````````` rust
*[foo*][ref]

[ref]: /uri
.
paragraph!(plain!("*"), linkref!("ref", plain!("foo*"))),
linkdef!("ref", "/uri")
````````````````````````````````

[Example 531](https://spec.commonmark.org/0.29/#example-531)

```````````````````````````````` rust
[foo *bar][ref]*

[ref]: /uri
.
paragraph!(
  linkref!("ref", plain!("foo *bar")),
  plain!("*")
),
linkdef!("ref", "/uri")
````````````````````````````````

[Example 533](https://spec.commonmark.org/0.29/#example-533)

```````````````````````````````` rust
[foo`][ref]`

[ref]: /uri
.
paragraph!(plain!("[foo"), code!("][ref]")),
linkdef!("ref", "/uri")
````````````````````````````````

[Example 534](https://spec.commonmark.org/0.29/#example-534)

```````````````````````````````` rust
[foo<http://example.com/?search=][ref]>

[ref]: /uri
.
paragraph!(plain!("[foo"),
  link!("http://example.com/?search=][ref]", "",
    plain!("http://example.com/?search=][ref]"))
),
linkdef!("ref", "/uri")
````````````````````````````````

[Example 535](https://spec.commonmark.org/0.29/#example-535)

```````````````````````````````` rust
[foo][BaR]

[bar]: /url "title"
.
paragraph!(linkref!("BaR", plain!("foo"))),
linkdef!("bar", "/url", "title")
````````````````````````````````

[Example 536](https://spec.commonmark.org/0.29/#example-536)

```````````````````````````````` rust
[ẞ]

[SS]: /url
.
paragraph!(linkref!(plain!("ẞ"))),
linkdef!("SS", "/url")
````````````````````````````````

[Example 537](https://spec.commonmark.org/0.29/#example-537)

```````````````````````````````` rust
[Foo
  bar]: /url

[Baz][Foo bar]
.
linkdef!("Foo\n  bar", "/url", ""),
paragraph!(linkref!("Foo bar", plain!("Baz")))
````````````````````````````````

[Example 538](https://spec.commonmark.org/0.29/#example-538)

```````````````````````````````` rust
[foo] [bar]

[bar]: /url "title"
.
paragraph!(linkref!(plain!("foo")), plain!(" "), linkref!(plain!("bar"))),
linkdef!("bar", "/url", "title")
````````````````````````````````

[Example 539](https://spec.commonmark.org/0.29/#example-539)

```````````````````````````````` rust
[foo]
[bar]

[bar]: /url "title"
.
paragraph!(linkref!(plain!("foo")), SoftBreak, linkref!(plain!("bar"))),
linkdef!("bar", "/url", "title")
````````````````````````````````

[Example 540](https://spec.commonmark.org/0.29/#example-540)

```````````````````````````````` rust
[foo]: /url1

[foo]: /url2

[bar][foo]
.
linkdef!("foo", "/url1"),
linkdef!("foo", "/url2"),
paragraph!(linkref!("foo", plain!("bar")))
````````````````````````````````

[Example 541](https://spec.commonmark.org/0.29/#example-541)

```````````````````````````````` rust
[bar][foo\!]

[foo!]: /url
.
paragraph!(linkref!("foo\\!", plain!("bar"))),
linkdef!("foo!", "/url")
````````````````````````````````

[Example 542](https://spec.commonmark.org/0.29/#example-542)

```````````````````````````````` rust
[foo][ref[]

[ref[]: /uri
.
paragraph!(linkref!(plain!("foo")), plain!("[ref[]")),
paragraph!(plain!("[ref[]: /uri"))
````````````````````````````````

[Example 543](https://spec.commonmark.org/0.29/#example-543)

```````````````````````````````` rust
[foo][ref[bar]]

[ref[bar]]: /uri
.
paragraph!(
  linkref!(plain!("foo")),
  linkref!(
    plain!("ref"),
    linkref!(plain!("bar"))
  )
),
paragraph!(
  linkref!(plain!("ref"), linkref!(plain!("bar"))),
  plain!(": /uri")
)
````````````````````````````````

[Example 544](https://spec.commonmark.org/0.29/#example-544):

```````````````````````````````` rust
[[[foo]]]

[[[foo]]]: /url
.
paragraph!(
  linkref!(linkref!(linkref!(plain!("foo"))))
),
paragraph!(
  linkref!(linkref!(linkref!(plain!("foo")))),
  plain!(": /url")
)
````````````````````````````````

[Example 545](https://spec.commonmark.org/0.29/#example-545)

```````````````````````````````` rust
[foo][ref\[]

[ref\[]: /uri
.
paragraph!(linkref!("ref[", plain!("foo"))),
linkdef!("ref[", "/uri")
````````````````````````````````

[Example 546](https://spec.commonmark.org/0.29/#example-546)

```````````````````````````````` rust
[bar\\]: /uri

[bar\\]
.
linkdef!(r"bar\\", "/uri"),
paragraph!(linkref!(plain!("bar\\")))
````````````````````````````````

[Example 549](https://spec.commonmark.org/0.29/#example-549)

```````````````````````````````` rust
[foo][]

[foo]: /url "title"
.
paragraph!(linkref!(plain!("foo"))),
linkdef!("foo", "/url", "title")
````````````````````````````````

[Example 550](https://spec.commonmark.org/0.29/#example-550)

```````````````````````````````` rust
[*foo* bar][]

[*foo* bar]: /url "title"
.
paragraph!(linkref!(emph!(plain!("foo")), plain!(" bar"))),
linkdef!("*foo* bar", "/url", "title")
````````````````````````````````

[Example 551](https://spec.commonmark.org/0.29/#example-551)

```````````````````````````````` rust
[Foo][]

[foo]: /url "title"
.
paragraph!(linkref!(plain!("Foo"))),
linkdef!("foo", "/url", "title")
````````````````````````````````

[Example 552](https://spec.commonmark.org/0.29/#example-552)

```````````````````````````````` rust
[foo] 
[]

[foo]: /url "title"
.
paragraph!(linkref!(plain!("foo")), SoftBreak, plain!("[]")),
linkdef!("foo", "/url", "title")
````````````````````````````````

[Example 553](https://spec.commonmark.org/0.29/#example-553)

```````````````````````````````` rust
[foo]

[foo]: /url "title"
.
paragraph!(linkref!(plain!("foo"))),
linkdef!("foo", "/url", "title")
````````````````````````````````

[Example 554](https://spec.commonmark.org/0.29/#example-554)

```````````````````````````````` rust
[*foo* bar]

[*foo* bar]: /url "title"
.
paragraph!(linkref!(emph!(plain!("foo")), plain!(" bar"))),
linkdef!("*foo* bar", "/url", "title")
````````````````````````````````

[Example 555](https://spec.commonmark.org/0.29/#example-555)

```````````````````````````````` rust
[[*foo* bar]]

[*foo* bar]: /url "title"
.
paragraph!(
  linkref!(
    linkref!(emph!(plain!("foo")), plain!(" bar"))
  )
),
linkdef!("*foo* bar", "/url", "title")
````````````````````````````````

[Example 556](https://spec.commonmark.org/0.29/#example-556)

```````````````````````````````` rust
[[bar [foo]

[foo]: /url
.
paragraph!(
  plain!("[[bar "), linkref!(plain!("foo"))
),
linkdef!("foo", "/url")
````````````````````````````````

[Example 557](https://spec.commonmark.org/0.29/#example-557)

```````````````````````````````` rust
[Foo]

[foo]: /url "title"
.
paragraph!(linkref!(plain!("Foo"))),
linkdef!("foo", "/url", "title")
````````````````````````````````

[Example 558](https://spec.commonmark.org/0.29/#example-558)

```````````````````````````````` rust
[foo] bar

[foo]: /url
.
paragraph!(linkref!(plain!("foo")), plain!(" bar")),
linkdef!("foo", "/url")
````````````````````````````````

[Example 559](https://spec.commonmark.org/0.29/#example-559)

```````````````````````````````` rust
\[foo]

[foo]: /url "title"
.
paragraph!(plain!("[foo]")),
linkdef!("foo", "/url", "title")
````````````````````````````````

[Example 560](https://spec.commonmark.org/0.29/#example-560)

That's the opposite of
[Example 519](https://spec.commonmark.org/0.29/#example-519) and can't be
decided during parsing. If the user really want a reference, he should write
`[foo*][]`. See also Example 163.

```````````````````````````````` rust
[foo*]: /url

*[foo*]
.
linkdef!("foo*", "/url"),
paragraph!(emph!(plain!("[foo")), plain!("]"))
````````````````````````````````

[Example 561](https://spec.commonmark.org/0.29/#example-561)

```````````````````````````````` rust
[foo][bar]

[foo]: /url1
[bar]: /url2
.
paragraph!(linkref!("bar", plain!("foo"))),
linkdef!("foo", "/url1"),
linkdef!("bar", "/url2")
````````````````````````````````

[Example 562](https://spec.commonmark.org/0.29/#example-562)

```````````````````````````````` rust
[foo][]

[foo]: /url1
.
paragraph!(linkref!(plain!("foo"))),
linkdef!("foo", "/url1")
````````````````````````````````

[Example 563](https://spec.commonmark.org/0.29/#example-563)

```````````````````````````````` rust
[foo]()

[foo]: /url1
.
paragraph!(link!("", "", plain!("foo"))),
linkdef!("foo", "/url1")
````````````````````````````````

[Example 564](https://spec.commonmark.org/0.29/#example-564)

```````````````````````````````` rust
[foo](not a link)

[foo]: /url1
.
paragraph!(linkref!(plain!("foo")), plain!("(not a link)")),
linkdef!("foo", "/url1")
````````````````````````````````

[Example 565](https://spec.commonmark.org/0.29/#example-565)

```````````````````````````````` rust
[foo][bar][baz]

[baz]: /url
.
// This is a violation of the spec, because the parser doesn't know which
// labels are valid while parsing. The spec expects
// `linkref!(plain!("foo")), linkref!("baz", plain!("bar"))`
paragraph!(linkref!("bar", plain!("foo")), linkref!(plain!("baz"))),
linkdef!("baz", "/url")
````````````````````````````````

[Example 566](https://spec.commonmark.org/0.29/#example-566)

```````````````````````````````` rust
[foo][bar][baz]

[baz]: /url1
[bar]: /url2
.
paragraph!(linkref!("bar", plain!("foo")), linkref!(plain!("baz"))),
linkdef!("baz", "/url1"),
linkdef!("bar", "/url2")
````````````````````````````````

[Example 567](https://spec.commonmark.org/0.29/#example-567)

```````````````````````````````` rust
[foo][bar][baz]

[baz]: /url1
[foo]: /url2
.
// This is a violation of the spec, because we don't know which labels are
// valid during parsing
paragraph!(linkref!("bar", plain!("foo")), linkref!(plain!("baz"))),
linkdef!("baz", "/url1"),
linkdef!("foo", "/url2")
````````````````````````````````

[Example 569](https://spec.commonmark.org/0.29/#example-569)

```````````````````````````````` rust
![foo *bar*]

[foo *bar*]: train.jpg "train & tracks"
.
paragraph!(imageref!(plain!("foo "), emph!(plain!("bar")))),
linkdef!("foo *bar*", "train.jpg", "train & tracks")
````````````````````````````````

[Example 570](https://spec.commonmark.org/0.29/#example-570)

```````````````````````````````` rust
![foo ![bar](/url)](/url2)
.
paragraph!(image!("/url2", "", plain!("foo "), image!("/url", "", plain!("bar"))))
````````````````````````````````

[Example 571](https://spec.commonmark.org/0.29/#example-571)

```````````````````````````````` rust
![foo [bar](/url)](/url2)
.
paragraph!(image!("/url2", "", plain!("foo "), link!("/url", "", plain!("bar"))))
````````````````````````````````

[Example 572](https://spec.commonmark.org/0.29/#example-572)

```````````````````````````````` rust
![foo *bar*][]

[foo *bar*]: train.jpg "train & tracks"
.
paragraph!(imageref!(plain!("foo "), emph!(plain!("bar")))),
linkdef!("foo *bar*", "train.jpg", "train & tracks")
````````````````````````````````

[Example 573](https://spec.commonmark.org/0.29/#example-573)

```````````````````````````````` rust
![foo *bar*][foobar]

[FOOBAR]: train.jpg "train & tracks"
.
paragraph!(imageref!("foobar", plain!("foo "), emph!(plain!("bar")))),
linkdef!("FOOBAR", "train.jpg", "train & tracks")
````````````````````````````````

[Example 578](https://spec.commonmark.org/0.29/#example-578)

```````````````````````````````` rust
![foo][bar]

[bar]: /url
.
paragraph!(imageref!("bar", plain!("foo"))),
linkdef!("bar", "/url")
````````````````````````````````

[Example 579](https://spec.commonmark.org/0.29/#example-579)

```````````````````````````````` rust
![foo][bar]

[BAR]: /url
.
paragraph!(imageref!("bar", plain!("foo"))),
linkdef!("BAR", "/url")
````````````````````````````````

[Example 580](https://spec.commonmark.org/0.29/#example-580)

```````````````````````````````` rust
![foo][]

[foo]: /url "title"
.
paragraph!(imageref!(plain!("foo"))),
linkdef!("foo", "/url", "title")
````````````````````````````````

[Example 581](https://spec.commonmark.org/0.29/#example-581)

```````````````````````````````` rust
![*foo* bar][]

[*foo* bar]: /url "title"
.
paragraph!(imageref!(emph!(plain!("foo")), plain!(" bar"))),
linkdef!("*foo* bar", "/url", "title")
````````````````````````````````

[Example 582](https://spec.commonmark.org/0.29/#example-582)

```````````````````````````````` rust
![Foo][]

[foo]: /url "title"
.
paragraph!(imageref!(plain!("Foo"))),
linkdef!("foo", "/url", "title")
````````````````````````````````

[Example 583](https://spec.commonmark.org/0.29/#example-583)

```````````````````````````````` rust
![foo] 
[]

[foo]: /url "title"
.
paragraph!(imageref!(plain!("foo")), SoftBreak, plain!("[]")),
linkdef!("foo", "/url", "title")
````````````````````````````````

[Example 584](https://spec.commonmark.org/0.29/#example-584)

```````````````````````````````` rust
![foo]

[foo]: /url "title"
.
paragraph!(imageref!(plain!("foo"))),
linkdef!("foo", "/url", "title")
````````````````````````````````

[Example 585](https://spec.commonmark.org/0.29/#example-585)

```````````````````````````````` rust
![*foo* bar]

[*foo* bar]: /url "title"
.
paragraph!(imageref!(emph!(plain!("foo")), plain!(" bar"))),
linkdef!("*foo* bar", "/url", "title")
````````````````````````````````

[Example 586](https://spec.commonmark.org/0.29/#example-586)

```````````````````````````````` rust
![[foo]]

[[foo]]: /url "title"
.
paragraph!(imageref!(linkref!(plain!("foo")))),
paragraph!(
  linkref!(linkref!(plain!("foo"))),
  plain!(": /url \"title\"")
)
````````````````````````````````

[Example 587](https://spec.commonmark.org/0.29/#example-587)

```````````````````````````````` rust
![Foo]

[foo]: /url "title"
.
paragraph!(imageref!(plain!("Foo"))),
linkdef!("foo", "/url", "title")
````````````````````````````````

[Example 588](https://spec.commonmark.org/0.29/#example-588)

```````````````````````````````` rust
!\[foo]

[foo]: /url "title"
.
paragraph!(plain!("![foo]")),
linkdef!("foo", "/url", "title")
````````````````````````````````

[Example 589](https://spec.commonmark.org/0.29/#example-589)

```````````````````````````````` rust
\![foo]

[foo]: /url "title"
.
paragraph!(plain!("!"), linkref!(plain!("foo"))),
linkdef!("foo", "/url", "title")
````````````````````````````````

## Inline HTML

[Example 609](https://spec.commonmark.org/0.29/#example-609)

```````````````````````````````` rust
<a><bab><c2c>
.
paragraph!(html!("<a>"), html!("<bab>"), html!("<c2c>"))
````````````````````````````````

[Example 610](https://spec.commonmark.org/0.29/#example-610)

```````````````````````````````` rust
<a/><b2/>
.
paragraph!(html!("<a/>"), html!("<b2/>"))
````````````````````````````````

[Example 611](https://spec.commonmark.org/0.29/#example-611)

```````````````````````````````` rust
<a  /><b2
data="foo" >
.
paragraph!(html!("<a  />"), html!("<b2\ndata=\"foo\" >"))
````````````````````````````````

[Example 612](https://spec.commonmark.org/0.29/#example-612)

```````````````````````````````` rust
<a foo="bar" bam = 'baz <em>"</em>'
_boolean zoop:33=zoop:33 />
.
paragraph!(html!("<a foo=\"bar\" bam = 'baz <em>\"</em>'\n_boolean zoop:33=zoop:33 />"))
````````````````````````````````

[Example 613](https://spec.commonmark.org/0.29/#example-613)

```````````````````````````````` rust
Foo <responsive-image src="foo.jpg" />
.
paragraph!(plain!("Foo "), html!("<responsive-image src=\"foo.jpg\" />"))
````````````````````````````````

[Example 619](https://spec.commonmark.org/0.29/#example-619)

```````````````````````````````` rust
</a></foo >
.
paragraph!(html!("</a>"), html!("</foo >"))
````````````````````````````````

[Example 625](https://spec.commonmark.org/0.29/#example-625)

```````````````````````````````` rust
foo <!ELEMENT br EMPTY>
.
paragraph!(plain!("foo "), html!("<!ELEMENT br EMPTY>"))
````````````````````````````````

[Example 626](https://spec.commonmark.org/0.29/#example-626)

```````````````````````````````` rust
foo <![CDATA[>&<]]>
.
paragraph!(plain!("foo "), html!("<![CDATA[>&<]]>"))
````````````````````````````````

[Example 627](https://spec.commonmark.org/0.29/#example-627)

```````````````````````````````` rust
foo <a href="&ouml;">
.
paragraph!(plain!("foo "), html!(r#"<a href="&ouml;">"#))
````````````````````````````````

[Example 628](https://spec.commonmark.org/0.29/#example-628)

```````````````````````````````` rust
foo <a href="\*">
.
paragraph!(plain!("foo "), html!(r#"<a href="\*">"#))
````````````````````````````````

[Example 639](https://spec.commonmark.org/0.29/#example-639)

```````````````````````````````` rust
<a href="foo  
bar">
.
paragraph!(html!("<a href=\"foo  \nbar\">"))
````````````````````````````````

[Example 640](https://spec.commonmark.org/0.29/#example-640)

```````````````````````````````` rust
<a href="foo\
bar">
.
paragraph!(html!("<a href=\"foo\\\nbar\">"))
````````````````````````````````

# Lists

## Indentation

[Example 227](https://spec.commonmark.org/0.29/#example-227):

```````````````````````````````` replacement
 -    one

     two
.
<ul>
<li>
<p>one</p>
<p>two</p>
</li>
</ul>
````````````````````````````````

[Example 238](https://spec.commonmark.org/0.29/#example-238):

The parser stores the whole start number and doesn't remove leading zeros.

```````````````````````````````` replacement
003. ok
.
<ol start="003">
<li>ok</li>
</ol>
````````````````````````````````
