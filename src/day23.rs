use std::collections::HashMap;
use std::fmt;
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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
enum Tile {
    Empty,
    Elf,
}

struct Map {
    tiles: Vec<Vec<Tile>>,
    proposal_order: [([Position; 3], Position); 4],
    adjacencies: [Position; 8],
}

impl fmt::Debug for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, row) in self.tiles.iter().enumerate() {
            if i > 0 {
                f.write_str("\n")?;
            }
            for tile in row {
                f.write_str(match tile {
                    Tile::Empty => ".",
                    Tile::Elf => "#",
                })?;
            }
        }
        Ok(())
    }
}

impl Map {
    fn from_str(s: &str, padding: usize) -> Map {
        let num_rows = s.lines().count() + 2 * padding;
        let num_cols = s.lines().next().unwrap().len() + 2 * padding;
        let mut tiles = vec![vec![Tile::Empty; num_cols]; num_rows];
        for (r, line) in s.lines().enumerate() {
            for (c, ch) in line.chars().enumerate() {
                tiles[padding + r][padding + c] = match ch {
                    '.' => Tile::Empty,
                    '#' => Tile::Elf,
                    _ => unreachable!(),
                }
            }
        }
        let proposal_order = [
            (
                [
                    Position::new(-1, -1),
                    Position::new(-1, 0),
                    Position::new(-1, 1),
                ],
                Position::new(-1, 0),
            ),
            (
                [
                    Position::new(1, -1),
                    Position::new(1, 0),
                    Position::new(1, 1),
                ],
                Position::new(1, 0),
            ),
            (
                [
                    Position::new(-1, -1),
                    Position::new(0, -1),
                    Position::new(1, -1),
                ],
                Position::new(0, -1),
            ),
            (
                [
                    Position::new(-1, 1),
                    Position::new(0, 1),
                    Position::new(1, 1),
                ],
                Position::new(0, 1),
            ),
        ];
        let adjacencies = [
            Position::new(-1, -1),
            Position::new(-1, 0),
            Position::new(-1, 1),
            Position::new(0, -1),
            Position::new(0, 1),
            Position::new(1, -1),
            Position::new(1, 0),
            Position::new(1, 1),
        ];
        Map {
            tiles,
            proposal_order,
            adjacencies,
        }
    }

    fn tile(&self, pos: &Position) -> Tile {
        self.tiles[pos.row as usize][pos.col as usize]
    }

    fn mut_tile(&mut self, pos: &Position) -> &mut Tile {
        &mut self.tiles[pos.row as usize][pos.col as usize]
    }

    fn round(&mut self, num: usize) -> usize {
        let mut proposals: Vec<(Position, Position)> = Vec::new();
        for (i, row) in self.tiles.iter().enumerate() {
            for (j, &tile) in row.iter().enumerate() {
                if tile != Tile::Elf {
                    continue;
                }

                let pos = Position::new(i as i64, j as i64);
                if self
                    .adjacencies
                    .iter()
                    .all(|&p| self.tile(&(pos + p)) == Tile::Empty)
                {
                    continue;
                }

                for (check, mv) in self.proposal_order.iter().cycle().skip(num - 1).take(4) {
                    if check.iter().all(|&p| self.tile(&(pos + p)) == Tile::Empty) {
                        proposals.push((pos, pos + *mv));
                        break;
                    }
                }
            }
        }

        let mut dst_hist: HashMap<Position, usize> = HashMap::new();
        for (_, dst) in &proposals {
            *dst_hist.entry(*dst).or_insert(0) += 1;
        }

        let mut num_moves = 0;
        for (src, dst) in &proposals {
            if dst_hist[dst] > 1 {
                continue;
            }
            *self.mut_tile(dst) = Tile::Elf;
            *self.mut_tile(src) = Tile::Empty;
            num_moves += 1;
        }
        num_moves
    }

    fn simulate(&mut self, count: usize) -> usize {
        let mut n = 1;
        loop {
            let num_moves = self.round(n);
            if count == n {
                break;
            }
            if num_moves == 0 {
                return n;
            }
            n += 1;
        }

        let (min_row, min_col, max_row, max_col) = self.bounds();
        let mut result = 0;
        for i in min_row..=max_row {
            for j in min_col..=max_col {
                if self.tiles[i][j] == Tile::Empty {
                    result += 1;
                }
            }
        }
        result
    }

    fn bounds(&self) -> (usize, usize, usize, usize) {
        let (mut min_row, mut min_col, mut max_row, mut max_col) =
            (usize::MAX, usize::MAX, usize::MIN, usize::MIN);
        for (i, row) in self.tiles.iter().enumerate() {
            for (j, &tile) in row.iter().enumerate() {
                if tile == Tile::Elf {
                    min_row = min_row.min(i);
                    min_col = min_col.min(j);
                    max_row = max_row.max(i);
                    max_col = max_col.max(j);
                }
            }
        }
        (min_row, min_col, max_row, max_col)
    }
}

pub fn empty_ground_tiles(s: &str) -> usize {
    // HACK: Padding was figured out based on out of bounds error.
    let mut map = Map::from_str(s, 6);
    map.simulate(10)
}

pub fn first_idle_round(s: &str) -> usize {
    // HACK: Padding was figured out based on out of bounds error.
    let mut map = Map::from_str(s, 60);
    map.simulate(usize::MAX)
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "....#..
..###.#
#...#.#
.#...##
#.###..
##.#.##
.#..#..";

    #[test]
    fn test_empty_ground_tiles() {
        assert_eq!(empty_ground_tiles(INPUT), 110);
    }

    #[test]
    fn test_first_idle_round() {
        assert_eq!(first_idle_round(INPUT), 20);
    }
}
