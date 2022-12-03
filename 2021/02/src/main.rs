use std::{
    fs::File,
    io::{prelude::*, BufReader},
};

fn main() -> Result<(), std::io::Error> {
    let file = File::open("input")?;

    let commands: Vec<Command> = BufReader::new(file)
        .lines()
        .flatten()
        .map(Command::from)
        .collect();
    let position = commands.iter().fold((0, 0), |sums, c| match c {
        Command::Forward(amount) => (sums.0 + amount, sums.1),
        Command::Up(amount) => (sums.0, sums.1 - amount),
        Command::Down(amount) => (sums.0, sums.1 + amount),
    });

    println!(
        "The submarine is at horizontal {} and depth {}, which is {} multiplied.",
        position.0,
        position.1,
        position.0 * position.1
    );

    let position_cor = commands.iter().fold((0, 0, 0), |sums, c| match c {
        Command::Forward(amount) => (sums.0, sums.1 + amount, sums.2 + (sums.0 * amount)),
        Command::Up(amount) => (sums.0 - amount, sums.1, sums.2),
        Command::Down(amount) => (sums.0 + amount, sums.1, sums.2),
    });

    println!(
        "With the correct computations the submarine has aim {} and is at horizontal {} and depth {}, which is {} multiplied.",
        position_cor.0,
        position_cor.1,
        position_cor.2,
        position_cor.1 * position_cor.2
    );

    Ok(())
}

enum Command {
    Forward(i32),
    Down(i32),
    Up(i32),
}

impl From<String> for Command {
    fn from(cs: String) -> Self {
        let command = cs.split(' ').collect::<Vec<&str>>();
        let parts = command.as_slice();
        assert!(parts.len() == 2, "Invalid command.");
        let amount: i32 = parts[1].parse().expect("Ïnvalid command.");
        match parts[0] {
            "forward" => Self::Forward(amount),
            "up" => Self::Up(amount),
            "down" => Self::Down(amount),
            _ => panic!("Invalid command."),
        }
    }
}
