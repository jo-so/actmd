use super::{
    Block,
    Inline,
};

macro_rules! str_arg {
    ($name:ident, $id:expr) => (
        #[macro_export]
        macro_rules! $name {
            ($s:literal) => ( $id($s.to_string()) )
        }
    )
}

#[cfg(not(feature = "location"))]
macro_rules! str_arg_loc {
    ($name:ident, $id:expr) => (
        str_arg_loc!(($) $name, $id);
    );

    (($dollar:tt) $name:ident, $id:expr) => (
        #[macro_export]
        macro_rules! $name {
            (
                $dollar( <$begin:literal, $end:literal> )? $s:literal
            ) => (
                $id($s.to_string(), Location::default())
            );
        }
    );
}

#[cfg(feature = "location")]
macro_rules! str_arg_loc {
    ($name:ident, $id:expr) => (
        #[macro_export]
        macro_rules! $name {
            (
                <$begin:literal, $end:literal> $s:literal
            ) => (
                $id($s.to_string(), Location { begin: $begin.into(), end: $end.into() })
            )
        }
    )
}

macro_rules! vec_arg {
    ( ($dollar:tt), $name:ident, $id:expr) => {
        #[macro_export]
        macro_rules! $name {
            ( $dollar($args:expr),* ) => ( $id(vec![ $dollar( $args ),* ] ) )
        }
    };

    ($name:ident, $id:expr) => ( vec_arg! { ($) , $name, $id } );
}

#[cfg(not(feature = "location"))]
macro_rules! vec_arg_loc {
    ( ($dollar:tt), $name:ident, $id:expr) => {
        #[macro_export]
        macro_rules! $name {
            ( ) => ( $id(vec![], Location::default() ) );
            ( <$begin:literal, $end:literal> ) => ( $id(vec![], Location::default() ) );

            ( <$begin:literal, $end:literal> $dollar($args:expr),+ ) => (
                $id(vec![ $dollar( $args ),+ ], Location::default() )
            );

            ( $dollar($args:expr),+ ) => (
                $id(vec![ $dollar( $args ),+ ], Location::default() )
            );
        }
    };

    ($name:ident, $id:expr) => ( vec_arg_loc! { ($) , $name, $id } );
}

#[cfg(feature = "location")]
macro_rules! vec_arg_loc {
    ( ($dollar:tt), $name:ident, $id:expr) => {
        #[macro_export]
        macro_rules! $name {
            ( <$begin:literal, $end:literal> $dollar($args:expr),* ) => (
                $id(vec![ $dollar( $args ),* ], Location { begin: $begin.into(), end: $end.into() } )
            )
        }
    };

    ($name:ident, $id:expr) => ( vec_arg_loc! { ($) , $name, $id } );
}

////
//
// Helper macros for block level
//
pub use crate::block as b;
pub use Block::Break;

vec_arg_loc!(paragraph, Block::Paragraph);
str_arg_loc!(html_block, Block::Html);

str_arg_loc!(emb_block_block, Block::EmbeddedBlock);
str_arg_loc!(emb_expr_block, Block::EmbeddedExpr);

#[macro_export]
#[cfg(not(feature = "location"))]
macro_rules! heading {
    (
        $( <$begin:literal, $end:literal> )? $level:literal $(,)? $($args:expr),*
    ) => (
        Block::Heading($level, vec![ $( $args ),* ], Location::default())
    );
}

#[macro_export]
#[cfg(feature = "location")]
macro_rules! heading {
    (
        <$begin:literal, $end:literal> $level:literal, $($args:expr),*
    ) => (
        Block::Heading(
            $level, vec![ $( $args ),* ],
            Location { begin: $begin.into(), end: $end.into() }
        )
    );
}

vec_arg_loc!(quote, Block::Quote);

#[macro_export]
#[cfg(not(feature = "location"))]
macro_rules! code_block {
    ($( <$begin:literal, $end:literal> )? $opt:literal) => ( code_block!($opt, "") );

    (
        $( <$begin:literal, $end:literal> )? $opt:literal, $ct:literal
    ) => (
        Block::Code($opt.to_string(), $ct.to_string(), Location::default())
    )
}

#[macro_export]
#[cfg(feature = "location")]
macro_rules! code_block {
    (
        <$begin:literal, $end:literal> $opt:literal,
    ) => (
        code_block!(($begin, $end), $opt, "")
    );

    (
        <$begin:literal, $end:literal> $opt:literal, $ct:literal
    ) => (
        Block::Code($opt.to_string(), $ct.to_string(), Location { begin: $begin.into(), end: $end.into() })
    )
}

#[macro_export]
macro_rules! ordered_list {
    (
        $start:literal, $($args:expr),*
    ) => (
        Block::OrderedList($start.to_string(), vec![ $( $args ),* ])
    );

    (
        $($args:expr),*
    ) => (
        Block::OrderedList("1".to_string(), vec![ $( $args ),* ])
    )
}

#[macro_export]
macro_rules! unordered_list {
    ( $($args:expr),* ) => ( Block::UnorderedList( vec![ $( $args ),* ] ) )
}

/// linkdef!(label, url, title)
#[macro_export]
macro_rules! linkdef {
    (
        $label:literal, $url:literal
    ) => (
        Block::LinkDef($label.to_string(), $url.to_string(), String::with_capacity(0))
    );

    (
        $label:literal, $url:literal, $title:literal
    ) => (
        Block::LinkDef($label.to_string(), $url.to_string(), $title.to_string())
    )
}

////
//
// Helper macros for inline level
//
pub use Inline::{HardBreak, SoftBreak};
str_arg_loc!(plain, Inline::Plain);
str_arg!(code, Inline::Code);
str_arg_loc!(html, Inline::Html);
vec_arg!(emph, Inline::Emph);
vec_arg!(strong, Inline::Strong);

str_arg_loc!(emb_block, Inline::EmbeddedBlock);
str_arg_loc!(emb_expr, Inline::EmbeddedExpr);

/// link!(url, title, inner_content...)
#[macro_export]
macro_rules! link {
    (
        $url:literal, $title:literal
    ) => (
        link!($url, $title, )
    );

    (
        $url:literal, $title:literal, $($args:expr),*
    ) => (
        Inline::Link(vec![ $( $args ),* ], $url.to_string(), $title.to_string())
    )
}

/// linkref!([label,] text...)
#[macro_export]
macro_rules! linkref {
    (
        $label:literal, $($text:expr),*
    ) => (
        Inline::LinkRef(vec![ $( $text ),* ], $label.to_string())
    );

    (
        $($text:expr),*
    ) => (
        Inline::LinkRef(vec![ $( $text ),* ], String::with_capacity(0))
    )
}

/// image!(url, title, inner_content...)
#[macro_export]
macro_rules! image {
    (
        $url:literal, $title:literal
    ) => (
        image!($url, $title, )
    );

    (
        $url:literal, $title:literal, $($args:expr),*
    ) => (
        Inline::Image(vec![ $( $args ),* ], $url.to_string(), $title.to_string())
    )
}

/// imageref!([label,] text...)
#[macro_export]
macro_rules! imageref {
    (
        $label:literal, $($text:expr),*
    ) => (
        Inline::ImageRef(vec![ $( $text ),* ], $label.to_string())
    );

    (
        $($text:expr),*
    ) => (
        Inline::ImageRef(vec![ $( $text ),* ], String::with_capacity(0))
    )
}
