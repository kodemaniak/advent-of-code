use std::{
    fs::File,
    io::{prelude::*, BufReader},
    str::FromStr,
};

fn main() -> Result<(), std::io::Error> {
    let file = File::open("input")?;

    let lines: Vec<String> = BufReader::new(file).lines().flatten().collect();

    part1(lines.clone());
    part2(lines);

    Ok(())
}

fn part1(lines: Vec<String>) {
    let parts = lines.iter().map(parse_line);

    let outputs = parts.map(|p| p.1);

    let num_unique_patterns = outputs
        .map(parse_output)
        .flatten()
        .map(|p| p.len())
        .filter(|l| *l == 2 || *l == 4 || *l == 3 || *l == 7)
        .count();

    println!("Number of unique patterns: {}", num_unique_patterns);
}

fn part2(lines: Vec<String>) {
    let parts = lines.iter().map(parse_line);

    let outputs = parts.map(|p| derive_display(p.0, p.1));

    let sum: u32 = outputs.sum();

    println!("Sum of outputs: {}", sum);
}

fn derive_display(random: String, output: String) -> u32 {
    let random_patterns: Vec<&str> = random.split(' ').collect();
    let mut digits: Vec<&str> = output.split(' ').collect();

    let mut bitmask_1 = 0b00000000;
    let mut bitmask_4 = 0b00000000;
    let mut bitmask_7 = 0b00000000;
    let mut bitmask_8 = 0b00000000;
    for pattern in random_patterns.iter() {
        if pattern.len() == 2 {
            bitmask_1 = pattern.chars().map(bitmask_for_char).fold(0, |m, c| m | c);
        } else if pattern.len() == 4 {
            bitmask_4 = pattern.chars().map(bitmask_for_char).fold(0, |m, c| m | c);
        } else if pattern.len() == 3 {
            bitmask_7 = pattern.chars().map(bitmask_for_char).fold(0, |m, c| m | c);
        } else if pattern.len() == 7 {
            bitmask_8 = pattern.chars().map(bitmask_for_char).fold(0, |m, c| m | c);
        }
    }
    let mut bitmask_0 = 0b00000000;
    let mut bitmask_2 = 0b00000000;
    let mut bitmask_3 = 0b00000000;
    let mut bitmask_5 = 0b00000000;
    let mut bitmask_6 = 0b00000000;
    let mut bitmask_9 = 0b00000000;
    for pattern in random_patterns {
        if pattern.len() == 5 {
            let bitmask = pattern.chars().map(bitmask_for_char).fold(0, |m, c| m | c);
            if bitmask & bitmask_1 == bitmask_1 {
                bitmask_3 = bitmask;
            } else if (bitmask | bitmask_1) & bitmask_4 == bitmask_4 {
                bitmask_5 = bitmask;
            } else {
                bitmask_2 = bitmask;
            }
        } else if pattern.len() == 6 {
            let bitmask = pattern.chars().map(bitmask_for_char).fold(0, |m, c| m | c);
            if bitmask & bitmask_1 != bitmask_1 {
                bitmask_6 = bitmask;
            } else if bitmask & bitmask_4 == bitmask_4 {
                bitmask_9 = bitmask;
            } else {
                bitmask_0 = bitmask;
            }
        }
    }

    digits.reverse();
    let c = digits.iter().enumerate().map(|(idx, digit)| {
        let mask = digit.chars().map(bitmask_for_char).fold(0, |m, c| m | c);

        let num = if mask == bitmask_0 {
            0
        } else if mask == bitmask_1 {
            1
        } else if mask == bitmask_2 {
            2
        } else if mask == bitmask_3 {
            3
        } else if mask == bitmask_4 {
            4
        } else if mask == bitmask_5 {
            5
        } else if mask == bitmask_6 {
            6
        } else if mask == bitmask_7 {
            7
        } else if mask == bitmask_8 {
            8
        } else if mask == bitmask_9 {
            9
        } else {
            panic!("Unexpected pattern.");
        };

        num * 10_u32.pow(idx as u32)
    });

    c.sum()
}

fn bitmask_for_char(c: char) -> u32 {
    match c {
        'a' => 1,
        'b' => 2,
        'c' => 4,
        'd' => 8,
        'e' => 16,
        'f' => 32,
        'g' => 64,
        _ => panic!("unexpected char"),
    }
}

#[allow(clippy::ptr_arg)]
fn parse_line(line: &String) -> (String, String) {
    let parts: Vec<String> = line
        .split('|')
        .map(|p| p.trim())
        .map(|p| String::from_str(p))
        .flatten()
        .collect();
    assert_eq!(parts.len(), 2);

    (parts[0].to_owned(), parts[1].to_owned())
}

fn parse_output(line: String) -> Vec<String> {
    let output: Vec<String> = line
        .split(' ')
        .map(|p| p.trim())
        .map(|p| String::from_str(p))
        .flatten()
        .collect();
    assert_eq!(output.len(), 4);

    output
}
