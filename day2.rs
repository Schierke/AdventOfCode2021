use std::fs::File;
use std::io::{self, BufRead, ErrorKind, Error};
use std::path::Path;
use std::str;

fn main() {
    let mut horizontal_pos : i64 = 0;
    let mut vertical_pos : i64 = 0;
    let mut depth : i64 = 0;
    let vec = read_file("input/day2.txt").unwrap();
    for item in vec {
        let mut split = item.split(" ");
        let direction = split.next().unwrap();
        let unit_str = split.next().unwrap();
        let unit : i64 = unit_str.parse::<i64>().unwrap();
        calculate_horizontal_pos(&mut horizontal_pos, vertical_pos, &mut depth, direction, unit);
        calculate_depth_pos(&mut vertical_pos, direction, unit);
    }

    let ans_part1 = horizontal_pos * vertical_pos;
    let ans_part2 = horizontal_pos * depth;
    println!("{} {}", ans_part1, ans_part2);
}


fn read_file<P>(filename :P) -> Result<Vec<String>, Error> 
where P: AsRef<Path> {
    let file = File::open(filename)?;
    io::BufReader::new(file).lines().map(|line| line.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e))))
    .collect()
}

fn calculate_horizontal_pos (pos : &mut i64, vertical_pos : i64, depth: &mut i64, direction : &str, unit: i64) {
    if direction.eq("forward") {
        *depth = *depth + vertical_pos * unit;
        *pos = *pos + unit;
    }
}

fn calculate_depth_pos (pos : &mut i64, direction : &str, unit: i64) {
    if direction == "up" {
        *pos = *pos - unit;
    } else if direction == "down"{
        *pos = *pos + unit;
    }
}