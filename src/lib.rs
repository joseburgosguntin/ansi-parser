use nom::*;

#[derive(Debug, PartialEq)]
enum TextAttributes {
    Off          = 0,
    Bold         = 1,
    Underscore   = 4,
    Blink        = 5,
    ReverseVideo = 7,
    Concealed    = 8
}

#[derive(Debug, PartialEq)]
enum Color {
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
enum AnsiSequence {
    CursorPos(u32, u32),
    CursorUp(u32),
    CursorDown(u32),
    CursorForward(u32),
    CursorBackward(u32),
    SaveCursorPos(u32),
    RestoreCursorPos(u32),
    EraseDisplay,
    EraseLine,
    SetGraphicsMode{
        ta: Option<TextAttributes>,
        fg: Option<Color>,
        bg: Option<Color>
    },
    SetMode(u8),
    Resetmode(u8),
}

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
    combined<AnsiSequence>,
    alt!(
          cursor_pos
        | cursor_up
        | cursor_down
        | cursor_forward
        | cursor_backward 
    )
);

named!(
    parse_escape<AnsiSequence>,
    do_parse!(
        tag_s!("\\\x27[") >>
        seq: combined     >>
        (seq)
    )
);

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_combined() {
        let temp = "\\\x27[15;244H";
        let temp = parse_escape(temp.as_bytes());

        assert!(temp.is_ok());
        assert_eq!(AnsiSequence::CursorPos(15, 244), temp.unwrap().1);

        let temp = "\\\x27[22D";
        let temp = parse_escape(temp.as_bytes());

        assert!(temp.is_ok());
        assert_eq!(AnsiSequence::CursorBackward(22), temp.unwrap().1);
    }
}
