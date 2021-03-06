#![feature(split_ascii_whitespace)]

extern crate argparse;

use argparse::{ArgumentParser, StoreTrue};
use std::io::{self, BufRead};
use std::str;

fn main() {
    let mut word_count_flag = false;
    let mut char_count_flag = false;
    let mut byte_count_flag = false;
    let mut line_count_flag = false;

    {
        let mut ap = ArgumentParser::new();
        ap.set_description("Print newline, word, and byte counts for each file");
        ap.refer(&mut word_count_flag).add_option(
            &["-w", "--words"],
            StoreTrue,
            "print the word counts",
        );
        ap.refer(&mut char_count_flag).add_option(
            &["-m", "--chars"],
            StoreTrue,
            "print the character counts",
        );
        ap.refer(&mut byte_count_flag).add_option(
            &["-c", "--bytes"],
            StoreTrue,
            "print the byte counts",
        );
        ap.refer(&mut line_count_flag).add_option(
            &["-l", "--lines"],
            StoreTrue,
            "print the line counts",
        );
        ap.parse_args_or_exit();
    }

    let (mut word_count, mut char_count, mut byte_count, mut line_count) =
        (0usize, 0usize, 0usize, 0usize);
    let mut buffer = Vec::new();
    let stdin = io::stdin();
    let mut stdin = stdin.lock();

    loop {
        match stdin.read_until(b'\n', &mut buffer) {
            Ok(0) => {
                break;
            }
            Ok(len) => {
                if word_count_flag {
                    word_count += str::from_utf8(&buffer)
                        .unwrap()
                        .split_ascii_whitespace()
                        .count();
                }

                if char_count_flag {
                    char_count += str::from_utf8(&buffer)
                        .unwrap()
                        .chars()
                        .count();
                }

                if byte_count_flag {
                    byte_count += len;
                }

                if line_count_flag {
                    line_count += 1;
                }

                buffer.clear();
            }
            Err(err) => {
                panic!("{}", err);
            }
        }
    }

    let print_count = |count: usize, should_print: bool| {
        if should_print {
            print!("{} ", count);
        }
    };

    print_count(line_count, line_count_flag);
    print_count(word_count, word_count_flag);
    print_count(char_count, char_count_flag);
    print_count(byte_count, byte_count_flag);
    println!();
}
