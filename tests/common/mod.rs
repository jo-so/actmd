pub use actmd::*;
pub use actmd::test_utils::*;

pub use std::assert_matches::assert_matches;

pub fn init_logger() {
    let mut builder = env_logger::builder();

    if std::env::var_os("CAPTURE_LOG").is_some() {
        builder.is_test(true);
    }

    let _ = builder.format(|buf, rec| {
        use env_logger::fmt::Color;
        use log::Level;
        use std::io::Write;

        let mut style = buf.style();
        match rec.level() {
            Level::Trace => style.set_color(Color::Magenta),
            Level::Debug => style.set_color(Color::Blue),
            Level::Info => style.set_color(Color::Green),
            Level::Warn => style.set_color(Color::Yellow),
            Level::Error => style.set_color(Color::Red),
        };

        let file = rec.file().unwrap_or("---");
        let width = 32 - file.len();

        writeln!(
            buf, " {level:5} {file}:{line:<width$} {arrow} {args}",
            level = style.value(rec.level()),
            line = rec.line().unwrap_or(0),
            arrow = style.value(">>"),
            args = rec.args()
        )
    }).try_init();
}

pub macro init {
    ($inp:expr) => (
        if std::env::var("RUST_LOG").is_ok() {
            crate::common::init_logger();
            log::info!(r#"Parsing "{}""#, $inp.escape_default());
        }
    ),
}

pub macro par_check {
    (
        $(#[$attr:meta])*
        $name:ident, $inp:literal, $( $val:expr ),*
    ) => {
        $(#[$attr])*
        #[test]
        fn $name() {
            init!($inp);

            match paragraph(&mut StringData::new($inp, ParserSettings::default()), false) {
                Block::Paragraph(ct, _)
                    => assert_eq!(vec![ $( $val ),* ] as Vec<Inline>, ct),

                x => assert_matches!(x, Block::Paragraph(..)),
            }
        }
    },

    (
        $(#[$attr:meta])*
        $name:ident, $opts:expr, $inp:literal, $( $val:expr ),*
    ) => {
        par_check!($(#[$attr:meta])* $name, $opts, ( $inp ), $( $val ),* );
    },

    (
        $(#[$attr:meta])*
        $name:ident, $opts:expr, ( $( $inp:literal ),+ $(,)? ), $( $val:expr ),*
    ) => {
        $(#[$attr])*
        #[test]
        fn $name() {
            init!(concat!( $($inp, "\n" ),+ ));

            let par = paragraph(
                &mut StringData::new(concat!($( $inp, "\n" ),+), $opts),
                false
            );

            match par {
                Block::Paragraph(ct, _)
                    => assert_eq!(vec![ $( $val ),* ] as Vec<Inline>, ct),

                x => assert_matches!(x, Block::Paragraph(..)),
            }
        }
    },
}

pub macro cm_par_check {
    (
        $(#[$attr:meta])*
        $name:ident, $inp:literal, $( $val:expr ),*
    ) => {
        cm_par_check!($(#[$attr])* $name, ( $inp ), $( $val ),*);
    },

    (
        $(#[$attr:meta])*
        $name:ident, ( $( $inp:literal ),+ $(,)? ), $( $val:expr ),*
    ) => {
        par_check!(
            $(#[$attr])* $name,
            ParserSettings::common_mark(),
            ( $( $inp ),+ ), $( $val ),*
        );
    },
}

pub macro body_check {
    (
        $(#[$attr:meta])*
        $name:ident, $inp:literal, $( $val:expr ),*
    ) => {
        $(#[$attr])*
        #[test]
        fn $name() {
            init!($inp);

            assert_eq!(
                [ $( $val ),* ].as_slice(),
                body(&mut StringData::new($inp, ParserSettings::default()))
            );
        }
    },

    (
        $(#[$attr:meta])*
        $name:ident, $opts:expr, $inp:literal, $( $val:expr ),*
    ) => {
        body_check!($(#[$attr:meta])* $name, $opts, ( $inp ), $( $val ),* );
    },

    (
        $(#[$attr:meta])*
        $name:ident, $opts:expr, ( $inp_first:literal $( , $inp:literal )* $(,)? ), $( $val:expr ),*
    ) => {
        $(#[$attr])*
        #[test]
        fn $name() {
            init!(concat!( $inp_first $( ,"\n", $inp )* ));

            assert_eq!(
                [ $( $val ),* ].as_slice(),
                body(&mut StringData::new(
                    concat!($inp_first $( ,"\n", $inp )* ), $opts
                ))
            );
        }
    },
}

pub macro cm_body_check {
    (
        $(#[$attr:meta])*
        $name:ident, $inp:literal, $( $val:expr ),*
    ) => {
        cm_body_check!($(#[$attr])* $name, ( $inp ), $( $val ),*);
    },

    (
        $(#[$attr:meta])*
        $name:ident, ( $( $inp:literal ),+ $(,)? ), $( $val:expr ),*
    ) => {
        body_check!(
            $(#[$attr])* $name,
            ParserSettings::common_mark(),
            ( $( $inp ),+ ), $( $val ),*
        );
    },
}
