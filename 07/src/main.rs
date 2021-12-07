use std::{
    fs::File,
    io::{prelude::*, BufReader},
};

fn main() -> Result<(), std::io::Error> {
    let file = File::open("input")?;

    let line = BufReader::new(file).lines().flatten().next().unwrap();
    let positions: Vec<i64> = line.split(',').map(|f| f.parse::<i64>().unwrap()).collect();

    let min_position = positions.iter().min().unwrap().to_owned();
    let max_position = positions.iter().max().unwrap().to_owned();

    // part 1
    let mut min_fuel_cost = i64::MAX;
    let mut best_target_position = 0;
    for target_position in min_position..=max_position {
        let offset_positions = positions.iter().map(|p| (p - target_position).abs());
        let fuel_cost: i64 = offset_positions.sum();
        if fuel_cost < min_fuel_cost {
            min_fuel_cost = fuel_cost;
            best_target_position = target_position;
        }
    }

    println!(
        "Best target position {} with fuel cost {}.",
        best_target_position, min_fuel_cost
    );

    // part 2
    let mut min_fuel_cost = i64::MAX;
    let mut best_target_position = 0;
    for target_position in min_position..=max_position {
        let offset_positions = positions.iter().map(|p| (p - target_position).abs());
        let fuel_cost: i64 = offset_positions.map(|d| d * (d + 1) / 2).sum();
        if fuel_cost < min_fuel_cost {
            min_fuel_cost = fuel_cost;
            best_target_position = target_position;
        }
    }

    println!(
        "Best target position {} with correct fuel cost {}.",
        best_target_position, min_fuel_cost
    );

    Ok(())
}
