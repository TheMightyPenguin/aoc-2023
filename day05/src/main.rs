use std::cmp::{max, min};
use std::env::current_dir;
use std::fs::File;
use std::io::{self, BufRead};
use std::ops::Range;
use std::path::Path;

#[derive(Debug)]
struct RangeMap {
    destination_range: Range<i128>,
    source_range: Range<i128>,
}

#[derive(Debug)]
struct MapGroup {
    name: String,
    ranges: Vec<RangeMap>,
}

trait RangeOverlap<T> {
    fn overlaps(&self, other: &Range<T>) -> bool;
}

impl RangeOverlap<i128> for Range<i128> {
    fn overlaps(&self, other: &Range<i128>) -> bool {
        self.start < other.end && self.end > other.start
    }
}

fn main() {
    let lines = read_lines("./src/input.txt")
        .expect("File should exist")
        .map(|l| l.expect("Line should be readable"))
        .collect::<Vec<_>>();

    let seeds_data = lines[0]
        .split(": ")
        .nth(1)
        .expect("seeds should be present")
        .split(" ")
        .map(|s| s.parse::<i128>().expect("seeds should be numbers"))
        .collect::<Vec<_>>();

    let mut seeds = parse_seeds(seeds_data);
    let groups = parse_groups(&lines[2..]);

    println!("Seeds: {:?}", seeds);
    println!("Groups: {:?}", groups);

    split_groups(&groups, &mut seeds);

    println!("seeds: {:?}", seeds);
    let min_seed = seeds.iter().map(|r| r.start).min().unwrap();
    println!("min: {}", min_seed);
}

fn split_groups(groups: &Vec<MapGroup>, seeds: &mut Vec<Range<i128>>) {
    for group in groups {
        let mut index = 0;
        while index < seeds.len() {
            let seed = &seeds[index];
            for range in &group.ranges {
                if seed.overlaps(&range.source_range) {
                    let diff = range.source_range.start - range.destination_range.start;
                    let new_start = max(seed.start, range.source_range.start);
                    let new_end = min(seed.end, range.source_range.end);
                    let new_range = new_start..new_end;

                    let cloned_seed = seed.clone();
                    if new_range.start > cloned_seed.start {
                        seeds.push(cloned_seed.start..new_range.start);
                    }
                    if new_range.end < cloned_seed.end {
                        seeds.push(new_range.end..cloned_seed.end);
                    }

                    seeds[index] = (new_range.start - diff)..(new_range.end - diff);
                    break;
                }
                // else seed range is already in the destination range
            }
            index += 1;
        }
    }
}

fn parse_seeds(data: Vec<i128>) -> Vec<Range<i128>> {
    let mut index = 0;
    let mut seeds: Vec<Range<i128>> = vec![];
    while index < data.len() {
        seeds.push(data[index]..(data[index] + data[index + 1]));
        index += 2;
    }
    seeds
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
                    .map(|s| s.parse::<i128>().expect("range items should be numbers"))
                    .collect::<Vec<_>>();
                let destination_start = items[0];
                let source_start = items[1];
                let range_length = items[2];
                group.ranges.push(RangeMap {
                    destination_range: destination_start..(destination_start + range_length),
                    source_range: source_start..(source_start + range_length),
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

#[cfg(test)]
mod test {
    use super::{split_groups, MapGroup, RangeMap};

    #[test]
    fn all_inside_seed() {
        let mut seeds = vec![0..10, 5..6, 4..7];
        let groups = vec![MapGroup {
            name: String::from("g1"),
            ranges: vec![RangeMap {
                destination_range: 10..20,
                source_range: 0..10,
            }],
        }];

        split_groups(&groups, &mut seeds);

        assert_eq!(seeds, vec![10..20, 15..16, 14..17]);
    }

    #[test]
    fn destination_range_smaller() {
        let mut seeds = vec![12..14, 10..20];
        let groups = vec![MapGroup {
            name: String::from("g1"),
            ranges: vec![RangeMap {
                destination_range: 0..10,
                source_range: 10..20,
            }],
        }];

        split_groups(&groups, &mut seeds);

        assert_eq!(seeds, vec![2..4, 0..10]);
    }

    #[test]
    fn passes_non_matching_seeds() {
        let mut seeds = vec![0..9, 20..21, 100..200, 9..10];
        let groups = vec![MapGroup {
            name: String::from("g1"),
            ranges: vec![RangeMap {
                destination_range: 0..10,
                source_range: 10..20,
            }],
        }];

        split_groups(&groups, &mut seeds);

        assert_eq!(seeds, vec![0..9, 20..21, 100..200, 9..10]);
    }

    #[test]
    fn seed_missing_left_side() {
        let mut seeds = vec![8..15, 2..20, 9..10];
        let groups = vec![MapGroup {
            name: String::from("g1"),
            ranges: vec![RangeMap {
                destination_range: 0..10,
                source_range: 10..20,
            }],
        }];

        split_groups(&groups, &mut seeds);

        assert_eq!(seeds, vec![0..5, 0..10, 9..10, 8..10, 2..10]);
    }

    #[test]
    fn seed_missing_right_side() {
        let mut seeds = vec![10..21, 20..21, 15..25];
        let groups = vec![MapGroup {
            name: String::from("g1"),
            ranges: vec![RangeMap {
                destination_range: 0..10,
                source_range: 10..20,
            }],
        }];

        split_groups(&groups, &mut seeds);

        assert_eq!(seeds, vec![0..10, 20..21, 5..10, 20..21, 20..25]);
    }

    #[test]
    fn seed_missing_both_sides() {
        let mut seeds = vec![8..23];
        let groups = vec![MapGroup {
            name: String::from("g1"),
            ranges: vec![RangeMap {
                destination_range: 0..10,
                source_range: 10..20,
            }],
        }];

        split_groups(&groups, &mut seeds);

        assert_eq!(seeds, vec![0..10, 8..10, 20..23]);
    }
}
