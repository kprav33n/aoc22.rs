use lazy_static::lazy_static;
use regex::Regex;
use std::fmt;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Tile {
    Empty,
    Open,
    Wall,
}

#[derive(Clone, Debug)]
enum Facing {
    Right,
    Down,
    Left,
    Up,
}

impl Facing {
    fn after(&self, i: &Instruction) -> Facing {
        match (self, i) {
            (Facing::Right, Instruction::Right) => Facing::Down,
            (Facing::Right, Instruction::Left) => Facing::Up,
            (Facing::Down, Instruction::Right) => Facing::Left,
            (Facing::Down, Instruction::Left) => Facing::Right,
            (Facing::Left, Instruction::Right) => Facing::Up,
            (Facing::Left, Instruction::Left) => Facing::Down,
            (Facing::Up, Instruction::Right) => Facing::Right,
            (Facing::Up, Instruction::Left) => Facing::Left,
            _ => self.clone(),
        }
    }
}

struct Map {
    tiles: Vec<Vec<Tile>>,
    num_rows: usize,
    num_cols: usize,
}

impl fmt::Debug for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, row) in self.tiles.iter().enumerate() {
            if i > 0 {
                f.write_str("\n")?;
            }
            for tile in row {
                f.write_str(match tile {
                    Tile::Empty => " ",
                    Tile::Open => ".",
                    Tile::Wall => "#",
                })?;
            }
        }
        Ok(())
    }
}

impl Map {
    fn from_str(s: &str) -> Map {
        let (num_rows, num_cols) = s
            .lines()
            .fold((usize::MIN, 0), |(r, c), l| (r + 1, l.len().max(c)));
        let mut tiles = vec![vec![Tile::Empty; num_cols]; num_rows];
        for (r, line) in s.lines().enumerate() {
            for (c, ch) in line.chars().enumerate() {
                tiles[r][c] = match ch {
                    '.' => Tile::Open,
                    '#' => Tile::Wall,
                    _ => Tile::Empty,
                }
            }
        }
        Map {
            tiles,
            num_rows,
            num_cols,
        }
    }

    fn execute(&self, instructions: &Vec<Instruction>) -> i64 {
        let mut facing = Facing::Right;
        let mut row = 0;
        let mut col = self.tiles[row]
            .iter()
            .position(|&t| t == Tile::Open)
            .unwrap();
        let mut execute_move = |facing| -> bool {
            let (pr, pc) = (row, col);

            loop {
                match facing {
                    Facing::Right => {
                        if col < self.num_cols - 1 {
                            col += 1;
                        } else {
                            col = 0;
                        }
                    }

                    Facing::Down => {
                        if row < self.num_rows - 1 {
                            row += 1;
                        } else {
                            row = 0;
                        }
                    }

                    Facing::Left => {
                        if col > 0 {
                            col -= 1;
                        } else {
                            col = self.num_cols - 1;
                        }
                    }

                    Facing::Up => {
                        if row > 0 {
                            row -= 1;
                        } else {
                            row = self.num_rows - 1;
                        }
                    }
                }
                match self.tiles[row][col] {
                    Tile::Empty => continue,
                    Tile::Wall => {
                        (row, col) = (pr, pc);
                        return false;
                    }
                    Tile::Open => return true,
                }
            }
        };

        for instruction in instructions {
            match instruction {
                Instruction::Right | Instruction::Left => {
                    facing = facing.after(instruction);
                }

                Instruction::Move(n) => {
                    for _ in 0..*n {
                        if !execute_move(facing.clone()) {
                            break;
                        }
                    }
                }
            }
        }
        1000 * (row + 1) as i64
            + 4 * (col + 1) as i64
            + match facing {
                Facing::Right => 0,
                Facing::Down => 1,
                Facing::Left => 2,
                Facing::Up => 3,
            }
    }
}

#[derive(Debug)]
enum Instruction {
    Move(usize),
    Right,
    Left,
}

impl Instruction {
    fn parse_instructions(s: &str) -> Vec<Self> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"((\d+)([LR]?))").unwrap();
        }
        let instructions: Vec<Instruction> = RE
            .captures_iter(s)
            .flat_map(|c| {
                let steps = c.get(2).unwrap().as_str().parse::<usize>().unwrap();
                let turn = c.get(3).unwrap().as_str();
                [
                    Some(Instruction::Move(steps)),
                    match turn {
                        "R" => Some(Instruction::Right),
                        "L" => Some(Instruction::Left),
                        _ => None,
                    },
                ]
            })
            .flatten()
            .collect();
        instructions
    }
}

pub fn final_password(s: &str) -> i64 {
    let (map, path) = s.split_once("\n\n").unwrap();
    let map = Map::from_str(map);
    let instructions = Instruction::parse_instructions(path);
    map.execute(&instructions)
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5
";

    #[test]
    fn test_final_password() {
        assert_eq!(final_password(INPUT), 6032);
    }
}
