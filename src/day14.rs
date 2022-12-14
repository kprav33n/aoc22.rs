use std::collections::HashMap;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
struct Position {
    x: i64,
    y: i64,
}

impl FromStr for Position {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.split_once(',').unwrap();

        let x = x.parse::<i64>()?;
        let y = y.parse::<i64>()?;

        Ok(Position { x, y })
    }
}

impl Position {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
}

#[derive(Debug)]
enum Material {
    Rock,
    // Air,
    Sand,
}

#[derive(Debug)]
struct Ground {
    source: Position,
    grid: HashMap<Position, Material>,
    deepest: i64,
    num_sand_units: usize,
    till_floor: bool,
}

impl Ground {
    fn new() -> Self {
        Ground {
            source: Position::new(500, 0),
            grid: HashMap::new(),
            deepest: i64::MIN,
            num_sand_units: 0,
            till_floor: false,
        }
    }

    fn parse_path(&mut self, s: &str) {
        let positions: Vec<&str> = s.trim().split(" -> ").collect();
        for window in positions.windows(2) {
            let p1 = window[0].parse::<Position>().unwrap();
            let p2 = window[1].parse::<Position>().unwrap();
            if p1.x == p2.x {
                for y in p1.y.min(p2.y)..=p1.y.max(p2.y) {
                    self.grid.insert(Position::new(p1.x, y), Material::Rock);
                }
            } else if p1.y == p2.y {
                for x in p1.x.min(p2.x)..=p1.x.max(p2.x) {
                    self.grid.insert(Position::new(x, p1.y), Material::Rock);
                }
            }
            self.deepest = self.deepest.max(p1.y).max(p2.y);
        }
    }

    fn start_filling(&mut self) {
        let mut position = self.source.clone();
        loop {
            match self.next_position(&position) {
                None => {
                    self.grid.insert(position.clone(), Material::Sand);
                    self.num_sand_units += 1;
                    if position == self.source {
                        break;
                    }
                    position = self.source.clone();
                }
                Some(next) => {
                    if !self.till_floor && next.y > self.deepest {
                        break;
                    }
                    position = next
                }
            }
        }
    }

    fn next_position(&self, p: &Position) -> Option<Position> {
        if self.till_floor && p.y == self.deepest + 1 {
            return None;
        }
        let candidates = [
            Position::new(p.x, p.y + 1),
            Position::new(p.x - 1, p.y + 1),
            Position::new(p.x + 1, p.y + 1),
        ];
        for candidate in candidates {
            match self.grid.get(&candidate) {
                None => return Some(candidate),
                _ => continue,
            }
        }
        None
    }
}

// Return number of resting sand units.
pub fn num_resting_sand_units(s: &str, till_floor: bool) -> usize {
    let mut ground = Ground::new();
    ground.till_floor = till_floor;
    let paths: Vec<&str> = s.trim().split('\n').collect();
    for path in paths {
        ground.parse_path(path);
    }
    ground.start_filling();
    ground.num_sand_units
}

// Return number of resting sand units according to part 1.
pub fn num_resting_sand_units_p1(s: &str) -> usize {
    num_resting_sand_units(s, false)
}

// Return number of resting sand units according to part 2.
pub fn num_resting_sand_units_p2(s: &str) -> usize {
    num_resting_sand_units(s, true)
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

    #[test]
    fn test_num_resting_sand_units() {
        assert_eq!(num_resting_sand_units_p1(INPUT), 24);
    }

    #[test]
    fn test_num_resting_sand_units_p2() {
        assert_eq!(num_resting_sand_units_p2(INPUT), 93);
    }
}
