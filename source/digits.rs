use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

fn sanitize_input(input: &str, allow_alphabetic: bool, allow_numeric: bool, allow_punctuational: bool, allow_unicode: bool) -> String {
    let mut sanitized = String::new();

    for c in input.chars() {
        if c.is_ascii_digit() && allow_numeric {
            sanitized.push(c);
        } else if c.is_ascii_alphabetic() && allow_alphabetic {
            sanitized.push(c);
        } else if c.is_ascii_punctuation() && allow_punctuational {
            sanitized.push(c);
        } else if !c.is_ascii() && allow_unicode {
            sanitized.push(c);
        }
    }

    sanitized
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        println!("Usage: {} <input_file> <output_file>", args[0]);
        return;
    }

    let input_file = &args[1];
    let output_file = &args[2];

    let input_path = std::path::Path::new(input_file);
    if !input_path.exists() {
        println!("File not found: {}", input_file);
        return;
    }

    let file = File::open(input_file).expect("Failed to open input file");
    let reader = BufReader::new(file);

    let mut sanitized_output = String::new();
    let mut prev_punctuation = false;

    for line in reader.lines() {
        let input_line = line.expect("Failed to read line");
        let sanitized_line = sanitize_input(&input_line, true, true, true, true);

        if !prev_punctuation || !sanitized_line.is_empty() {
            sanitized_output.push_str(&sanitized_line);
            sanitized_output.push('\n');
        }

        prev_punctuation = sanitized_line.chars().last().map(|c| c.is_ascii_punctuation()).unwrap_or(false);
    }

    let mut output_file = File::create(output_file).expect("Failed to create output file");
    output_file
        .write_all(sanitized_output.as_bytes())
        .expect("Failed to write output file");

    println!("Sanitized file created: {}", output_file);
}
