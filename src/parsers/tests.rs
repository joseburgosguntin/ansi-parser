use super::*;

#[test]
fn test_graphics_mode() {
    let parse = "4;31;42m";
    let temp = graphics_mode(parse.as_bytes());

    assert!(temp.is_ok());
    assert_eq!(AnsiSequence::SetGraphicsMode{
            ta: TextAttribute::Underscore,
            fg: Color::Red,
            bg: Color::Green
        },
        temp.unwrap().1
    );
}

#[test]
fn test_set_mode() {
    let parse = "=7h";
    let temp  = set_mode(parse.as_bytes());

    assert_eq!(AnsiSequence::SetMode(7), temp.unwrap().1);
}

#[test]
fn test_reset_mode() {
    let parse = "=13l";
    let temp  = reset_mode(parse.as_bytes());

    assert_eq!(AnsiSequence::ResetMode(13), temp.unwrap().1);
}

#[test]
fn test_parser_iterator() {
    let parse_str = "Hello, world? How are \x27[=7lyou? I hope you're doing well.";

    let strings: Vec<Output> = iterate_on(parse_str)
        .collect();

    println!("{:#?}", strings);
}
