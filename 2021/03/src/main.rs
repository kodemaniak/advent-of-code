use std::{
    fs::File,
    io::{prelude::*, BufReader},
};

const BITS_PER_NUMBER: usize = 12;

fn main() -> Result<(), std::io::Error> {
    let file = File::open("input")?;

    let binary_numbers: Vec<BinaryNumber> = BufReader::new(file)
        .lines()
        .flatten()
        .map(BinaryNumber::new)
        .collect();
    let bit_counts = BitCount::from_binary_numbers(&binary_numbers);

    let gamme_rate = get_gamma_rate(&bit_counts);
    let epsilon_rate = get_epsilon_rate(&bit_counts);
    let power_consumption = gamme_rate * epsilon_rate;

    println!("BitCounts: {:?}.", bit_counts);
    println!("gamma rate: {}", gamme_rate);
    println!("epsilon rate: {}", epsilon_rate);
    println!("power consumption: {}", power_consumption);

    let mut remaining = binary_numbers.clone();
    let mut position = 0;
    while remaining.len() > 1 && position < BITS_PER_NUMBER {
        dbg!(position);
        dbg!(remaining.len());
        let bit_counts = BitCount::from_binary_numbers(&remaining);
        if bit_counts.ones.get(position).unwrap() >= bit_counts.zeros.get(position).unwrap() {
            remaining = remaining
                .iter()
                .filter(|e| e.matches_at_position(position, '1'))
                .map(|e| e.to_owned())
                .collect();
        } else {
            remaining = remaining
                .iter()
                .filter(|e| e.matches_at_position(position, '0'))
                .map(|e| e.to_owned())
                .collect();
        }
        dbg!(remaining.len());

        position += 1;
    }
    let oxygen = remaining.first().unwrap().as_u32();

    let mut remaining = binary_numbers;
    let mut position = 0;
    while remaining.len() > 1 && position < BITS_PER_NUMBER {
        dbg!(position);
        dbg!(remaining.len());
        let bit_counts = BitCount::from_binary_numbers(&remaining);
        if bit_counts.ones.get(position).unwrap() >= bit_counts.zeros.get(position).unwrap() {
            remaining = remaining
                .iter()
                .filter(|e| e.matches_at_position(position, '0'))
                .map(|e| e.to_owned())
                .collect();
        } else {
            remaining = remaining
                .iter()
                .filter(|e| e.matches_at_position(position, '1'))
                .map(|e| e.to_owned())
                .collect();
        }
        position += 1;
    }
    let co2 = remaining.first().unwrap().as_u32();

    let life_support = oxygen * co2;

    println!("oxygen generator rating: {}", oxygen);
    println!("CO2 scrubber rating: {}", co2);
    println!("life support rating: {}", life_support);

    Ok(())
}

fn get_gamma_rate(bc: &BitCount) -> u32 {
    let mut rate = 0;
    let bits = bc.zeros.len();
    for (idx, (zeros, ones)) in bc.zeros.iter().zip(bc.ones.iter()).enumerate() {
        let exp = (bits - idx - 1) as u32;
        if ones > zeros {
            rate += 2_u32.pow(exp);
        }
    }

    rate
}

fn get_epsilon_rate(bc: &BitCount) -> u32 {
    let mut rate = 0;
    let bits = bc.zeros.len();
    for (idx, (zeros, ones)) in bc.zeros.iter().zip(bc.ones.iter()).enumerate() {
        let exp = (bits - idx - 1) as u32;
        if zeros > ones {
            rate += 2_u32.pow(exp);
        }
    }

    rate
}

#[derive(Debug)]
struct BitCount {
    zeros: Vec<u16>,
    ones: Vec<u16>,
}

impl BitCount {
    fn new() -> Self {
        Self {
            zeros: Vec::new(),
            ones: Vec::new(),
        }
    }

    fn from_binary_numbers(binary_numbers: &[BinaryNumber]) -> Self {
        binary_numbers
            .iter()
            .fold(BitCount::new(), |sums, bn| sums.update(bn))
    }

    fn update(mut self, num: &BinaryNumber) -> Self {
        if self.zeros.len() < num.0.len() {
            self.zeros.resize(num.0.len(), 0);
            self.ones.resize(num.0.len(), 0);
        }
        for (idx, bit) in num.0.chars().enumerate() {
            if bit == '0' {
                let new_count = self.zeros.get(idx).unwrap() + 1;
                let _ = std::mem::replace(&mut self.zeros[idx], new_count);
            } else {
                let new_count = self.ones.get(idx).unwrap() + 1;
                let _ = std::mem::replace(&mut self.ones[idx], new_count);
            }
        }

        self
    }
}

#[derive(Clone)]
struct BinaryNumber(String);

impl BinaryNumber {
    fn new(num: String) -> Self {
        Self(num)
    }

    fn as_u32(&self) -> u32 {
        let mut value = 0;
        let bits = self.0.len();
        for (idx, char) in self.0.chars().enumerate() {
            let exp = (bits - idx - 1) as u32;
            if char == '1' {
                value += 2_u32.pow(exp);
            }
        }

        value
    }

    fn matches_at_position(&self, position: usize, char: char) -> bool {
        let char_at_position: &[char] = &self.0.chars().collect::<Vec<char>>();

        println!("{:?}", char_at_position);
        dbg!(char_at_position[position]);
        dbg!(char);

        char_at_position[position] == char
    }
}
