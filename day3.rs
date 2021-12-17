use std::fs::File;
use std::io::{self, BufRead, ErrorKind, Error};
use std::path::Path;
use std::isize;

fn main() {
    let input : Vec<String> = read_file("input/day3.txt").unwrap();
    let string_size = input[0].len();

    // PART 1
    let mut nb_bit_one : Vec<i32> = vec![0; string_size];
    let mut nb_bit_zero : Vec<i32> = vec![0; string_size];
    for string in &input {
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
    let res_part1 = gamma * eps;
    println!("{} {} {}", res_part1, gamma, eps);

    // PART 2
    // DO it stack-like (do Rust has stack (?)). Eliminate until stack has only 1 value left
    // oxygen part
    let mut input_oxygen : Vec<String> = input.clone();
    let mut index = 0;
    while index < string_size && input_oxygen.len() > 1 {
        // same as part 1, counting nb of 1 in each index
        let mut nb_bit_one_current_index : i32 = 0;
        let mut nb_bit_zero_current_index : i32 = 0;
        for string in &input_oxygen {
            let chars : Vec<char> = string.chars().collect();
            if chars[index] ==  '0' {
                nb_bit_zero_current_index += 1;
            } else {
                nb_bit_one_current_index += 1;
            }
        }

        let char_to_keep = if nb_bit_zero_current_index > nb_bit_one_current_index { '0' } else { '1' };
        let mut idx = 0;
        while idx < input_oxygen.len() {
            let chars : Vec<char> = input_oxygen[idx].chars().collect();
            if chars[index] != char_to_keep {
                input_oxygen.remove(idx);
                continue;
            }

            idx = idx + 1;
        }

        index += 1;
    }

    let mut input_co2 : Vec<String> = input.clone();
    index = 0;
    while index < string_size && input_co2.len() > 1 {
        // same as part 1, counting nb of 1 in each index
        let mut nb_bit_one_current_index : i32 = 0;
        let mut nb_bit_zero_current_index : i32 = 0;
        for string in &input_co2 {
            let chars : Vec<char> = string.chars().collect();
            if chars[index] ==  '0' {
                nb_bit_zero_current_index += 1;
            } else {
                nb_bit_one_current_index += 1;
            }
        }

        let char_to_keep = if nb_bit_zero_current_index > nb_bit_one_current_index { '1' } else { '0' };
        let mut idx = 0;
        while idx < input_co2.len() {
            let chars : Vec<char> = input_co2[idx].chars().collect();
            if chars[index] != char_to_keep {
                input_co2.remove(idx);
                continue;
            }

            idx = idx + 1;
        }

        index += 1;
    }

    let oxygen  = isize::from_str_radix(&input_oxygen[0], 2).unwrap();
    let co2  = isize::from_str_radix(&input_co2[0], 2).unwrap();
    let res_part2 = oxygen * co2;
    println!("{} {} {}", res_part2, oxygen, co2);

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
