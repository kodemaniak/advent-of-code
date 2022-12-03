use anyhow::Result;
use std::{
    fs::File,
    io::{prelude::*, BufReader},
};

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();

    let file = File::open(&args[1])?;
    let sum: u16 = BufReader::new(file)
        .lines()
        .flatten()
        .map(|mut l| {
            let middle = l.len() / 2;
            let second = l.split_off(middle);
            (l, second)
        })
        .map(|(c1, c2)| find_error_in_line(c1, c2))
        .map(|c| priority(c))
        .sum();

    println!("sum prio item types: {}", sum);

    let file = File::open(&args[1])?;
    let lines: Vec<String> = BufReader::new(file).lines().flatten().collect();
    let sum: u16 = lines
        .chunks(3)
        .map(|chunks| find_badge(chunks))
        .map(|c| priority(c))
        .sum();

    println!("sum prio badges: {}", sum);

    Ok(())
}

fn find_error_in_line(c1: String, c2: String) -> char {
    for i1 in c1.chars() {
        for i2 in c2.chars() {
            if i1 == i2 {
                return i1;
            }
        }
    }

    unreachable!("Wrong input!")
}

fn priority(c: char) -> u16 {
    if c.is_lowercase() {
        (c as u16) - 96
    } else {
        (c as u16) - 38
    }
}

fn find_badge(chunks: &[String]) -> char {
    assert!(chunks.len() == 3);

    for i1 in chunks[0].chars() {
        for i2 in chunks[1].chars() {
            if i1 == i2 {
                for i3 in chunks[2].chars() {
                    if i3 == i2 {
                        return i1;
                    }
                }
            }
        }
    }

    unreachable!("Wrong input!")
}
