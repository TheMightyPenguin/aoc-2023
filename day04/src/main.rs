use std::collections::HashSet;
use std::env::current_dir;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut sum = 0;

    let lines = read_lines("./src/input.txt")
        .expect("File should exist")
        .map(|l| l.expect("Line should be readable"))
        .collect::<Vec<_>>();

    for (_, line) in lines.iter().enumerate() {
        println!("Line: {}", line);
        let mut points = 0;
        let mut winning_numbers = HashSet::new();

        let parts = line
            .split(": ")
            .nth(1)
            .expect("game content should be present");
        let number_parts = parts.split(" | ").collect::<Vec<_>>();

        for number in number_parts[0].split_whitespace() {
            winning_numbers.insert(number.parse::<i32>().expect("number should be valid"));
        }

        for candidate in number_parts[1].split_whitespace() {
            let candidate_number = candidate.parse::<i32>().expect("number should be valid");
            if winning_numbers.contains(&candidate_number) {
                points = if points == 0 { 1 } else { points * 2 };
            }
        }

        sum += points;
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
