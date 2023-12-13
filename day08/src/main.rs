use num::integer::lcm;
use regex::Regex;
use std::collections::HashMap;
use std::env::current_dir;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// (left, right)
type Node = (String, String);
type Graph = HashMap<String, Node>;

fn main() {
    let mut nodes: Graph = HashMap::new();
    let mut current_nodes: Vec<String> = Vec::new();
    let line_re = Regex::new(r"([A-Z\d]+) = \(([A-Z\d]+), ([A-Z\d]+)\)").unwrap();

    let lines = read_lines("./src/input.txt")
        .expect("File should exist")
        .map(|l| l.expect("Line should be readable"))
        .collect::<Vec<_>>();

    let directions = lines[0].clone();

    for line in &lines[2..] {
        let results = line_re
            .captures(line)
            .expect("regex should be valid per line");

        let node_label = results.get(1).unwrap().as_str();
        let left_label = results.get(2).unwrap().as_str();
        let right_label = results.get(3).unwrap().as_str();

        let last_char = node_label.chars().last().unwrap();
        if last_char == 'A' {
            current_nodes.push(node_label.to_string());
        }

        nodes.insert(
            node_label.to_string(),
            (left_label.to_string(), right_label.to_string()),
        );
    }

    let cycle_lengths = current_nodes
        .iter()
        .map(|n| get_cycle_length(n.to_string(), &nodes, directions.clone()));

    let mut steps = 1;

    for item in cycle_lengths {
        steps = lcm(steps, item);
    }

    println!("Steps: {}", steps);
}

fn get_cycle_length(start_node: String, nodes: &Graph, directions: String) -> usize {
    let mut length = 0;
    let mut current_node = start_node.clone();

    while current_node.chars().last().unwrap() != 'Z' {
        for dir in directions.chars() {
            let (left, right) = nodes.get(&current_node).unwrap();
            match dir {
                'L' => {
                    current_node = left.clone();
                }
                'R' => {
                    current_node = right.clone();
                }
                _ => panic!("Invalid direction"),
            }
            length += 1;
            if current_node.chars().last().unwrap() == 'Z' {
                break;
            }
        }
    }

    length
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

    #[test]
    fn can_detect_length() {
        let mut g: Graph = HashMap::new();
        g.insert(
            String::from("11A"),
            (String::from("11B"), String::from("XXX")),
        );
        g.insert(
            String::from("11B"),
            (String::from("XXX"), String::from("11Z")),
        );
        g.insert(
            String::from("11Z"),
            (String::from("22A"), String::from("XXX")),
        );
        g.insert(
            String::from("22A"),
            (String::from("22B"), String::from("XXX")),
        );
        g.insert(
            String::from("22B"),
            (String::from("22C"), String::from("22C")),
        );
        g.insert(
            String::from("22C"),
            (String::from("22Z"), String::from("22Z")),
        );
        g.insert(
            String::from("22Z"),
            (String::from("22B"), String::from("22B")),
        );
        g.insert(
            String::from("XXX"),
            (String::from("XXX"), String::from("XXX")),
        );

        assert_eq!(
            get_cycle_length(String::from("11A"), &g, String::from("LR")),
            2
        );
        assert_eq!(
            get_cycle_length(String::from("22A"), &g, String::from("LR")),
            3
        );
    }
}
