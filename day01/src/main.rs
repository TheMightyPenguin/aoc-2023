use std::collections::HashMap;
use std::env::current_dir;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::vec;

fn main() {
    let numbers = HashMap::from([
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9'),
    ]);

    let lines = read_lines("./src/input.txt");
    let mut sum = 0;

    if let Ok(lines) = lines {
        for line in lines {
            let mut digits: Vec<(char, usize)> = vec![];
            let line = line.expect("line should be valid");

            // implementation could also use string.get with ranges to detect the numbers
            // this iteration parses the whole line 10 times (one per potential word digit + one to check every char)
            for number in &numbers {
                line.match_indices(number.0);
                for (pos, _) in line.match_indices(number.0) {
                    digits.push((number.1.clone(), pos));
                }
            }

            for (index, char) in line.chars().enumerate() {
                if char.is_numeric() {
                    digits.push((char, index));
                }
            }

            digits.sort_by(|a, b| a.1.cmp(&b.1));

            let first = digits[0].0;
            let last = digits[digits.len() - 1].0;

            let number = format!("{}{}", first, last)
                .parse::<u32>()
                .expect("should be a valid number");

            sum += number;
        }
    }

    println!("Sum: {}", sum);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let dir = current_dir()?;
    let file_path = dir.join(filename);
    let file = File::open(file_path)?;
    Ok(io::BufReader::new(file).lines())
}
