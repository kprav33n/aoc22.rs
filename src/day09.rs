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

impl Command {
    fn move_tuple(&self) -> (usize, i64, i64) {
        match &self {
            Command::Left(n) => (*n, 0, -1),
            Command::Right(n) => (*n, 0, 1),
            Command::Up(n) => (*n, 1, 0),
            Command::Down(n) => (*n, -1, 0),
        }
    }

    fn execute<F>(&self, mut update_fn: F)
    where
        F: FnMut(i64, i64),
    {
        let (n, row, col) = self.move_tuple();
        for _ in 0..n {
            update_fn(row, col);
        }
    }
}

// Return the number of positions tail visited at least once.
fn num_tail_positions(s: &str, n: usize) -> usize {
    let mut knots = vec![(0, 0); n];
    let mut tail_positions: HashSet<(i64, i64)> = HashSet::new();
    tail_positions.insert(*knots.last().unwrap());

    let mut update_head = |row: i64, col: i64| {
        knots[0] = (knots[0].0 + row, knots[0].1 + col);
        for i in 1..n {
            let (row_offset, col_offset) =
                (knots[i - 1].0 - knots[i].0, knots[i - 1].1 - knots[i].1);
            let (row_op, col_op) = (row_offset.signum(), col_offset.signum());
            if row_offset.abs() > 1 || col_offset.abs() > 1 {
                knots[i] = (knots[i].0 + row_op, knots[i].1 + col_op);
            }
        }
        tail_positions.insert(*knots.last().unwrap());
    };

    let lines: Vec<&str> = s.trim().split('\n').collect();
    for line in lines {
        let command: Command = line.parse().unwrap();
        command.execute(&mut update_head);
    }
    tail_positions.len()
}

// Return the number of positions tail visited at least once.
pub fn num_tail_positions_p1(s: &str) -> usize {
    num_tail_positions(s, 2)
}

// Return the number of positions tail visited at least once.
pub fn num_tail_positions_p2(s: &str) -> usize {
    num_tail_positions(s, 10)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_tail_positions_p1() {
        let input = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";
        assert_eq!(num_tail_positions_p1(input), 13)
    }

    #[test]
    fn test_num_tail_positions_p2() {
        let input = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";
        assert_eq!(num_tail_positions_p2(input), 36)
    }
}
