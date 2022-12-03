use anyhow::Result;
use std::{
    fs::File,
    io::{prelude::*, BufReader},
};

fn main() -> Result<()> {
    let file = File::open("input")?;

    let lines: Vec<u16> = BufReader::new(file)
        .lines()
        .flatten()
        .map(|s| s.parse::<u16>())
        .flatten()
        .collect();
    let windows = lines.windows(2);
    let increases: u32 = windows.map(|x| if x[1] > x[0] { 1 } else { 0 }).sum();
    println!("The depth measurement incresd {} times!", increases);

    let sums: Vec<u16> = lines.windows(3).map(|x| x.iter().sum()).collect();
    let sum_increases: u32 = sums
        .windows(2)
        .map(|x| if x[1] > x[0] { 1 } else { 0 })
        .sum();
    println!(
        "The sums of depth measurement incresd {} times!",
        sum_increases
    );

    Ok(())
}
