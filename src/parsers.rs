#[cfg(test)]
mod tests;

use crate::{AnsiSequence, Output};

use std::convert::TryInto;
use nom::*;

named!(
    parse_int<&str, u32>,
    map_res!(
        nom::digit,
        |s: &str| s.parse::<u32>()
    )
);

named!(
    cursor_pos<&str, AnsiSequence>,
    do_parse!(
        x: parse_int    >>
        tag!(";")       >>
        y: parse_int    >>
        alt!(
            tag!("H") | 
            tag!("f")
        )               >>
        (AnsiSequence::CursorPos(x, y))
    )
);

named!(
    cursor_up<&str, AnsiSequence>,
    do_parse!(
        am: parse_int >>
        tag!("A")     >>
        (AnsiSequence::CursorUp(am))
    )
);

named!(
    cursor_down<&str, AnsiSequence>,
    do_parse!(
        am: parse_int >>
        tag!("B")     >>
        (AnsiSequence::CursorDown(am))
    )
);

named!(
    cursor_forward<&str, AnsiSequence>,
    do_parse!(
        am: parse_int >>
        tag!("C")     >>
        (AnsiSequence::CursorForward(am))
    )
);

named!(
    cursor_backward<&str, AnsiSequence>,
    do_parse!(
        am: parse_int >>
        tag!("D")     >>
        (AnsiSequence::CursorBackward(am))
    )
);

named!(
    cursor_save<&str, AnsiSequence>,
    do_parse!(
        tag!("s") >>
        (AnsiSequence::CursorSave)
    )
);

named!(
    cursor_restore<&str, AnsiSequence>,
    do_parse!(
        tag!("u") >>
        (AnsiSequence::CursorRestore)
    )
);

named!(
    erase_display<&str, AnsiSequence>,
    do_parse!(
        tag!("2J") >>
        (AnsiSequence::EraseDisplay)
    )
);

named!(
    erase_line<&str, AnsiSequence>,
    do_parse!(
        tag!("K") >>
        (AnsiSequence::EraseDisplay)
    )
);

named!(
    graphics_mode1<&str, AnsiSequence>,
    do_parse!(
        val: parse_int >>
        tag!("m")      >>
        (AnsiSequence::SetGraphicsMode(vec![val]))
    )
);

named!(
    graphics_mode2<&str, AnsiSequence>,
    do_parse!(
        val1: parse_int >>
        tag!(";")       >>
        val2: parse_int >>
        tag!("m")       >>
        (AnsiSequence::SetGraphicsMode(vec![val1, val2]))
    )
);

named!(
    graphics_mode3<&str, AnsiSequence>,
    do_parse!(
        val1: parse_int >>
        tag!(";")       >>
        val2: parse_int >>
        tag!(";")       >>
        val3: parse_int >>
        tag!("m")       >>
        (AnsiSequence::SetGraphicsMode(vec![val1, val2, val3]))
    )
);

named!(
    graphics_mode<&str, AnsiSequence>,
    alt!(
          graphics_mode1
        | graphics_mode2
        | graphics_mode3)
);

named!(
    set_mode<&str, AnsiSequence>,
    do_parse!(
        tag!("=")                        >>
        mode: parse_int                  >>
        conv: expr_res!(mode.try_into()) >>
        tag!("h")                        >>
        (AnsiSequence::SetMode(conv))
    )
);

named!(
    reset_mode<&str, AnsiSequence>,
    do_parse!(
        tag!("=")                        >>
        mode: parse_int                  >>
        conv: expr_res!(mode.try_into()) >>
        tag!("l")                        >>
        (AnsiSequence::ResetMode(conv))
    )
);

named!(
    hide_cursor<&str, AnsiSequence>,
    do_parse!(
        tag!("?25l") >>
        (AnsiSequence::HideCursor)
    )
);

named!(
    show_cursor<&str, AnsiSequence>,
    do_parse!(
        tag!("?25h") >>
        (AnsiSequence::ShowCursor)
    )
);

named!(
    combined<&str, AnsiSequence>,
    alt!(
          cursor_pos
        | cursor_up
        | cursor_down
        | cursor_forward
        | cursor_backward 
        | cursor_save
        | cursor_restore
        | erase_display
        | erase_line
        | graphics_mode
        | set_mode
        | reset_mode
        | hide_cursor
        | show_cursor
    )
);

named!(
    parse_escape<&str, Output>,
    do_parse!(
        tag_s!("\u{1b}[") >>
        seq: combined     >>
        (Output::Escape(seq))
    )
);

pub struct ParserIterator<'a> {
    dat: &'a str,
}

impl<'a> Iterator for ParserIterator<'a> {
    type Item = Output<'a>;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.dat == "" {
            return None;
        }

        let pos = self.dat.find('\u{1b}');
        if let Some(loc) = pos {
            if loc == 0 {
                let res = parse_escape(&self.dat[loc..]);

                if let Ok(ret) = res {
                    self.dat = &ret.0;
                    Some(ret.1)
                }else{
                    let pos = self.dat[(loc+1)..].find('\u{1b}');
                    if let Some(loc) = pos {
                        //Added to because it's based one character ahead
                        let loc = loc+1;

                        let temp = &self.dat[..loc];
                        self.dat = &self.dat[loc..];

                        Some(Output::TextBlock(temp))
                    }else{
                        let temp = self.dat;
                        self.dat = "";

                        Some(Output::TextBlock(temp))
                    }
                }

            }else {
                let temp = &self.dat[..loc];
                self.dat = &self.dat[loc..];

                Some(Output::TextBlock(&temp))
            }
        }else{
            let temp = self.dat;
            self.dat = "";
            Some(Output::TextBlock(temp))
        }
    }
}

impl<'a> ParserIterator<'a> {
    pub fn new(string: &'a str) -> ParserIterator<'a> {
        ParserIterator {
            dat: string,
        }
    }
}
