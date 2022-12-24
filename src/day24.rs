use std::collections::HashSet;
use std::ops::Add;

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, PartialOrd)]
struct Position {
    row: i64,
    col: i64,
}

impl Position {
    fn new(row: i64, col: i64) -> Position {
        Position { row, col }
    }

    fn neighbors(&self) -> [Position; 5] {
        [
            Position::new(self.row - 1, self.col),
            Position::new(self.row + 1, self.col),
            Position::new(self.row, self.col - 1),
            Position::new(self.row, self.col + 1),
            Position::new(self.row, self.col),
        ]
    }
}

impl Add for Position {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            row: self.row + other.row,
            col: self.col + other.col,
        }
    }
}

#[derive(Debug)]
struct Basin {
    walls: HashSet<Position>,
    blizzards: HashSet<(Position, Position)>,
    num_rows: i64,
    num_cols: i64,
    start: Position,
    goal: Position,
}

impl Basin {
    fn from_str(s: &str) -> Basin {
        let mut walls: HashSet<Position> = HashSet::new();
        let mut blizzards: HashSet<(Position, Position)> = HashSet::new();
        let (mut num_rows, mut num_cols) = (i64::MIN, i64::MIN);
        for (i, line) in s.lines().enumerate() {
            for (j, ch) in line.chars().enumerate() {
                let (i, j) = (i as i64 - 1, j as i64 - 1);
                let pos = Position::new(i, j);
                num_rows = num_rows.max(i);
                num_cols = num_cols.max(j);
                match ch {
                    '#' => walls.insert(pos),
                    '^' => blizzards.insert((pos, Position::new(-1, 0))),
                    'v' => blizzards.insert((pos, Position::new(1, 0))),
                    '<' => blizzards.insert((pos, Position::new(0, -1))),
                    '>' => blizzards.insert((pos, Position::new(0, 1))),
                    '.' => true,
                    _ => unreachable!(),
                };
            }
        }
        walls.insert(Position::new(-2, 0));
        let start = Position::new(-1, 0);
        let goal = Position::new(num_rows, num_cols - 1);
        Basin {
            walls,
            blizzards,
            num_rows,
            num_cols,
            start,
            goal,
        }
    }

    fn fewest_minutes_to_goal(&self) -> i64 {
        let mut minutes = 1;
        let mut current: HashSet<Position> = HashSet::new();
        current.insert(self.start);
        loop {
            // Brute force idea courtesy: u/KeyJ
            let blizzards: HashSet<Position> = self
                .blizzards
                .iter()
                .map(|(p, d)| {
                    Position::new(
                        (p.row + d.row * minutes).rem_euclid(self.num_rows),
                        (p.col + d.col * minutes).rem_euclid(self.num_cols),
                    )
                })
                .collect();
            current = current
                .iter()
                .flat_map(|p| p.neighbors())
                .filter(|p| !blizzards.contains(p) && !self.walls.contains(p))
                .collect();
            // println!("After {} minutes: {:?}", minutes, current);
            if current.contains(&self.goal) {
                return minutes;
            }
            minutes += 1;
        }
    }
}

pub fn fewest_minutes_to_goal(s: &str) -> i64 {
    let basin = Basin::from_str(s);
    basin.fewest_minutes_to_goal()
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#";

    #[test]
    fn test_fewest_minutes_to_goal() {
        assert_eq!(fewest_minutes_to_goal(INPUT), 18);
    }
}
