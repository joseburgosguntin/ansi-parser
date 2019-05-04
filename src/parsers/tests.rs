use super::*;

macro_rules! test_parser {
    ($func:expr, $string:expr) => ($func($string));
}

#[test]
fn test_invalid_escapes() {
    let parse = "4;31;1;5m";
    let temp = parse_escape(parse);

    assert!(!temp.is_ok());
}

#[test]
fn test_graphics_mode() {
    let temp = test_parser!(graphics_mode, "4;31;42m");

    assert!(temp.is_ok());
    assert_eq!(AnsiSequence::SetGraphicsMode(
            vec![4,31,42]
        ),
        temp.unwrap().1
    );

    let temp = test_parser!(graphics_mode, "4m");

    assert!(temp.is_ok());
    assert_eq!(AnsiSequence::SetGraphicsMode(vec![4]),
        temp.unwrap().1
    );
}

#[test]
fn test_set_mode() {
    let temp  = test_parser!(set_mode, "=7h");
    assert_eq!(AnsiSequence::SetMode(7), temp.unwrap().1);
}

#[test]
fn test_reset_mode() {
    let parse = "=13l";
    let temp  = reset_mode(parse);

    assert_eq!(AnsiSequence::ResetMode(13), temp.unwrap().1);
}

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
