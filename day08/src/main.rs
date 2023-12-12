use regex::Regex;
use std::collections::HashMap;
use std::env::current_dir;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut nodes: HashMap<String, (String, String)> = HashMap::new();
    let mut sum = 0;
    let line_re = Regex::new(r"([A-Z]+) = \(([A-Z]+), ([A-Z]+)\)").unwrap();

    let lines = read_lines("./src/input.txt")
        .expect("File should exist")
        .map(|l| l.expect("Line should be readable"))
        .collect::<Vec<_>>();

    let directions = lines[0].clone();

    for line in &lines[2..] {
        println!("Line: {}", line);
        let results = line_re
            .captures(line)
            .expect("regex should be valid per line");

        let node_label = results.get(1).unwrap().as_str();
        let left_label = results.get(2).unwrap().as_str();
        let right_label = results.get(3).unwrap().as_str();
        nodes.insert(
            node_label.to_string(),
            (left_label.to_string(), right_label.to_string()),
        );
    }

    let mut current_node = "AAA";

    while current_node != "ZZZ" {
        for dir in directions.chars() {
            sum += 1;
            match dir {
                'L' => {
                    current_node = nodes
                        .get(current_node)
                        .expect("node should exist")
                        .0
                        .as_str();
                }
                'R' => {
                    current_node = nodes
                        .get(current_node)
                        .expect("node should exist")
                        .1
                        .as_str();
                }
                _ => {
                    panic!("Unknown direction: {}", dir);
                }
            }
        }
    }

    println!("directions: {:?}", directions);
    println!("Nodes: {:?}", nodes);

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

#[cfg(test)]
mod test {
    use super::*;
}
