use std::fs::File;
use std::io::{self, BufRead, ErrorKind, Error};
use std::path::Path;

fn main() {
    let input : Vec<String> = read_file("input/day3.txt").unwrap();
    let string_size = input[0].len();
    let mut nb_bit_one : Vec<i32> = vec![0; string_size];
    let mut nb_bit_zero : Vec<i32> = vec![0; string_size];
    for string in input {
        for (index, c) in string.chars().enumerate() {
            if c ==  '0' {
                nb_bit_zero[index] += 1;
            } else {
                nb_bit_one[index] += 1;
            }
        }
    }
    let mut gamma_rate : Vec<i32> = Vec::new();
    let mut eps_rate : Vec<i32> = Vec::new();
    for index in 0..string_size {
        if nb_bit_zero[index] > nb_bit_one[index] {
            gamma_rate.push(0);
            eps_rate.push(1);
        } else {
            gamma_rate.push(1);
            eps_rate.push(0);
        }
    }

    for index in 0..string_size {
        print!("{}", gamma_rate[index]);
    }
    println!();

    for index in 0..string_size {
        print!("{}", eps_rate[index]);
    }
    println!();
    
    let gamma = convert_binary_decimal(gamma_rate);
    let eps = convert_binary_decimal(eps_rate);
    let res = gamma * eps;
    println!("{} {} {}", res, gamma, eps);
}


fn read_file<P>(filename :P) -> Result<Vec<String>, Error> 
where P: AsRef<Path> {
    let file = File::open(filename)?;
    io::BufReader::new(file).lines().map(|line| line.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e))))
    .collect()
}


fn convert_binary_decimal(input : Vec<i32>) -> i32 {
    let mut output = 0;
    let mut base = 1;
    for index in (0..input.len()).rev() {
        output += input[index] *  base;
        base *= 2;
    }

    output
}