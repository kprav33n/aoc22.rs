use std::collections::HashSet;
use std::str::FromStr;

#[derive(Debug)]
enum Command {
    Left(usize),
    Right(usize),
    Up(usize),
    Down(usize),
}

impl FromStr for Command {
    type Err = ();

    fn from_str(s: &str) -> Result<Command, Self::Err> {
        let Some((d, n)) = s.split_once(' ') else {
            return Err(());
        };

        let Ok(n) = n.parse() else {
            return Err(());
        };

        match d {
            "L" => Ok(Command::Left(n)),
            "R" => Ok(Command::Right(n)),
            "U" => Ok(Command::Up(n)),
            "D" => Ok(Command::Down(n)),
            _ => Err(()),
        }
    }
}

// Return the number of positions tail visited at least once.
pub fn num_tail_positions(s: &str) -> usize {
    let mut head = (0, 0);
    let mut tail = (0, 0);
    let mut tail_positions: HashSet<(i64, i64)> = HashSet::new();
    tail_positions.insert(tail);

    let mut update_head = |row: i64, col: i64| {
        head = (head.0 + row, head.1 + col);
        let (row_offset, col_offset) = (head.0 - tail.0, head.1 - tail.1);
        let (row_op, col_op) = (row_offset.signum(), col_offset.signum());
        if row_offset.abs() > 1 || col_offset.abs() > 1 {
            tail = (tail.0 + row_op, tail.1 + col_op);
        }
        tail_positions.insert(tail);
    };

    let lines: Vec<&str> = s.trim().split('\n').collect();
    for line in lines {
        let Ok(command) = line.parse() else {
            unreachable!();
        };
        match command {
            Command::Left(n) => {
                for _ in 0..n {
                    update_head(0, -1);
                }
            }

            Command::Right(n) => {
                for _ in 0..n {
                    update_head(0, 1);
                }
            }

            Command::Up(n) => {
                for _ in 0..n {
                    update_head(1, 0);
                }
            }

            Command::Down(n) => {
                for _ in 0..n {
                    update_head(-1, 0);
                }
            }
        }
    }
    tail_positions.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_tail_positions() {
        let input = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";
        assert_eq!(num_tail_positions(input), 13)
    }
}
