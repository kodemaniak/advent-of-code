use std::{
    collections::{HashSet, VecDeque},
    fs::File,
    io::{BufRead, BufReader},
    ops::RangeBounds,
};

fn main() -> Result<(), std::io::Error> {
    let file = File::open("input")?;

    let lines: Vec<String> = BufReader::new(file).lines().flatten().collect();

    part1(lines.clone());
    part2(lines);

    Ok(())
}

fn part1(lines: Vec<String>) {
    let score: u32 = lines
        .iter()
        .map(find_illegal_char)
        .flatten()
        .map(score)
        .sum();

    println!("The score for part 1 is {}", score);
}

fn part2(lines: Vec<String>) {
    let completions: Vec<(&String, VecDeque<char>)> = lines
        .iter()
        .map(|l| (l, get_incomplete_stack(l)))
        .filter(|(_, stack)| !stack.is_empty())
        .collect();
    dbg!(&completions);
    let mut scores: Vec<u64> = completions
        .iter()
        .map(|(_, s)| s.iter().map(missing_score).fold(0_u64, |r, s| r * 5 + s))
        .collect();

    scores.sort_unstable();

    scores.iter().for_each(|l| println!("{}", l));

    let middle = scores.len() / 2;

    let middle_score = scores[middle];

    println!("The score for part 2 is {}", middle_score);
}

fn find_illegal_char(line: &String) -> Option<char> {
    let mut stack = VecDeque::new();

    for char in line.chars() {
        if is_opening_bracket(char) {
            stack.push_front(char);
        } else {
            match stack.pop_front() {
                Some(head) if !matches(head, char) => return Some(char),
                _ => (),
            }
        }
    }

    None
}

fn is_opening_bracket(char: char) -> bool {
    let opening_brackets = vec!['(', '{', '[', '<'];
    opening_brackets.contains(&char)
}

fn matches(head: char, char: char) -> bool {
    let opening_brackets = vec!['(', '{', '[', '<'];
    let closing_brackets = vec![')', '}', ']', '>'];

    let (idx, _) = opening_brackets
        .iter()
        .enumerate()
        .find(|(_, char)| **char == head)
        .unwrap();

    closing_brackets[idx] == char
}

fn score(char: char) -> u32 {
    match char {
        ')' => 3,
        '}' => 1197,
        ']' => 57,
        '>' => 25137,
        _ => panic!("Unexpected char"),
    }
}

fn get_incomplete_stack(line: &String) -> VecDeque<char> {
    let mut stack = VecDeque::new();

    for char in line.chars() {
        if is_opening_bracket(char) {
            stack.push_front(char);
        } else {
            match stack.pop_front() {
                Some(head) if !matches(head, char) => return VecDeque::new(),
                _ => {}
            }
        }
    }

    stack
}

fn missing_score(opening: &char) -> u64 {
    match opening {
        '(' => 1,
        '{' => 3,
        '[' => 2,
        '<' => 4,
        _ => panic!("Unexpected char"),
    }
}
