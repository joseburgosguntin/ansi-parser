mod enums;
mod parsers;

///This is a library for parsing ANSI escape sequences. Currently all the basic escape sequences
///are implemented:
/// + Cursor Position
/// + Cursor {Up, Down, Forward, Backward}
/// + Cursor {Save, Restore}
/// + Erase Display
/// + Erase Line
/// + Set Graphics mode
/// + Set and Reset Text Mode
///
/// This is done through a pulldown type parser, where an iterator is exposed. This essentially
/// turns all of the ANSI sequences into enums and splits the string at every location that there
/// was an ANSI Sequence.

pub use enums::*;
pub use parsers::{
    ParserIterator
};
