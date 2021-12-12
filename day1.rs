use std::fs::File;
use std::io::{self, BufRead, ErrorKind, Error};
use std::path::Path;

fn main() {
    let mut ans_part1 : i32 = 0;
    let mut ans_part2 : i32 = 0;
    let mut previous_num : i64 = 0;
    let mut previous_sum_3 : i64 = 0;
    let vec = read_file("input/day1.txt").unwrap();
    for index in 0..vec.len() - 2 {
        if index > 0 && vec[index] > previous_num  {
            ans_part1 = ans_part1 + 1;
        }

        let current_three_mesure = vec[index] + vec[index + 1] + vec[index + 2];

        if index > 0 && current_three_mesure > previous_sum_3 {
            ans_part2 = ans_part2 + 1;
        }

        previous_num = vec[index];
        previous_sum_3 = current_three_mesure;
    }

    println!("{}", ans_part1);
    println!("{}", ans_part2);
}


fn read_file<P>(filename :P) -> Result<Vec<i64>, Error> 
where P: AsRef<Path> {
    let file = File::open(filename)?;
    io::BufReader::new(file).lines().map(|line| line.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e))))
    .collect()
}
