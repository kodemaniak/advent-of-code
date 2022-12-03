use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
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
    let mut map = Map::new();
    let connections: Vec<Connection> = lines.iter().map(|l| Connection::from_line(l)).collect();
    for con in connections {
        map.add(con);
    }

    let _paths = search(&map);
}

fn part2(_lines: Vec<String>) {}

fn search(_map: &Map) -> Vec<Vec<MapPoint>> {
    Vec::new()
}

struct Map {
    map: HashMap<MapPoint, Vec<MapPoint>>,
}

impl Map {
    fn new() -> Self {
        Map {
            map: HashMap::new(),
        }
    }

    fn add(&mut self, con: Connection) {
        match self.map.get_mut(&con.from) {
            Some(list) => list.push(con.to),
            None => {
                let list = vec![con.to];
                self.map.insert(con.from, list);
            }
        }
    }
}

struct Connection {
    from: MapPoint,
    to: MapPoint,
}

impl Connection {
    fn from_line(line: &str) -> Connection {
        let parts: Vec<&str> = line.split('-').collect();
        let from = MapPoint::from_str(parts[0]);
        let to = MapPoint::from_str(parts[1]);
        Connection { from, to }
    }
}

#[derive(PartialEq, Eq, Hash, Clone)]
enum MapPoint {
    Start,
    End,
    Small(String),
    Big(String),
}

impl MapPoint {
    fn from_str(string: &str) -> MapPoint {
        if string == "start" {
            MapPoint::Start
        } else if string == "end" {
            MapPoint::End
        } else if string.chars().all(|c| c.is_lowercase()) {
            MapPoint::Small(String::from_str(string).unwrap())
        } else {
            MapPoint::Big(String::from_str(string).unwrap())
        }
    }
}
