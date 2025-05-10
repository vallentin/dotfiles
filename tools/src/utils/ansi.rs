use std::fmt;

use crate::utils::color;

macro_rules! def_enum {
    ($vis:vis $name:ident {
        $($color:ident -> $variant:ident => $code:expr),*
        $(,)?
    }) => {
        #[derive(Clone, Copy, Debug)]
        $vis enum $name {
            $($variant),*
        }

        impl $name {
            pub const VARIANTS: &'static [Self] = &[
                $(Self::$variant),*
            ];

            pub const fn to_ansi_code(self) -> &'static str {
                match self {
                    $(Self::$variant => $code),*
                }
            }
        }

        $(
            pub const $color: $name = $name::$variant;
        )*
    };
}

def_enum!(pub AnsiColor {
    RESET          -> Reset         => "\x1B[0m",
    BLACK          -> Black         => "\x1B[30m",
    RED            -> Red           => "\x1B[31m",
    GREEN          -> Green         => "\x1B[32m",
    YELLOW         -> Yellow        => "\x1B[33m",
    BLUE           -> Blue          => "\x1B[34m",
    MAGENTA        -> Magenta       => "\x1B[35m",
    CYAN           -> Cyan          => "\x1B[36m",
    WHITE          -> White         => "\x1B[37m",
    BRIGHT_BLACK   -> BrightBlack   => "\x1B[90m",
    BRIGHT_RED     -> BrightRed     => "\x1B[91m",
    BRIGHT_GREEN   -> BrightGreen   => "\x1B[92m",
    BRIGHT_YELLOW  -> BrightYellow  => "\x1B[93m",
    BRIGHT_BLUE    -> BrightBlue    => "\x1B[94m",
    BRIGHT_MAGENTA -> BrightMagenta => "\x1B[95m",
    BRIGHT_CYAN    -> BrightCyan    => "\x1B[96m",
    BRIGHT_WHITE   -> BrightWhite   => "\x1B[97m",
});

impl fmt::Display for AnsiColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if color::is_enabled() {
            self.to_ansi_code().fmt(f)?;
        }

        Ok(())
    }
}
