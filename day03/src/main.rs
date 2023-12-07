use regex::Regex;
use std::collections::HashSet;
use std::env::current_dir;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut sparse_map: Vec<HashSet<usize>> = Vec::new();
    let mut sum = 0;
    let number_re = Regex::new(r"(\d+)").expect("Regex should be valid");

    let lines = read_lines("./src/input.txt").expect("File should exist");

    for line in lines {
        let line = line.expect("line should be valid");
        let mut row: HashSet<usize> = HashSet::new();
        for (index, char) in line.chars().enumerate() {
            if char != '.' && !char.is_digit(16) {
                println!("{}: {}", index, char);
                row.insert(index);
            }
        }
        sparse_map.push(row);
    }

    let lines = read_lines("./src/input.txt").expect("File should exist");

    for (row_index, line) in lines.enumerate() {
        let line = line.expect("line should be valid");
        for mtch in number_re.find_iter(line.as_str()) {
            let number = mtch.as_str().parse::<i32>().expect("should be a number");
            let start_index = mtch.start();
            // end index is offset by +1
            let end_index = mtch.end();
            let mut is_adjacent = false;

            for n in start_index..end_index {
                println!("{}", n);
                let current_row = &sparse_map[row_index];
                if (n > 0 && current_row.contains(&(n - 1)))
                    || (n < (line.len() - 1) && current_row.contains(&(n + 1)))
                {
                    is_adjacent = true;
                    break;
                }
                if row_index > 0 {
                    let top_row = &sparse_map[row_index - 1];
                    if top_row.contains(&n)
                        || (n > 0 && top_row.contains(&(n - 1)))
                        || (n < (line.len() - 1) && top_row.contains(&(n + 1)))
                    {
                        is_adjacent = true;
                        break;
                    }
                }
                if row_index < (sparse_map.len() - 1) {
                    let bottom_row = &sparse_map[row_index + 1];
                    if bottom_row.contains(&n)
                        || (n > 0 && bottom_row.contains(&(n - 1)))
                        || (n < (line.len() - 1) && bottom_row.contains(&(n + 1)))
                    {
                        is_adjacent = true;
                        break;
                    }
                }
            }
            println!(
                "{} at {} ending at {}, is: {}",
                number, start_index, end_index, is_adjacent
            );

            if is_adjacent {
                sum += number;
            }
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
