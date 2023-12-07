use regex::Regex;
use std::cmp::max;
use std::env::current_dir;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug)]
struct Game {
    pub id: i32,
    pub max_blue: i32,
    pub max_red: i32,
    pub max_green: i32,
}

fn main() {
    let lines = read_lines("./src/input.txt").expect("File should exist");
    let mut games: Vec<Game> = Vec::new();
    let mut sum = 0;

    let id_re = Regex::new(r"\d+").unwrap();

    for line in lines {
        let line = line.expect("line should be valid");
        let parts = line.split(": ").collect::<Vec<_>>();

        let id = id_re
            .find(parts[0])
            .expect("should have an id")
            .as_str()
            .parse::<i32>()
            .expect("id should be a number");

        let mut game = Game {
            id,
            max_blue: 0,
            max_red: 0,
            max_green: 0,
        };

        for set in parts[1].split("; ") {
            for cube in set.split(", ") {
                let cube_parts = cube.split(" ").collect::<Vec<_>>();
                let count = cube_parts[0]
                    .parse::<i32>()
                    .expect("count should be a number");
                let color = cube_parts[1];
                match color {
                    "blue" => {
                        game.max_blue = max(game.max_blue, count);
                    }
                    "red" => {
                        game.max_red = max(game.max_red, count);
                    }
                    "green" => {
                        game.max_green = max(game.max_green, count);
                    }
                    _ => println!("unknown color: {}", color),
                }
            }
        }

        sum += game.max_blue * game.max_red * game.max_green;

        games.push(game);
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
