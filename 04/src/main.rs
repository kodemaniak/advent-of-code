use std::{
    fs::File,
    io::{prelude::*, BufReader},
};

fn main() -> Result<(), std::io::Error> {
    let file = File::open("input")?;

    let mut lines = BufReader::new(file).lines().flatten();
    let drawn_numbers = lines.next().unwrap();
    let drawn_numbers: Vec<u8> = drawn_numbers
        .split(',')
        .map(|c| c.parse::<u8>().unwrap())
        .collect();

    let mut boards = Vec::new();
    let mut current_board = 0;
    for line in lines {
        if line.trim().is_empty() {
            current_board += 1;
            boards.push(Board::new());
        } else {
            let split: Vec<&str> = line.split_whitespace().collect();
            let mut nums: Vec<u8> = split.iter().map(|n| n.parse::<u8>().unwrap()).collect();
            boards[current_board - 1].append(&mut nums);
        }
    }

    let mut first_winner_score: Option<u32> = None;
    let mut last_winner_score: Option<u32> = None;
    for draw in drawn_numbers.iter() {
        for board in boards.iter_mut().filter(|b| !b.is_finished()) {
            board.update(*draw);
        }

        if let Some((_idx, winner)) = boards.iter().enumerate().find(|(_, b)| b.is_finished()) {
            let score = winner.score();
            let final_score = score * *draw as u32;
            if first_winner_score.is_none() {
                first_winner_score.replace(final_score);
            }

            last_winner_score.replace(final_score);
        }

        boards = boards
            .iter()
            .filter(|b| !b.is_finished())
            .map(|b| b.to_owned())
            .collect();
    }

    println!(
        "First board to win with score: {}",
        first_winner_score.unwrap()
    );
    println!(
        "Last board to win with score: {}",
        last_winner_score.unwrap()
    );

    Ok(())
}

#[derive(Clone, Debug)]
struct Board {
    numbers: Vec<u8>,
    marks: Vec<bool>,
}

impl Board {
    fn new() -> Self {
        Self {
            numbers: Vec::new(),
            marks: Vec::new(),
        }
    }

    fn append(&mut self, numbers: &mut Vec<u8>) {
        self.numbers.append(numbers);
        self.marks.resize(self.numbers.len(), false);
    }

    fn update(&mut self, draw: u8) {
        for (idx, num) in self.numbers.iter().enumerate() {
            if num == &draw {
                self.marks[idx] = true;
            }
        }
    }

    fn is_finished(&self) -> bool {
        self.marks
            .chunks(5)
            .enumerate()
            .any(|(_, chunk)| chunk.iter().all(|f| *f))
    }

    fn score(&self) -> u32 {
        let unmarked = self
            .numbers
            .iter()
            .zip(self.marks.iter())
            .filter(|(_, mark)| !(**mark))
            .map(|e| *e.0 as u32);

        unmarked.sum()
    }
}
