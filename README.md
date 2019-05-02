![pipeline status](https://img.shields.io/gitlab/pipeline/gitlab-org/gitlab-ce.svg?style=flat-square) ![deps](https://img.shields.io/librariesio/release/cargo/ansi-parser.svg?style=flat-square) ![license](https://img.shields.io/crates/l/ansi-parser.svg?style=flat-square) ![downloads](https://img.shields.io/crates/d/ansi-parser.svg?style=flat-square)
# Ansi Escape Sequence Parser


This is a library for parsing ANSI escape sequences. Currently all the basic escape sequences
are implemented:
 + Cursor Position
 + Cursor {Up, Down, Forward, Backward}
 + Cursor {Save, Restore}
 + Erase Display
 + Erase Line
 + Set Graphics mode
 + Set and Reset Text Mode

 This is done through a pulldown type parser, where an iterator is exposed. This essentially
 turns all of the ANSI sequences into enums and splits the string at every location that there
 was an ANSI Sequence.

 Example:

 ```rust

use ansi_parser::{Output, ParserIterator};

fn main() {
    //Your input string here
    let string = "...";
    let parsed: Vec<Output> = ParserIterator::new(&string)
        //Because it implements Iterator, you can use whatever
        //your favorite iterator functions are.
        .take(4)
        .collect();

    for block in parsed.into_iter() {
        match block {
            TextBlock(text)   => println!("{}", text),
            AnsiSequence(seq) => println!("{}", seq)
        }
    }
}
 ```
