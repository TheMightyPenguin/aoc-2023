use std::collections::{HashMap, HashSet};
use std::env::current_dir;
use std::fs::File;
use std::io::{self, BufRead};
use std::ops::Add;
use std::path::Path;

fn main() {
    let mut sum = 0;
    let mut scratchcard_counts: HashMap<usize, usize> = HashMap::new();

    let lines = read_lines("./src/input.txt")
        .expect("File should exist")
        .map(|l| l.expect("Line should be readable"))
        .collect::<Vec<_>>();

    for (_, line) in lines.iter().enumerate() {
        let mut points = 0;
        let mut winning_numbers = HashSet::new();

        let parts = line.split(": ").collect::<Vec<_>>();
        let game_id = parts[0]
            .split_whitespace()
            .nth(1)
            .expect("game id should exist")
            .parse::<usize>()
            .expect("game id should be valid number");
        let number_parts = parts[1].split(" | ").collect::<Vec<_>>();

        for number in number_parts[0].split_whitespace() {
            winning_numbers.insert(number.parse::<i32>().expect("number should be valid"));
        }

        for candidate in number_parts[1].split_whitespace() {
            let candidate_number = candidate.parse::<i32>().expect("number should be valid");
            if winning_numbers.contains(&candidate_number) {
                points += 1;
            }
        }

        let entry = scratchcard_counts.get(&game_id).unwrap_or(&0);
        let multiplier = entry.add(1);
        scratchcard_counts.insert(game_id, multiplier);

        for n in (game_id + 1)..=(game_id + points) {
            let entry = scratchcard_counts.get(&n).unwrap_or(&0);
            scratchcard_counts.insert(n, entry.add(1 * multiplier));
        }
    }

    for (_, count) in scratchcard_counts.iter() {
        sum += count;
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
