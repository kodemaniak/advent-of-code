use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> Result<(), std::io::Error> {
    let file = File::open("input_test")?;

    let lines: Vec<String> = BufReader::new(file).lines().flatten().collect();

    let grid = Grid::from_lines(lines);

    part1(&grid);
    part2(&grid);

    Ok(())
}

fn part1(grid: &Grid) {
    let mut total_risk_level = 0;
    for x in 0..grid.width {
        for y in 0..grid.height {
            println!("{}, {}", x, y);
            if grid.is_low_point(x, y) {
                total_risk_level += grid.get_risk_level(x, y);
            }
        }
    }

    println!("The total risk level is {}", total_risk_level);
}

fn part2(grid: &Grid) {
    let mut basins = Vec::new();
    for x in 0..grid.width {
        for y in 0..grid.height {
            if grid.is_low_point(x, y) {
                basins.push(grid.get_basins(x, y));
            }
        }
    }
    basins.sort_by_key(|a| a.len());
    basins.reverse();
    let three: Vec<usize> = basins.iter().take(3).map(|b| b.len()).collect();
    dbg!(three);
    let result = basins.iter().take(3).fold(1, |r, b| r * b.len());

    println!("Result for part 2: {}", result);
}

struct Grid {
    width: usize,
    height: usize,
    grid: Vec<Vec<u32>>,
}

impl Grid {
    fn from_lines(lines: Vec<String>) -> Self {
        let grid: Vec<Vec<u32>> = lines
            .iter()
            .map(|line| line.chars().map(|c| c.to_digit(10)).flatten().collect())
            .collect();
        let width = grid.get(0).unwrap().len();
        let height = grid.len();
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
            Some((x - 1, y, self.grid[y][x - 1]))
        }
    }

    fn get_right(&self, x: usize, y: usize) -> Option<(usize, usize, u32)> {
        if x + 1 == self.width {
            None
        } else {
            Some((x + 1, y, self.grid[y][x + 1]))
        }
    }

    fn get_top(&self, x: usize, y: usize) -> Option<(usize, usize, u32)> {
        if y == 0 {
            None
        } else {
            Some((x, y - 1, self.grid[y - 1][x]))
        }
    }

    fn get_bottom(&self, x: usize, y: usize) -> Option<(usize, usize, u32)> {
        if y + 1 == self.height {
            None
        } else {
            Some((x, y + 1, self.grid[y + 1][x]))
        }
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
        let value = self.grid[y][x];
        let left = self.get_left(x, y).map(|e| e.2).unwrap_or(9);
        let right = self.get_right(x, y).map(|e| e.2).unwrap_or(9);
        let top = self.get_top(x, y).map(|e| e.2).unwrap_or(9);
        let bottom = self.get_bottom(x, y).map(|e| e.2).unwrap_or(9);
        value < left && value < right && value < top && value < bottom
    }

    fn get_risk_level(&self, x: usize, y: usize) -> u32 {
        self.grid[y][x] + 1
    }

    fn get_basins(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let own_level = self.grid[y][x];
        dbg!(own_level);
        let next_level_neighbors: Vec<(usize, usize, u32)> = self
            .get_neighbors(x, y)
            .into_iter()
            .filter(|(_, _, l)| *l == own_level + 1)
            .collect();
        dbg!(next_level_neighbors);
        Vec::new()
    }
}
