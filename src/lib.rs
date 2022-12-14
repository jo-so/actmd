#![cfg_attr(test, feature(assert_matches))]

use std::{
    convert::{Infallible, TryFrom},
    fmt::{self, Display},
    fs,
    io,
    ops::{Add, Deref, Sub},
    path::Path,
    str::FromStr,
};

mod block;
pub use block::block;

mod paragraph;
pub use paragraph::paragraph;

mod string;
pub use string::StringData;

#[cfg(test)]
mod tests;

pub mod test_utils;

mod tools;
pub use tools::{
    Tools,
    Pattern,
};

mod trans;
pub use trans::Transaction;

#[cfg(feature = "log")]
#[macro_export]
macro_rules! log {
    ( call t $( $args:expr ),* ) => ( log::trace!( $( $args ),* ); );
    ( call d $( $args:expr ),* ) => ( log::debug!( $( $args ),* ); );
    ( call i $( $args:expr ),* ) => ( log::info!( $( $args ),* ); );
    ( call w $( $args:expr ),* ) => ( log::warn!( $( $args ),* ); );
    ( call e $( $args:expr ),* ) => ( log::error!( $( $args ),* ); );

    (
        $level:tt, $data:expr, $fn:literal, $msg:literal $( $args:tt )*
    ) => (
        $crate::log!(
            call $level
                concat!($fn, " > ", $msg, " @ ({:?}, {:?})")
                $( $args )*
                , $data.pos() , $data.peek()
        );
    );

    (
        $level:tt, $fn:literal, $msg:literal $( $args:tt )*
    ) => (
        $crate::log!(
            call $level
                concat!($fn, " > ", $msg) $( $args )*
        );
    );
}

#[cfg(not(feature = "log"))]
#[macro_export]
macro_rules! log {
    ($( $junk:tt )* ) => ();
}

/// Whitespace within a line
pub static LINE_WS : [char; 2] = [' ', '\t'];
/// New line `\n`, Carriage return `\r`
pub static NL_CR : [char; 2] = ['\n', '\r'];

pub type Position = usize;

mod ps_help {
    #![allow(non_upper_case_globals)]
    use bitflags::bitflags;

    bitflags! {
        pub struct ParserSettings: u32 {
            const None = 0;
            /// Allow embedded code syntax `@…`
            const Embedded = 1 << 0;
            /// Parse HTML, otherwise it's plain text
            const Html = 1 << 1;
            // /// Deletions ~abc~
            // const Del = 1 << 4;
            // /// Tables |…|…|
            // const Tables = 1 << 6;
            // /// Allow incomplete input at end
            // ///
            // /// `text *text` gets parsed as `Plain(text ) Emph(Plain(text))`
            // /// and `<a href="` get valid `Html`. This makes it easier for syntax
            // /// highlighting in editors.
            // const IncompleteEnd = 1 << 7,
            // /// Checkboxes * [X], Input [___]
            // const Checkboxes = 1 << 8,
        }
    }

    impl Default for ParserSettings {
        fn default() -> Self {
            ParserSettings::Html |
            ParserSettings::Embedded
        }
    }

    impl ParserSettings {
        #[must_use]
        pub fn common_mark() -> Self {
            ParserSettings::Html
        }
    }
}

pub use ps_help::ParserSettings;

pub trait ParserData {
    fn settings(&self) -> ParserSettings;

    fn peek(&self) -> Option<char>;
    fn advance(&mut self);
    fn pos(&self) -> Position;
    fn reset(&mut self, pos: Position) -> Result<(), ()>;
}

impl ParserData for &mut dyn ParserData {
    fn settings(&self) -> ParserSettings {
        (**self).settings()
    }

    fn peek(&self) -> Option<char> {
        (**self).peek()
    }

    fn advance(&mut self) {
        (**self).advance();
    }

    fn pos(&self) -> Position {
        (**self).pos()
    }

    fn reset(&mut self, pos: Position) -> Result<(), ()> {
        (**self).reset(pos)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(not(feature = "location"), derive(Default))]
pub struct LocationPosition(
    #[cfg(feature = "location")]
    pub Position
);

// TODO: entfernen
#[cfg(feature = "location")]
impl Default for LocationPosition {
    fn default() -> Self {
        LocationPosition(0)
    }
}

impl Add<usize> for LocationPosition {
    type Output = Self;

    #[cfg(feature = "location")]
    fn add(self, other: usize) -> Self::Output {
        LocationPosition(self.0 + other)
    }

    #[cfg(not(feature = "location"))]
    fn add(self, _other: usize) -> Self::Output {
        LocationPosition::default()
    }
}

impl Sub<usize> for LocationPosition {
    type Output = Self;

    #[cfg(feature = "location")]
    fn sub(self, other: usize) -> Self::Output {
        // TODO: LocationPosition(self.0 - other)
        LocationPosition(self.0.saturating_sub(other))
    }

    #[cfg(not(feature = "location"))]
    fn sub(self, _other: usize) -> Self::Output {
        LocationPosition::default()
    }
}

impl Sub<LocationPosition> for LocationPosition {
    type Output = Self;

    #[cfg(feature = "location")]
    fn sub(self, other: LocationPosition) -> Self::Output {
        // TODO: LocationPosition(self.0 - other.0)
        LocationPosition(self.0.saturating_sub(other.0))
    }

    #[cfg(not(feature = "location"))]
    fn sub(self, _other: LocationPosition) -> Self::Output {
        LocationPosition::default()
    }
}

impl LocationPosition {
    #[cfg(feature = "location")]
    fn from<T: ParserData>(data: &T) -> Self {
        Self(data.pos())
    }

    #[cfg(not(feature = "location"))]
    fn from<T: ParserData>(_data: &T) -> Self {
        Self::default()
    }
}

impl From<usize> for LocationPosition {
    #[cfg(feature = "location")]
    fn from(loc: usize) -> Self {
        Self(loc)
    }

    #[cfg(not(feature = "location"))]
    fn from(_loc: usize) -> Self {
        Self()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(not(feature = "location"), derive(Default))]
pub struct Location {
    pub begin: LocationPosition,
    pub end: LocationPosition,
}

#[cfg(feature = "location")]
impl Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}..{}", self.begin.0, self.end.0)
    }
}

trait LocationHelper {
    fn loc(&self) -> LocationPosition;
    fn loc_end(&self, loc_begin: LocationPosition) -> Location;
}

impl<T: ParserData> LocationHelper for T {
    fn loc(&self) -> LocationPosition {
        LocationPosition::from(self)
    }

    fn loc_end(&self, begin: LocationPosition) -> Location {
        Location {
            begin,
            end: LocationPosition::from(self),
        }
    }
}

/// Top level elements of a document
#[derive(Debug, PartialEq)]
pub enum Block {
    /// Headline `# ...` (`level, content, location`)
    Heading(u8, Vec<Inline>, Location),

    /// Paragraph (`content, location`)
    Paragraph(Vec<Inline>, Location),

    /// Quotation `> ...` (`content, location`)
    Quote(Vec<Block>, Location),

    /// Fenced code block ```` ```info... ```` (`info, content, location`)
    Code(String, String, Location),

    /// Ordered list `1. ...` (`start, content`)
    OrderedList(String, Vec<Vec<Block>>),

    /// Unordered list `* ...` (`content`)
    UnorderedList(Vec<Vec<Block>>),

    /// HTML section (`content, location`)
    Html(String, Location),

    /// Thematic break `* * *`
    ///
    /// In HTML rendered as `<hr/>`
    Break,

    /// Link definition (`label, url, title, location`)
    ///
    /// Used to resolve [`Inline::ImageRef`] and [`Inline::LinkRef`]
    LinkDef(String, String, String, Location),

    /// Embedded code block `@{...}`
    ///
    /// This block contains a block of statements that should be evaluated on
    /// output generation.
    EmbeddedBlock(String, Location),

    /// Embedded expression `@(...)`
    ///
    /// The expression returns a value that should be included in the output.
    EmbeddedExpr(String, Location),
}

#[derive(Debug, PartialEq)]
pub enum Inline {
    /// Plain text
    Text(String, Location),

    /// HTML tag `<...>` in a paragraph
    Html(String, Location),

    /// Code segment `` `...` `` in a paragraph
    Code(String, Location),

    /// Common break between lines
    ///
    /// This should not influence the rendering. See also [HardBreak]
    SoftBreak,

    /// A special marked linebreak `...\`
    ///
    /// This should create an visual linebreak in the output.
    HardBreak,

    /// Emphasized section
    Emph(Vec<Inline>),

    /// Stronger emphasized section
    Strong(Vec<Inline>),

    /// `Image(description/alt text, src url, title)`
    ///
    /// * `![1](2 "3")` => `Image(1, 2, 3)`
    /// * `![1](2)` => `Image(1, 2, "")`
    Image(Vec<Inline>, String, String, Location),

    /// `ImageRef(description, label)` (must be resolved with Block::LinkDef)
    ///
    /// * `![1][2]` => `ImageRef(1, 2)`
    /// * `![1][]` => `ImageRef(1, "")`
    /// * `![1]` => `ImageRef(1, "")`
    ImageRef(Vec<Inline>, String, Location),

    /// `Link(link text, url, title)`
    ///
    /// * `[1](2 "3")` => `Link(1, 2, 3)`
    /// * `[1](2)` => `Link(1, 2, "")`
    Link(Vec<Inline>, String, String, Location),

    /// `LinkRef(text, label)` (must be resolved with Block::LinkDef)
    ///
    /// * `[1][2]` => `LinkRef(1, 2)`
    /// * `[1][]` => `LinkRef(1, "")`
    /// * `[1]` => `LinkRef(1, "")`
    LinkRef(Vec<Inline>, String, Location),

    /// Embedded code block `@{...}`
    ///
    /// This block contains a block of statements that should be evaluated on
    /// output generation.
    EmbeddedBlock(String, Location),

    /// Embedded expression `@(...)`
    ///
    /// The expression returns a value that should be included in the output.
    EmbeddedExpr(String, Location),
}

/// String with all ASCII characters in lowercase
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Lcstr(Box<str>);

impl From<&str> for Lcstr {
    fn from(src: &str) -> Self {
        Self(src.to_ascii_lowercase().into())
    }
}

impl From<String> for Lcstr {
    fn from(mut src: String) -> Self {
        src.make_ascii_lowercase();
        Self(src.into())
    }
}

impl FromStr for Lcstr {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}


impl Deref for Lcstr {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for Lcstr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

pub type Head = Vec<(Lcstr, Box<str>)>;

#[derive(Default)]
pub struct Document {
    src: Box<str>,
    head: Head,
    body: Vec<Block>,
}

impl Document {
    #[must_use]
    pub fn parse(src: impl Into<Box<str>>, data: &mut impl ParserData) -> Self {
        Self {
            src: src.into(),
            head: head(data),
            body: body(data),
        }
    }

    pub fn src(&self) -> &str {
        &self.src
    }

    pub fn head(&self) -> &[(Lcstr, Box<str>)] {
        self.head.as_slice()
    }

    pub fn head_mut(&mut self) -> &mut Head {
        &mut self.head
    }

    pub fn add_header(&mut self, key: impl Into<Lcstr>, value: impl Into<Box<str>>) {
        self.head.push((key.into(), value.into()));
    }

    pub fn last_head_val(&self, key: impl Into<Lcstr>) -> Option<&str> {
        let key = key.into();

        self.head.iter()
            .rfind(|(k, _)| *k == key)
            .map(|(_, v)| &**v)
    }

    pub fn body(&self) -> &Vec<Block> {
        &self.body
    }

    pub fn body_mut(&mut self) -> &mut Vec<Block> {
        &mut self.body
    }
}

impl FromStr for Document {
    type Err = Infallible;

    fn from_str(inp: &str) -> Result<Self, Self::Err> {
        Ok(Self::parse("(from str)", &mut StringData::from(inp)))
    }
}

impl TryFrom<&Path> for Document {
    type Error = io::Error;

    fn try_from(path: &Path) -> Result<Self, Self::Error> {
        Ok(Self::parse(
            path.to_string_lossy(),
            &mut StringData::new(
                &fs::read_to_string(path)?,
                Default::default(),
            ),
        ))
    }
}

pub fn head(data: &mut impl ParserData) -> Head {
    let mut header = Vec::new();
    head_in(data, &mut header);
    header
}

pub fn head_in(data: &mut impl ParserData, header: &mut Head) {
    fn not_newline(c: char) -> bool {
        c != '\r' && c != '\n'
    }

    loop {
        if data.skip('/') && data.skip('/') {
            data.skip_all(not_newline);

            if data.skip_newline() {
                continue;
            }

            break;
        }

        if !data.looking_at(char::is_alphanumeric) {
            break;
        }

        let mut key = String::new();

        data.copy_all(&mut key, |c: char| c.is_alphanumeric() || "_.-".contains(c));

        if data.skip(':') && !data.looking_at(" \n\r\t") {
            break;
        }

        data.skip_all(LINE_WS);

        if data.skip_newline() && data.skip_all(LINE_WS) == 0 {
            key.make_ascii_lowercase();
            header.push((key.into(), "".into()));
            continue;
        }

        let mut val = String::new();

        loop {
            data.copy_all(&mut val, not_newline);

            if !(data.skip_newline() && data.skip(LINE_WS)) {
                break;
            }

            val.push('\n');
        }

        key.make_ascii_lowercase();
        header.push((key.into(), val.into_boxed_str()));
    }
}

pub fn body(data: &mut impl ParserData) -> Vec<Block> {
    let mut open_embedded_codes = 0;
    let mut blocks = Vec::new();

    while data.peek().is_some() {
        block::block(data, &mut blocks, &mut open_embedded_codes);
    }

    // TODO: verbessern
    assert_eq!(0, open_embedded_codes);

    blocks
}

const NAMED_HTML_ENTITY : &[(&str, &str)] = &include!("html_entities.rs");

fn html_entity(data: &mut impl ParserData, buf: &mut String) {
    if !data.skip('&') {
        return;
    }

    log!(d, data, "html entity", "begin");
    let buf_start_len = buf.len();
    buf.push('&');

    let peek = if let Some(x) = data.peek() {
        x
    } else {
        return;
    };

    if peek == '#' {
        buf.push('#');

        match data.next() {
            Some(c @ ('x' | 'X')) => {
                buf.push(c);

                while let Some(c) = data.next() {
                    if c.is_ascii_hexdigit() && buf.len() - buf_start_len < 8 {
                        buf.push(c);
                    } else if c == ';' {
                        data.advance();

                        let hex_char = u32::from_str_radix(&buf[buf_start_len + 3..], 16).ok()
                            .and_then(|x| char::try_from(x).ok());

                        let ch = if let Some(x) = hex_char {
                            buf.truncate(buf_start_len);
                            if x == '\0' { '\u{FFFD}' } else { x }
                        } else {
                            ';'
                        };

                        buf.push(ch);

                        break;
                    } else {
                        break;
                    }
                }
            }

            Some(c) if c.is_ascii_digit() => {
                buf.push(c);

                while let Some(c) = data.next() {
                    if c.is_ascii_digit() && buf.len() - buf_start_len < 9 {
                        buf.push(c);
                    } else if c == ';' {
                        data.advance();

                        let ch = buf[buf_start_len + 2..].parse::<u32>().ok()
                            .and_then(|x| char::try_from(x).ok());

                        let ch = if let Some(x) = ch {
                            buf.truncate(buf_start_len);
                            if x == '\0' { '\u{FFFD}' } else { x }
                        } else {
                            ';'
                        };

                        buf.push(ch);

                        break;
                    } else {
                        break;
                    }
                }
            }

            _ => (),
        }
    } else if peek.is_ascii_alphabetic() {
        buf.push(peek);

        while let Some(c) = data.next() {
            if c.is_ascii_alphanumeric() {
                buf.push(c);
            } else if c == ';' {
                data.advance();

                if let Ok(idx) = NAMED_HTML_ENTITY
                    .binary_search_by_key(&&buf[buf_start_len + 1..], |e| e.0)
                {
                    buf.truncate(buf_start_len);
                    buf.push_str(NAMED_HTML_ENTITY[idx].1);
                } else {
                    buf.push(';');
                }

                break;
            } else {
                break;
            }
        }
    }

    log!(d, data, "html entity", "end");
}

fn is_ascii_alphabetic(ch: char) -> bool {
    char::is_ascii_alphabetic(&ch)
}

fn is_ascii_alphanumeric(ch: char) -> bool {
    char::is_ascii_alphanumeric(&ch)
}
