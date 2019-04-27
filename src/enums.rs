#[cfg(test)]
mod tests;

use num_derive::FromPrimitive;
use num_derive::ToPrimitive;

use num_traits::ToPrimitive;

///A list of available text attributes.
#[derive(Debug, PartialEq, FromPrimitive, ToPrimitive)]
pub enum TextAttribute {
    Off          = 0,
    Bold         = 1,
    Underscore   = 4,
    Blink        = 5,
    ReverseVideo = 7,
    Concealed    = 8
}

impl Display for TextAttribute {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "{}", self.to_u8().unwrap_or(0))
    }
}

///The basic ANSI colors.
#[derive(Debug, PartialEq, FromPrimitive, ToPrimitive)]
pub enum Color {
    Black   = 30,
    Red     = 31,
    Green   = 32,
    Yellow  = 33,
    Blue    = 34,
    Magenta = 35,
    Cyan    = 36,
    White   = 37
}

impl Display for Color {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "{}", self.to_u8().unwrap_or(0))
    }
}

///The following are the implemented ANSI escape sequences. More to be added.
#[derive(Debug, PartialEq)]
pub enum AnsiSequence {
    CursorPos(u32, u32),
    CursorUp(u32),
    CursorDown(u32),
    CursorForward(u32),
    CursorBackward(u32),
    CursorSave,
    CursorRestore,
    EraseDisplay,
    EraseLine,
    SetGraphicsMode{
        ta: TextAttribute,
        fg: Color,
        bg: Color
    },
    SetMode(u8),
    ResetMode(u8),
}

use std::fmt::Display;
impl Display for AnsiSequence {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "\x1b[")?;
        
        use AnsiSequence::*;
        match self {
            CursorPos(line, col) 
                => write!(formatter, "{};{}H", line, col),
            CursorUp(amt)
                => write!(formatter, "{}A", amt),
            CursorDown(amt)
                => write!(formatter, "{}B", amt),
            CursorForward(amt)
                => write!(formatter, "{}C", amt),
            CursorBackward(amt)
                => write!(formatter, "{}D", amt),
            CursorSave
                => write!(formatter, "s"),
            CursorRestore
                => write!(formatter, "u"),
            EraseDisplay
                => write!(formatter, "2J"),
            EraseLine
                => write!(formatter, "K"),
            SetGraphicsMode{ta, fg, bg}
                => write!(formatter, "{};{};{}m", ta, fg, bg),
            SetMode(mode)
                => write!(formatter, "={}h", mode),
            ResetMode(mode)
                => write!(formatter, "={}l", mode)
        }
    }
}

///This is what is outputted by the parsing iterator.
///Each block contains either straight-up text, or simply
///an ANSI escape sequence.
#[derive(Debug, PartialEq)]
pub enum Output<'a> {
    TextBlock(&'a str),
    Escape(AnsiSequence)
}
