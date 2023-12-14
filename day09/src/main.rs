use std::env::current_dir;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let lines = read_lines("./src/input.txt")
        .expect("File should exist")
        .map(|l| l.expect("Line should be readable"))
        .collect::<Vec<_>>();

    let sequence_groups = lines
        .iter()
        .map(|line| {
            line.split_whitespace()
                .map(|i| i.parse::<i64>().expect("number should be valid"))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    println!("sequence_groups: {:?}", sequence_groups);

    let sum = sequence_groups
        .iter()
        .map(|seq| get_sequence_value(seq.clone()))
        .fold(0, |a, b| a + b);

    println!("sum {:?}", sum);
}

fn get_sequence_value(first_seq: Vec<i64>) -> i64 {
    let mut sequences = vec![first_seq.clone()];
    let mut last_seq = &sequences[sequences.len() - 1];
    while last_seq.iter().any(|n| *n != 0) {
        let mut new_seq: Vec<i64> = vec![];
        let mut index = 1;
        while index < last_seq.len() {
            let val = last_seq[index] - last_seq[index - 1];
            new_seq.push(val);
            index += 1;
        }
        sequences.push(new_seq);
        last_seq = &sequences[sequences.len() - 1];
    }
    let last_seq = sequences.last_mut().unwrap();
    last_seq.push(0);

    
    for index in (0..(sequences.len() - 1)).rev() {
        let prev_last = *sequences[index + 1].last().unwrap();
        let current_last = *sequences[index].last().unwrap();
        sequences[index].push(prev_last + current_last);
    }
    println!("sequences after processing {:?}", sequences);

    let val = *sequences.first().unwrap().last().unwrap();
    val
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
