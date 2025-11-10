use std::env;
use std::process;
use std::fs::File; // For read_file_lines()
use std::io::{self, BufRead}; // For read_file_lines()

/// Reads the file at the supplied path, and returns a vector of strings.
fn read_file_lines(filename: &String) -> Result<Vec<String>, io::Error> {
    let file = File::open(filename)?;
    let mut res: Vec<String> = Vec::<String>::new();
    for line in io::BufReader::new(file).lines() {
        let line_str = line?;
        res.push(line_str);
    }
    Ok(res)
    // Be sure to delete the #[allow(unused)] line above
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Too few arguments.");
        process::exit(1);
    }
    let filename = &args[1];
    let lines = read_file_lines(filename).unwrap();
    let mut word_num = 0;
    let mut char_num = 0;
    for line in &lines {
        word_num += line.split(' ').count();
        char_num += line.chars().count();
    }
    println!("lines {}, words {}, chars {}", lines.len(), word_num, char_num);
}
