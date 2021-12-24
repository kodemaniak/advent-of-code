use std::collections::HashMap;
use std::iter::FromIterator;
use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> Result<(), std::io::Error> {
    let file = File::open("input")?;

    let lines: Vec<String> = BufReader::new(file).lines().flatten().collect();

    let grid = Grid::from_lines(lines);

    part1(&grid);
    part2(grid);

    Ok(())
}

fn part1(grid: &Grid) {
    let total_risk_level: u32 = grid
        .get_low_points()
        .iter()
        .map(|(x, y)| grid.get_risk_level(*x, *y))
        .sum();

    println!("The total risk level is {}", total_risk_level);
}

fn part2(mut grid: Grid) {
    let low_points = grid.get_low_points();

    let mut basin_sizes = Vec::new();

    dbg!(&low_points);

    for (x, y) in low_points.iter() {
        grid.clear_marks();
        grid.mark_basin(*x, *y);
        basin_sizes.push((grid.clone(), grid.get_basin_size()));
    }

    basin_sizes.sort_by(|a, b| a.1.cmp(&b.1));
    basin_sizes.reverse();

    let top3: Vec<(Grid, usize)> = basin_sizes.into_iter().take(3).collect();

    top3.iter().for_each(|(grid, _)| {
        grid.print();
        println!()
    });

    let result = top3.into_iter().fold(1, |e, (_, s)| e * s);

    println!("Result for part 2: {}", result);
}

#[derive(Clone, Debug)]
struct Grid {
    width: usize,
    height: usize,
    grid: HashMap<(usize, usize), (u32, bool)>,
}

impl Grid {
    fn from_lines(lines: Vec<String>) -> Self {
        let converted_lines: Vec<Vec<u32>> = lines
            .iter()
            .map(|line| line.chars().map(|c| c.to_digit(10)).flatten().collect())
            .collect();
        let width = converted_lines.get(0).unwrap().len();
        let height = converted_lines.len();

        let mut grid = HashMap::new();

        for (row, cols) in converted_lines.iter().enumerate() {
            for (col, value) in cols.iter().enumerate() {
                grid.insert((row, col), (*value, false));
            }
        }

        Self {
            width,
            height,
            grid,
        }
    }

    fn get_left(&self, x: usize, y: usize) -> Option<(usize, usize, u32)> {
        if x == 0 {
            None
        } else {
            Some((x - 1, y, self.grid.get(&(y, x - 1)).unwrap().to_owned().0))
        }
    }

    fn get_right(&self, x: usize, y: usize) -> Option<(usize, usize, u32)> {
        if x + 1 == self.width {
            None
        } else {
            Some((x + 1, y, self.grid.get(&(y, x + 1)).unwrap().to_owned().0))
        }
    }

    fn get_top(&self, x: usize, y: usize) -> Option<(usize, usize, u32)> {
        if y == 0 {
            None
        } else {
            Some((x, y - 1, self.grid.get(&(y - 1, x)).unwrap().to_owned().0))
        }
    }

    fn get_bottom(&self, x: usize, y: usize) -> Option<(usize, usize, u32)> {
        if y + 1 == self.height {
            None
        } else {
            Some((x, y + 1, self.grid.get(&(y + 1, x)).unwrap().to_owned().0))
        }
    }

    fn get(&self, x: usize, y: usize) -> u32 {
        self.grid.get(&(y, x)).unwrap().to_owned().0
    }

    fn is_marked(&self, x: usize, y: usize) -> bool {
        self.grid.get(&(y, x)).unwrap().to_owned().1
    }

    fn get_neighbors(&self, x: usize, y: usize) -> Vec<(usize, usize, u32)> {
        vec![
            self.get_left(x, y),
            self.get_right(x, y),
            self.get_top(x, y),
            self.get_bottom(x, y),
        ]
        .into_iter()
        .flatten()
        .collect()
    }

    fn is_low_point(&self, x: usize, y: usize) -> bool {
        let value = self.get(x, y);
        let left = self.get_left(x, y).map(|e| e.2).unwrap_or(9);
        let right = self.get_right(x, y).map(|e| e.2).unwrap_or(9);
        let top = self.get_top(x, y).map(|e| e.2).unwrap_or(9);
        let bottom = self.get_bottom(x, y).map(|e| e.2).unwrap_or(9);
        value < left && value < right && value < top && value < bottom
    }

    fn get_low_points(&self) -> Vec<(usize, usize)> {
        (0..self.height)
            .flat_map(move |y| {
                (0..self.width).map(move |x| {
                    if self.is_low_point(x, y) {
                        Some((x, y))
                    } else {
                        None
                    }
                })
            })
            .flatten()
            .collect()
    }

    fn get_risk_level(&self, x: usize, y: usize) -> u32 {
        self.get(x, y) + 1
    }

    fn mark(&mut self, x: usize, y: usize) {
        let entry = self.grid.entry((y, x));
        entry.and_modify(|e| e.1 = true);
    }

    fn mark_basin(&mut self, x: usize, y: usize) {
        let mut cur_level = self.get(x, y);
        self.mark(x, y);

        while self.mark_next_level(cur_level) {
            cur_level += 1;
        }
    }

    fn mark_next_level(&mut self, level: u32) -> bool {
        let next_level = level + 1;
        if next_level == 9 {
            return false;
        }

        let mut changed = false;
        for y in 0..self.height {
            for x in 0..self.width {
                let value = self.get(x, y);
                if value == level && self.is_marked(x, y) {
                    let neighbors = self.get_neighbors(x, y);
                    for (x, y, l) in neighbors {
                        if l == next_level {
                            self.mark(x, y);
                            changed = true;
                        }
                    }
                }
            }
        }

        changed
    }

    fn print(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                if (self.is_marked(x, y)) {
                    print!("\x1b[2m{}", self.get(x, y));
                } else {
                    print!("\x1b[0m{}", self.get(x, y))
                }
            }
            println!();
        }
    }

    fn clear_marks(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                self.grid.entry((y, x)).and_modify(|e| e.1 = false);
            }
        }
    }

    fn get_basin_size(&self) -> usize {
        self.grid.values().filter(|(_, marked)| *marked).count()
    }
}
