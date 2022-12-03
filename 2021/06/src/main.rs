use std::{
    fs::File,
    io::{prelude::*, BufReader},
};

fn main() -> Result<(), std::io::Error> {
    let file = File::open("input")?;

    let line = BufReader::new(file).lines().flatten().next().unwrap();
    let mut population: Vec<(u8, u64)> = line
        .split(',')
        .map(|f| (f.parse::<u8>().unwrap(), 1))
        .collect();

    for _ in 0..80 {
        let mut spawned_fish = 0_u64;
        for (fish, count) in population.iter_mut() {
            if *fish == 0 {
                spawned_fish += *count;
                *fish = 6;
            } else {
                *fish -= 1;
            }
        }
        population.push((8, spawned_fish));
    }

    let pop_80: u64 = population.iter().map(|(_, count)| count).sum();
    println!("Number of fish after 80 days: {}", pop_80);

    for _ in 0..176 {
        let mut spawned_fish = 0_u64;
        for (fish, count) in population.iter_mut() {
            if *fish == 0 {
                spawned_fish += *count;
                *fish = 6;
            } else {
                *fish -= 1;
            }
        }
        population.push((8, spawned_fish));
    }

    let pop_256: u64 = population.iter().map(|(_, count)| count).sum();
    println!("Number of fish after 256 days: {}", pop_256);

    Ok(())
}
