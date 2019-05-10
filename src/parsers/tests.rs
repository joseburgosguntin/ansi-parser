use super::*;
use crate::*;

use std::fmt::Write;

macro_rules! test_parser {
    ($name:ident, $string:expr) => {
        #[test]
        fn $name() {
            let mut buff = String::new();
            let ret = parse_escape($string);

            assert!(ret.is_ok());
            let ret = ret.unwrap().1;

            write!(&mut buff, "{}", ret)
                .unwrap();

            assert_eq!(buff, $string);
        }
    }
}

test_parser!(cursor_pos,     "\u{1b}[10;5H");
test_parser!(cursor_up,      "\u{1b}[5A");
test_parser!(cursor_down,    "\u{1b}[5B");
test_parser!(cursor_forward, "\u{1b}[5C");
test_parser!(cursor_backward,"\u{1b}[5D");
test_parser!(cursor_save,    "\u{1b}[s");
test_parser!(cursor_restore, "\u{1b}[u");

test_parser!(erase_display, "\u{1b}[2J");
test_parser!(erase_line,    "\u{1b}[K");

test_parser!(set_video_mode_a, "\u{1b}[4m");
test_parser!(set_video_mode_b, "\u{1b}[4;42m");
test_parser!(set_video_mode_c, "\u{1b}[4;31;42m");

test_parser!(reset_mode, "\u{1b}[=13l");
test_parser!(set_mode,   "\u{1b}[=7h");

test_parser!(show_cursor,   "\u{1b}[?25h");
test_parser!(hide_cursor,   "\u{1b}[?25l");
test_parser!(cursor_to_app, "\u{1b}[?1h");

test_parser!(set_newline_mode,  "\u{1b}[20h");
test_parser!(set_column_132,    "\u{1b}[?3h");
test_parser!(set_smooth_scroll, "\u{1b}[?4h");
test_parser!(set_reverse_video, "\u{1b}[?5h");
test_parser!(set_origin_rel,    "\u{1b}[?6h");
test_parser!(set_auto_wrap,     "\u{1b}[?7h");
test_parser!(set_auto_repeat,   "\u{1b}[?8h");
test_parser!(set_interlacing,   "\u{1b}[?9h");

test_parser!(set_cursor_key_to_cursor, "\u{1b}[?1l");

test_parser!(set_linefeed,      "\u{1b}[20l");
test_parser!(set_vt52,          "\u{1b}[?2l");
test_parser!(set_col80,         "\u{1b}[?3l");
test_parser!(set_jump_scroll,   "\u{1b}[?4l");
test_parser!(set_normal_video,  "\u{1b}[?5l");
test_parser!(set_origin_abs,    "\u{1b}[?6l");
test_parser!(reset_auto_wrap,   "\u{1b}[?7l");
test_parser!(reset_auto_repeat, "\u{1b}[?8l");
test_parser!(reset_interlacing, "\u{1b}[?9l");

test_parser!(set_alternate_keypad, "\u{1b}=");
test_parser!(set_numeric_keypad, "\u{1b}>");
test_parser!(set_uk_g0, "\u{1b}(A");
test_parser!(set_uk_g1, "\u{1b})A");
test_parser!(set_us_g0, "\u{1b}(B");
test_parser!(set_us_g1, "\u{1b})B");
test_parser!(set_g0_special, "\u{1b}(0");
test_parser!(set_g1_special, "\u{1b})0");
test_parser!(set_g0_alternate, "\u{1b}(1");
test_parser!(set_g1_alternate, "\u{1b})1");
test_parser!(set_g0_graph, "\u{1b}(2");
test_parser!(set_g1_graph, "\u{1b})2");
test_parser!(set_single_shift2, "\u{1b}N");
test_parser!(set_single_shift3, "\u{1b}O");

#[test]
fn test_parser_iterator() {
    let strings: Vec<_> = "\x1b[=25l\x1b[=7l\x1b[0m\x1b[36m\x1b[1m-`"
        .ansi_parse()
        .collect();

    assert_eq!(strings.len(), 6);
}

#[test]
fn test_parser_iterator_failure() {
    let strings: Vec<_> = "\x1b[=25l\x1b[=7l\x1b[0m\x1b[36;1;15;2m\x1b[1m-`"
        .ansi_parse()
        .collect();

    assert_eq!(strings.len(), 6);
}
