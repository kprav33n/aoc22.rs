extern crate pathfinding;

use pathfinding::prelude::bfs;
use std::str::FromStr;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Position(usize, usize);

#[derive(Debug)]
struct HeightMap {
    grid: Vec<Vec<char>>,
    start: Position,
    end: Position,
}

impl FromStr for HeightMap {
    type Err = ();

    fn from_str(s: &str) -> Result<HeightMap, Self::Err> {
        let mut grid: Vec<Vec<char>> = s.trim().split('\n').map(|s| s.chars().collect()).collect();
        let (mut start, mut end) = (Position(0, 0), Position(0, 0));
        for (i, row) in grid.iter().enumerate() {
            for (j, &ch) in row.iter().enumerate() {
                if ch == 'S' {
                    start = Position(i, j);
                } else if ch == 'E' {
                    end = Position(i, j);
                }
            }
        }
        grid[start.0][start.1] = 'a';
        grid[end.0][end.1] = 'z';
        Ok(HeightMap { grid, start, end })
    }
}

impl HeightMap {
    fn adjancencies(&self, pos: &Position) -> Vec<Position> {
        let mut v: Vec<Position> = Vec::new();
        if pos.0 > 0 {
            v.push(Position(pos.0 - 1, pos.1));
        }
        if pos.0 < self.grid.len() - 1 {
            v.push(Position(pos.0 + 1, pos.1));
        }
        if pos.1 > 0 {
            v.push(Position(pos.0, pos.1 - 1));
        }
        if pos.1 < self.grid[pos.0].len() - 1 {
            v.push(Position(pos.0, pos.1 + 1));
        }
        v.into_iter()
            .filter(|p| (self.grid[p.0][p.1] as i64 - self.grid[pos.0][pos.1] as i64) <= 1)
            .collect::<Vec<_>>()
    }
}

pub fn num_steps_to_target(s: &str) -> usize {
    let Ok(map) = s.parse::<HeightMap>() else {
        unreachable!();
    };
    let Some(path) = bfs(&map.start, |p| map.adjancencies(p), |&p| p == map.end) else {
        unreachable!();
    };
    path.len() - 1
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

    #[test]
    fn test_num_steps_to_target() {
        assert_eq!(num_steps_to_target(INPUT), 31);
    }
}
