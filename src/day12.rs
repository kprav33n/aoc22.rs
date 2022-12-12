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
    fn neighbors(&self, pos: &Position) -> Vec<Position> {
        [
            if pos.0 > 0 {
                Some(Position(pos.0 - 1, pos.1))
            } else {
                None
            },
            if pos.0 < self.grid.len() - 1 {
                Some(Position(pos.0 + 1, pos.1))
            } else {
                None
            },
            if pos.1 > 0 {
                Some(Position(pos.0, pos.1 - 1))
            } else {
                None
            },
            if pos.1 < self.grid[pos.0].len() - 1 {
                Some(Position(pos.0, pos.1 + 1))
            } else {
                None
            },
        ]
        .into_iter()
        .flatten()
        .filter(|p| self.grid[p.0][p.1] as i64 - self.grid[pos.0][pos.1] as i64 <= 1)
        .collect::<Vec<_>>()
    }

    fn shortest_path(&self, start: &Position) -> Option<Vec<Position>> {
        bfs(start, |p| self.neighbors(p), |&p| p == self.end)
    }
}

pub fn num_steps_to_target_p1(s: &str) -> usize {
    let Ok(map) = s.parse::<HeightMap>() else {
        unreachable!();
    };
    let Some(path) = map.shortest_path(&map.start) else {
        unreachable!();
    };
    path.len() - 1
}

pub fn num_steps_to_target_p2(s: &str) -> usize {
    let Ok(map) = s.parse::<HeightMap>() else {
        unreachable!();
    };

    let mut starts: Vec<Position> = Vec::new();
    for (i, row) in map.grid.iter().enumerate() {
        for (j, &ch) in row.iter().enumerate() {
            if ch == 'a' {
                starts.push(Position(i, j));
            }
        }
    }

    starts
        .iter()
        .filter_map(|start| map.shortest_path(start))
        .map(|path| path.len() - 1)
        .min()
        .unwrap()
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
    fn test_num_steps_to_target_p1() {
        assert_eq!(num_steps_to_target_p1(INPUT), 31);
    }

    #[test]
    fn test_num_steps_to_target_p2() {
        assert_eq!(num_steps_to_target_p2(INPUT), 29);
    }
}
