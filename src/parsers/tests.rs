use super::*;
use std::fmt::Write;

macro_rules! test_parser {
    ($name:ident, $string:expr) => {
        #[test]
        fn $name() {
            let mut buff = String::new();
            let ret = parse_escape($string);

            println!("{:#?}", ret);
            assert!(ret.is_ok());
            let ret = ret.unwrap().1;

            write!(&mut buff, "{}", ret)
                .unwrap();

            assert_eq!(buff, $string);
        }
    }
}

test_parser!(set_video_mode_a, "\u{1b}[4;31;42m");
test_parser!(set_video_mode_b, "\u{1b}[4m");

test_parser!(reset_mode, "\u{1b}[=13l");
test_parser!(set_mode,   "\u{1b}[=7h");

#[test]
fn test_parser_iterator() {
    let parse_str = "\x1b[=25l\x1b[=7l\x1b[0m\x1b[36m\x1b[1m-`";

    let strings: Vec<Output> = ParserIterator::new(parse_str)
        .collect();

    assert_eq!(strings.len(), 6);
}

#[test]
fn test_parser_iterator_failure() {
    let parse_str = "\x1b[=25l\x1b[=7l\x1b[0m\x1b[36;1;15;2m\x1b[1m-`";
    let strings: Vec<Output> = ParserIterator::new(parse_str)
        .collect();

    assert_eq!(strings.len(), 6);
}
