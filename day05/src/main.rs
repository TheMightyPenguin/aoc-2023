use std::env::current_dir;
use std::fs::File;
use std::io::{self, BufRead};
use std::ops::Range;
use std::path::Path;

#[derive(Debug)]
struct RangeMap {
    destination_range: Range<usize>,
    sourge_range: Range<usize>,
}

#[derive(Debug)]
struct MapGroup {
    name: String,
    ranges: Vec<RangeMap>,
}

fn main() {
    let lines = read_lines("./src/input.txt")
        .expect("File should exist")
        .map(|l| l.expect("Line should be readable"))
        .collect::<Vec<_>>();

    let mut seeds = lines[0]
        .split(": ")
        .nth(1)
        .expect("seeds should be present")
        .split(" ")
        .map(|s| s.parse::<usize>().expect("seeds should be numbers"))
        .collect::<Vec<_>>();

    let groups = parse_groups(&lines[2..]);

    println!("Seeds: {:?}", seeds);
    println!("Groups: {:?}", groups);

    for group in groups {
        let mut index = 0;
        while index < seeds.len() {
            let seed = seeds[index];
            for range in &group.ranges {
                if range.sourge_range.contains(&seed) {
                    // simplified formula for converting between 2 ranges, because new and old range have the same distance
                    // https://stackoverflow.com/a/929107
                    let destination =
                        seed - range.sourge_range.start + range.destination_range.start;
                    seeds[index] = destination;
                    break;
                }
                // else seed is already in the destination value
            }
            index += 1;
        }
    }

    println!("seeds: {:?}", seeds);
    let min_seed = seeds.iter().min().unwrap();
    println!("min: {}", min_seed);
}

fn parse_groups(lines: &[String]) -> Vec<MapGroup> {
    let mut groups: Vec<MapGroup> = vec![];
    let mut index = 0;

    while index < lines.len() {
        let mut line = lines[index].trim();
        let mut group = MapGroup {
            name: String::from(""),
            ranges: vec![],
        };
        while line.len() > 0 {
            if line.contains("map:") {
                group.name = line
                    .split_whitespace()
                    .nth(0)
                    .expect("name should be present")
                    .to_string();
            } else {
                let items = line
                    .split_whitespace()
                    .map(|s| s.parse::<usize>().expect("range items should be numbers"))
                    .collect::<Vec<_>>();
                let destination_start = items[0];
                let source_start = items[1];
                let range_length = items[2];
                group.ranges.push(RangeMap {
                    destination_range: destination_start..(destination_start + range_length),
                    sourge_range: source_start..(source_start + range_length),
                });
            }
            index += 1;
            if index >= lines.len() {
                break;
            }
            line = lines[index].trim();
        }
        groups.push(group);
        index += 1;
    }

    groups
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
