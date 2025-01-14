use std::env;
use std::io;
use std::process;

fn match_pattern(input_line: &str, pattern: &str) -> bool {
    if pattern.starts_with("^") {
        // Match at the start of the input only, don't search later in the input string
        return match_pattern_occurrence(input_line, &pattern[1..]);
    }

    let mut chars_iter = input_line.chars();
    loop {
        if match_pattern_occurrence(chars_iter.as_str(), pattern) {
            return true;
        }

        if chars_iter.next().is_none() {
            return false;
        }
    }
}

fn match_pattern_occurrence(input: &str, pattern: &str) -> bool {
    if pattern.is_empty() {
        true
    } else if pattern == "$" {
        input.len() == 0
    } else if input.is_empty() {
        false
    } else if pattern.starts_with(r"\d") {
        match input.chars().next() {
            Some('0'..='9') => match_pattern_occurrence(&input[1..], &pattern[2..]),
            _ => false,
        }
    } else if pattern.starts_with(r"\w") {
        match input.chars().next() {
            Some('0'..='9' | 'a'..='z' | 'A'..='Z' | '_') => {
                match_pattern_occurrence(&input[1..], &pattern[2..])
            }
            _ => false,
        }
    } else if pattern.starts_with("[^") {
        let end_index = if let Some(i) = pattern.find("]") {
            i
        } else {
            panic!("Unclosed '[' in pattern");
        };

        let current_input_char = input.chars().next().unwrap();
        for ch in pattern.chars().skip(2).take(end_index - 2) {
            if ch == current_input_char {
                return false;
            }
        }

        match_pattern_occurrence(
            &input[current_input_char.len_utf8()..],
            &pattern[(end_index + 1)..],
        )
    } else if pattern.starts_with("[") {
        let end_index = if let Some(i) = pattern.find("]") {
            i
        } else {
            panic!("Unclosed '[' in pattern");
        };

        let current_input_char = input.chars().next().unwrap();
        for ch in pattern.chars().skip(1).take(end_index - 1) {
            if ch == current_input_char {
                return match_pattern_occurrence(
                    &input[current_input_char.len_utf8()..],
                    &pattern[(end_index + 1)..],
                );
            }
        }

        false
    } else if pattern.chars().count() == 1 {
        input.contains(pattern)
    } else if input.chars().next() == pattern.chars().next() {
        match_pattern_occurrence(
            &input[input.chars().next().unwrap().len_utf8()..],
            &pattern[pattern.chars().next().unwrap().len_utf8()..],
        )
    } else {
        // No way to match the pattern to the input, stop and let the program proceed to the
        // next character in the input.
        false
    }
}

// Usage: echo <input_text> | your_grep.sh -E <pattern>
fn main() {
    if env::args().nth(1).unwrap() != "-E" {
        println!("Expected first argument to be '-E'");
        process::exit(1);
    }

    let pattern = env::args().nth(2).unwrap();
    let mut input_line = String::new();

    io::stdin().read_line(&mut input_line).unwrap();

    if match_pattern(&input_line, &pattern) {
        process::exit(0)
    } else {
        process::exit(1)
    }
}
