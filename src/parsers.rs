#[cfg(test)]
mod tests;

use crate::{AnsiSequence, Output};

use std::convert::TryInto;
use nom::*;

macro_rules! tag_parser {
    ($sig:ident, $tag:expr, $ret:expr) => {
        named!(
            $sig<&str, AnsiSequence>,
            do_parse!(
                tag!($tag) >>
                ($ret)
            )
        );
    }
}

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

tag_parser!(cursor_save,           "s", AnsiSequence::CursorSave);
tag_parser!(cursor_restore,        "u", AnsiSequence::CursorRestore);
tag_parser!(erase_display,        "2J", AnsiSequence::EraseDisplay);
tag_parser!(erase_line,            "K", AnsiSequence::EraseLine);
tag_parser!(hide_cursor,        "?25l", AnsiSequence::HideCursor);
tag_parser!(show_cursor,        "?25h", AnsiSequence::ShowCursor);
tag_parser!(cursor_to_app,       "?1h", AnsiSequence::CursorToApp);
tag_parser!(set_new_line_mode,   "20h", AnsiSequence::SetNewLineMode);
tag_parser!(set_col_132,         "?3h", AnsiSequence::SetCol132);
tag_parser!(set_smooth_scroll,   "?4h", AnsiSequence::SetSmoothScroll);
tag_parser!(set_reverse_video,   "?5h", AnsiSequence::SetReverseVideo);
tag_parser!(set_origin_rel,      "?6h", AnsiSequence::SetOriginRelative);
tag_parser!(set_auto_wrap,       "?7h", AnsiSequence::SetAutoWrap);
tag_parser!(set_auto_repeat,     "?8h", AnsiSequence::SetAutoRepeat);
tag_parser!(set_interlacing,     "?9h", AnsiSequence::SetInterlacing);
tag_parser!(set_linefeed,        "20l", AnsiSequence::SetLineFeedMode);
tag_parser!(set_cursorkey,       "?1l", AnsiSequence::SetCursorKeyToCursor);
tag_parser!(set_vt52,            "?2l", AnsiSequence::SetVT52);
tag_parser!(set_col80,           "?3l", AnsiSequence::SetCol80);
tag_parser!(set_jump_scroll,     "?4l", AnsiSequence::SetJumpScrolling);
tag_parser!(set_normal_video,    "?5l", AnsiSequence::SetNormalVideo);
tag_parser!(set_origin_abs,      "?6l", AnsiSequence::SetOriginAbsolute);
tag_parser!(reset_auto_wrap,     "?7l", AnsiSequence::ResetAutoWrap);
tag_parser!(reset_auto_repeat,   "?8l", AnsiSequence::ResetAutoRepeat);
tag_parser!(reset_interlacing,   "?9l", AnsiSequence::ResetInterlacing);

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
        | cursor_to_app
        | set_new_line_mode
        | set_col_132
        | set_smooth_scroll
        | set_reverse_video
        | set_origin_rel
        | set_auto_wrap
        | set_auto_repeat
        | set_interlacing
        | set_linefeed
        | set_cursorkey
        | set_vt52
        | set_col80
        | set_jump_scroll
        | set_normal_video
        | set_origin_abs
        | reset_auto_wrap
        | reset_auto_repeat
        | reset_interlacing
    )
);

named!(
    pub parse_escape<&str, Output>,
    do_parse!(
        tag_s!("\u{1b}[") >>
        seq: combined     >>
        (Output::Escape(seq))
    )
);

