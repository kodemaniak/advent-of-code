use std::cmp::Ordering;
use std::collections::{binary_heap, BinaryHeap, HashMap};
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> Result<(), std::io::Error> {
    let file = File::open("input_all_test")?;

    let lines: Vec<String> = BufReader::new(file).lines().flatten().collect();

    let grid = Grid::from_lines(lines);

    part1(grid.clone());
    part2(grid);

    Ok(())
}

fn part1(grid: Grid) {
    grid.print();
    let total = a_star(grid);
    dbg!(total);
}

fn part2(mut grid: Grid) {}

fn a_star(grid: Grid) -> Option<u32> {
    let start = grid.start;
    let end = grid.end;

    let mut open_list = BinaryHeap::new();
    let mut closed_list: Vec<Waypoint> = Vec::new();

    let start_cell = grid.get(start.0, start.1).unwrap();
    open_list.push(Waypoint {
        acc_risk_level: 0,
        cell: start_cell.clone(),
        from: None,
    });

    while let Some(waypoint) = open_list.pop() {
        let cell = waypoint.cell.clone();
        if cell.coords == end {
            closed_list.push(waypoint);
            break;
        }

        if closed_list.iter().find(|wp| wp.cell == cell).is_some() {
            continue;
        }

        let next_positions = grid
            .get_neighbors(cell.coords.0, cell.coords.1)
            .into_iter()
            .filter(|c| closed_list.iter().find(|wp| wp.cell == *c).is_none());

        for next_position in next_positions {
            let acc_risk_level = waypoint.acc_risk_level + next_position.risk_level;
            open_list.push(Waypoint {
                acc_risk_level,
                cell: next_position,
                from: Some(cell.clone()),
            });
        }

        closed_list.push(waypoint);
    }

    if let Some(head) = closed_list.pop() {
        if head.cell.coords == end {
            Some(head.acc_risk_level)
        } else {
            None
        }
    } else {
        None
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Waypoint {
    acc_risk_level: u32,
    cell: GridCell,
    from: Option<GridCell>,
}

impl Ord for Waypoint {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        let other_cost = other.acc_risk_level + other.cell.distance as u32;
        let my_cost = self.acc_risk_level + self.cell.distance as u32;
        other_cost.cmp(&my_cost)
    }
}

impl PartialOrd for Waypoint {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone, Debug)]
struct Grid {
    width: usize,
    height: usize,
    grid: HashMap<(usize, usize), GridCell>,
    start: (usize, usize),
    end: (usize, usize),
}

impl Grid {
    fn from_lines(lines: Vec<String>) -> Self {
        let converted_lines: Vec<Vec<u32>> = lines
            .iter()
            .map(|line| line.chars().map(|c| c.to_digit(10)).flatten().collect())
            .collect();
        let width = converted_lines.get(0).unwrap().len();
        let height = converted_lines.len();
        let start = (0_usize, 0_usize);
        let end = (height - 1, width - 1);

        let mut grid = HashMap::new();

        for (row, cols) in converted_lines.iter().enumerate() {
            for (col, value) in cols.iter().enumerate() {
                grid.insert(
                    (row, col),
                    GridCell::new((row, col), *value, Self::distance((row, col), end)),
                );
            }
        }

        Self {
            width,
            height,
            grid,
            start,
            end,
        }
    }

    fn distance(from: (usize, usize), to: (usize, usize)) -> i32 {
        (to.0 as i32 - from.0 as i32).abs() + (to.1 as i32 - from.1 as i32).abs()
    }

    fn get_left(&self, row: usize, col: usize) -> Option<GridCell> {
        if col == 0 {
            None
        } else {
            self.get(row, col - 1)
        }
    }

    fn get_right(&self, row: usize, col: usize) -> Option<GridCell> {
        if col + 1 == self.width {
            None
        } else {
            self.get(row, col + 1)
        }
    }

    fn get_top(&self, row: usize, col: usize) -> Option<GridCell> {
        if row == 0 {
            None
        } else {
            self.get(row - 1, col)
        }
    }

    fn get_bottom(&self, row: usize, col: usize) -> Option<GridCell> {
        if row + 1 == self.height {
            None
        } else {
            self.get(row + 1, col)
        }
    }

    fn get_neighbors(&self, x: usize, y: usize) -> Vec<GridCell> {
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

    fn get(&self, row: usize, cell: usize) -> Option<GridCell> {
        self.grid.get(&(row, cell)).cloned()
    }

    fn print(&self) {
        for row in 0..self.height {
            for col in 0..self.width {
                // if self.is_marked(x, y) {
                //     print!("\x1b[2m{}", self.get(x, y));
                // } else {
                print!("\x1b[0m{}", self.get(row, col).unwrap().risk_level)
                // }
            }
            println!();
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct GridCell {
    coords: (usize, usize),
    risk_level: u32,
    distance: i32,
}

impl GridCell {
    fn new(coords: (usize, usize), risk_level: u32, distance: i32) -> Self {
        Self {
            coords,
            risk_level,
            distance,
        }
    }

    fn cost(&self) -> u32 {
        self.risk_level + self.distance as u32
    }
}
