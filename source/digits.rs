use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader, BufWriter, Write};

fn sanitize_line(line: &str, allow_alpha: bool, allow_numeric: bool, allow_punctuation: bool, allow_unicode: bool, collapse_punctuation: bool) -> String {
    let mut sanitized = String::new();

    for c in line.chars() {
        if allow_alpha && c.is_alphabetic() {
            sanitized.push(c);
        } else if allow_numeric && c.is_numeric() {
            sanitized.push(c);
        } else if allow_punctuation && c.is_ascii_punctuation() {
            sanitized.push(c);
        } else if allow_unicode {
            sanitized.push(c);
        } else if collapse_punctuation && sanitized.ends_with(c) {
            continue;
        }
    }

    sanitized
}

fn sanitize_file(input_path: &str, output_path: &str, allow_alpha: bool, allow_numeric: bool, allow_punctuation: bool, allow_unicode: bool, collapse_punctuation: bool) -> io::Result<()> {
    let input_file = File::open(input_path)?;
    let reader = BufReader::new(input_file);

    let output_file = File::create(output_path)?;
    let mut writer = BufWriter::new(output_file);

    for line in reader.lines() {
        let line = line?;
        let sanitized = sanitize_line(&line, allow_alpha, allow_numeric, allow_punctuation, allow_unicode, collapse_punctuation);
        writeln!(writer, "{}", sanitized)?;
    }

    Ok(())
}

fn print_help() {
    println!("Usage: program_name <input_file> <output_file> [options]");
    println!("Options:");
    println!("-a  Allow alphabetic characters");
    println!("-n  Allow numeric characters");
    println!("-p  Allow punctuational characters");
    println!("-u  Allow unicode characters");
    println!("--help  Show this help message");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: {} <input_file> <output_file> [options]", args[0]);
        std::process::exit(1);
    }

    let input_file = &args[1];
    let output_file = &args[2];

    if input_file == "--help" {
        print_help();
        std::process::exit(0);
    }

    if let Err(err) = sanitize_file(input_file, output_file, true, true, true, true, false) {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    }

    println!("Sanitized file created: {}", output_file);
}
