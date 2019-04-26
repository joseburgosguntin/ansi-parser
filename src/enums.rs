use num_derive::FromPrimitive;

#[derive(Debug, PartialEq, FromPrimitive)]
pub enum TextAttribute {
    Off          = 0,
    Bold         = 1,
    Underscore   = 4,
    Blink        = 5,
    ReverseVideo = 7,
    Concealed    = 8
}

#[derive(Debug, PartialEq, FromPrimitive)]
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

#[derive(Debug, PartialEq)]
pub enum Output<'a> {
    TextBlock(&'a str),
    Escape(AnsiSequence)
}
