#[cfg(test)]
mod tests;

use crate::{AnsiSequence, Output, Color, TextAttribute};

use num_traits::cast::FromPrimitive;
use std::convert::TryInto;
use nom::*;

named!(
    parse_int<u32>,
    map_res!(
        map_res!(
            nom::digit,
            std::str::from_utf8
        ),
        |s: &str| s.parse::<u32>()
    )
);

named!(
    parse_fg_color<Color>,
    do_parse!(
        val: parse_int >>
        worked: expr_opt!(FromPrimitive::from_u32(val)) >>
        (worked)
    )
);

named!(
    parse_bg_color<Color>,
    do_parse!(
        val:    parse_int                               >>
        val:    expr_opt!(val.checked_sub(10))          >>
        worked: expr_opt!(FromPrimitive::from_u32(val)) >>
        (worked)
    )
);

named!(
    parse_text_attr<TextAttribute>,
    map!(
        map!(
            parse_int,
            FromPrimitive::from_u32
        ),
        Option::unwrap
    )
);

named!(
    cursor_pos<AnsiSequence>,
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
    cursor_up<AnsiSequence>,
    do_parse!(
        am: parse_int >>
        tag!("A")     >>
        (AnsiSequence::CursorUp(am))
    )
);

named!(
    cursor_down<AnsiSequence>,
    do_parse!(
        am: parse_int >>
        tag!("B")     >>
        (AnsiSequence::CursorDown(am))
    )
);

named!(
    cursor_forward<AnsiSequence>,
    do_parse!(
        am: parse_int >>
        tag!("C")     >>
        (AnsiSequence::CursorForward(am))
    )
);

named!(
    cursor_backward<AnsiSequence>,
    do_parse!(
        am: parse_int >>
        tag!("D")     >>
        (AnsiSequence::CursorBackward(am))
    )
);

named!(
    cursor_save<AnsiSequence>,
    do_parse!(
        tag!("s") >>
        (AnsiSequence::CursorSave)
    )
);

named!(
    cursor_restore<AnsiSequence>,
    do_parse!(
        tag!("u") >>
        (AnsiSequence::CursorRestore)
    )
);

named!(
    erase_display<AnsiSequence>,
    do_parse!(
        tag!("2J") >>
        (AnsiSequence::EraseDisplay)
    )
);

named!(
    erase_line<AnsiSequence>,
    do_parse!(
        tag!("K") >>
        (AnsiSequence::EraseDisplay)
    )
);

named!(
    graphics_mode<AnsiSequence>,
    do_parse!(
        ta: parse_text_attr >>
        tag!(";")           >>
        fg: parse_fg_color  >>
        tag!(";")           >>
        bg: parse_bg_color  >>
        tag!("m")           >>
        (AnsiSequence::SetGraphicsMode{
            ta: ta,
            fg: fg,
            bg: bg
        })
    )
);

named!(
    set_mode<AnsiSequence>,
    do_parse!(
        tag!("=")                        >>
        mode: parse_int                  >>
        conv: expr_res!(mode.try_into()) >>
        tag!("h")                        >>
        (AnsiSequence::SetMode(conv))
    )
);

named!(
    reset_mode<AnsiSequence>,
    do_parse!(
        tag!("=")                        >>
        mode: parse_int                  >>
        conv: expr_res!(mode.try_into()) >>
        tag!("l")                        >>
        (AnsiSequence::ResetMode(conv))
    )
);

named!(
    combined<AnsiSequence>,
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
    )
);

named!(
    parse_escape<Output>,
    do_parse!(
        tag_s!("\x27[") >>
        seq: combined     >>
        (Output::Escape(seq))
    )
);

named!(
    parse_str<Output>,
    do_parse!(
        text: map_res!(
            take_until!("\x27["),
            std::str::from_utf8
        ) >>
        (Output::TextBlock(text))
    )
);

named!(
    parse_output<Output>,
    do_parse!(
        out: alt!(parse_escape | parse_str) >>
        (out)
    )
);

pub struct ParserIterator<'a> {
    dat: &'a[u8],
    done: bool
}

impl<'a> Iterator for ParserIterator<'a> {
    type Item = Output<'a>;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }
        let parse = parse_output(self.dat);

        if parse.is_ok() {
            let parse = parse.unwrap();

            self.dat = parse.0;
            Some(parse.1)
        }else{
            if self.done {
                None
            }else{
                self.done = true;
                Some(Output::TextBlock(std::str::from_utf8(self.dat)
                    .unwrap()))
            }
        }
    }
}

impl<'a> ParserIterator<'a> {
    pub fn new(string: &'a str) -> ParserIterator<'a> {
        ParserIterator {
            dat: string.as_bytes(),
            done: false
        }
    }
}
