use regex::Regex;
use std::collections::HashMap;
use std::env::current_dir;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::vec;

fn main() {
    let mut groups: HashMap<(usize, usize), Vec<i32>> = HashMap::new();
    let mut sum = 0;
    let number_re = Regex::new(r"(\d+)").expect("Regex should be valid");

    let lines = read_lines("./src/input.txt")
        .expect("File should exist")
        .map(|l| l.expect("Line should be readable"))
        .collect::<Vec<_>>();

    println!("{:?}", lines);
    // return;

    for (row_index, line) in lines.iter().enumerate() {
        for mtch in number_re.find_iter(line.as_str()) {
            let number = mtch.as_str().parse::<i32>().expect("should be a number");

            let start_x = mtch.start().saturating_sub(1);
            // end() index is offset by +1
            let end_x = mtch.end();
            let start_y = row_index.saturating_sub(1);
            let end_y = row_index + 1;

            for y in start_y..=end_y {
                for x in start_x..=end_x {
                    if y < lines.len() {
                        let char = lines[y].chars().nth(x);
                        if char == Some('*') {
                            groups.entry((x, y)).or_insert(vec![]).push(number);
                        }
                    }
                }
            }
        }
    }

    for (_, group) in groups.iter() {
        if group.len() == 2 {
            sum += group[0] * group[1];
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
