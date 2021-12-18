use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

fn main() -> Result<(), std::io::Error> {
    let file = File::open("input")?;

    let mut lines = BufReader::new(file).lines().flatten();
    let template = lines.next().unwrap();
    let insert_rules = lines.skip(1);
    let mut map = HashMap::new();
    for rule in insert_rules {
        let parts: Vec<&str> = rule.split(" -> ").collect();
        let key = String::from_str(parts[0]).unwrap();
        let val = String::from_str(parts[1]).unwrap();
        map.insert(key, val);
    }

    part1(&template, &map);
    part2(&template, &map);

    Ok(())
}

#[allow(clippy::ptr_arg)]
fn part1(template: &String, insert_rules: &HashMap<String, String>) {
    let mut current_template = template.clone();
    for _ in 0..10 {
        let pairs: Vec<String> = current_template
            .chars()
            .collect::<Vec<char>>()
            .windows(2)
            .map(|pair| {
                let key = format!("{}{}", pair[0], pair[1]);
                let insert = insert_rules.get(&key).unwrap();
                format!("{}{}", pair[0], insert)
            })
            .collect();
        let last_element = template.chars().last().unwrap();
        let output = pairs.join("");
        current_template = format!("{}{}", output, last_element);
        println!("{}", current_template);
    }

    let mut counts = HashMap::new();
    for c in current_template.chars() {
        match counts.get_mut(&c) {
            Some(c) => *c += 1,
            None => {
                counts.insert(c, 1);
            }
        }
    }
    dbg!(&counts);
    let count_values: Vec<i32> = counts.into_values().collect();
    let max = count_values.iter().max().unwrap();
    let min = count_values.iter().min().unwrap();
    let result = max - min;
    println!("Result: {}", result);
}

#[allow(clippy::ptr_arg)]
fn part2(template: &String, insert_rules: &HashMap<String, String>) {
    let last = template.chars().last().unwrap();
    let current_template: Vec<(char, char)> = template
        .clone()
        .chars()
        .collect::<Vec<char>>()
        .windows(2)
        .map(|cs| {
            let l = cs[0];
            let r = cs[1];
            (l, r)
        })
        .collect();
    let mut counts = HashMap::new();
    for pair in current_template {
        *counts.entry(pair).or_insert(0_u64) += 1_u64;
    }
    for _ in 0..40 {
        let mut new_counts = HashMap::new();
        for ((l, r), count) in counts {
            let key = format!("{}{}", l, r);
            let insert = insert_rules
                .get(&key)
                .unwrap()
                .chars()
                .collect::<Vec<char>>()[0];
            *new_counts.entry((l, insert)).or_insert(0_u64) += count;
            *new_counts.entry((insert, r)).or_insert(0_u64) += count;
        }
        counts = new_counts;
    }

    let mut element_counts: HashMap<char, u64> = HashMap::new();
    for ((l, _r), count) in counts.iter() {
        *element_counts.entry(*l).or_insert(0_u64) += count;
        // *element_counts.entry(*r).or_insert(0_u64) += count;
    }
    *element_counts.entry(last).or_insert(0) += 1;
    dbg!(&element_counts);
    let count_values: Vec<u64> = element_counts.into_values().collect();
    let max = count_values.iter().max().unwrap();
    let min = count_values.iter().min().unwrap();
    let result = max - min;
    println!("Result: {}", result);
}
