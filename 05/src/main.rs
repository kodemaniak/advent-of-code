use std::{
    collections::HashMap,
    fs::File,
    io::{prelude::*, BufReader},
    ops::Neg,
};

fn main() -> Result<(), std::io::Error> {
    let file = File::open("input")?;

    let lines = BufReader::new(file).lines().flatten().map(Line::from);

    let points = lines.map(|l| l.get_points()).flatten();

    let mut map: HashMap<(u16, u16), u16> = HashMap::new();
    for point in points {
        match map.get_mut(&point) {
            Some(counter) => {
                *counter += 1;
            }
            None => {
                map.insert(point, 1);
            }
        }
    }

    let overlaps = map.iter().filter(|(_, c)| **c > 1).count();

    println!("Points with overalpping lines: {}", overlaps);

    Ok(())
}

#[derive(Debug)]
struct Line(u16, u16, u16, u16);

impl Line {
    fn get_points(&self) -> Vec<(u16, u16)> {
        if self.0 == self.2 {
            let diff = self.1 as i16 - self.3 as i16;
            let signum = diff.signum().neg();
            let num_points = diff.abs();

            (0..=num_points)
                .map(|i| (self.0, (self.1 as i16 + i * signum) as u16))
                .collect()
        } else if self.1 == self.3 {
            let diff = self.0 as i16 - self.2 as i16;
            let signum = diff.signum().neg();
            let num_points = diff.abs();

            (0..=num_points)
                .map(|i| ((self.0 as i16 + i * signum) as u16, self.1))
                .collect()
        } else {
            let diff_v = self.0 as i16 - self.2 as i16;
            let diff_h = self.1 as i16 - self.3 as i16;
            let signum_h = diff_h.signum().neg();
            let signum_v = diff_v.signum().neg();
            let num_points = diff_h.abs();

            (0..=num_points)
                .map(|i| {
                    (
                        (self.0 as i16 + i * signum_v) as u16,
                        (self.1 as i16 + i * signum_h) as u16,
                    )
                })
                .collect()
        }
    }
}

impl From<String> for Line {
    fn from(cs: String) -> Self {
        let points: Vec<&str> = cs.split(" -> ").collect();
        let p1: Vec<u16> = points[0]
            .split(',')
            .map(|c| c.parse::<u16>().unwrap())
            .collect();
        let p2: Vec<u16> = points[1]
            .split(',')
            .map(|c| c.parse::<u16>().unwrap())
            .collect();
        Line(p1[0], p1[1], p2[0], p2[1])
    }
}
