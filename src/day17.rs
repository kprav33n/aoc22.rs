use std::fmt;

const NUM_ROCKS: usize = 2022;
const NUM_ROCK_KINDS: usize = 5;
const TOTAL_ROCK_KINDS_HEIGHT: usize = 13;
const HEIGHT: usize = TOTAL_ROCK_KINDS_HEIGHT / NUM_ROCK_KINDS * NUM_ROCKS;
const WIDTH: usize = 7;

#[derive(Debug, Clone)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, PartialEq)]
enum Rock {
    HorizontalBar,
    Plus,
    InvertedL,
    VerticalBar,
    Square,
}

impl Rock {
    fn height(&self) -> usize {
        match self {
            Self::HorizontalBar => 1,
            Self::Plus => 3,
            Self::InvertedL => 3,
            Self::VerticalBar => 4,
            Self::Square => 2,
        }
    }

    fn width(&self) -> usize {
        match self {
            Self::HorizontalBar => 4,
            Self::Plus => 3,
            Self::InvertedL => 3,
            Self::VerticalBar => 1,
            Self::Square => 2,
        }
    }

    fn shape(&self) -> Vec<Position> {
        match self {
            Self::HorizontalBar => vec![
                Position::new(0, 0),
                Position::new(1, 0),
                Position::new(2, 0),
                Position::new(3, 0),
            ],
            Self::Plus => vec![
                Position::new(1, 0),
                Position::new(0, 1),
                Position::new(1, 1),
                Position::new(2, 1),
                Position::new(1, 2),
            ],
            Self::InvertedL => vec![
                Position::new(2, 0),
                Position::new(2, 1),
                Position::new(0, 2),
                Position::new(1, 2),
                Position::new(2, 2),
            ],
            Self::VerticalBar => vec![
                Position::new(0, 0),
                Position::new(0, 1),
                Position::new(0, 2),
                Position::new(0, 3),
            ],
            Self::Square => vec![
                Position::new(0, 0),
                Position::new(1, 0),
                Position::new(0, 1),
                Position::new(1, 1),
            ],
        }
    }

    // Resting edges was never needed. Wasted so much time basing the logic of
    // off resting edges. Leaving it as a reminder to not overcomplicate
    // solutions.
    #[allow(dead_code)]
    fn resting_edges(&self) -> Vec<Position> {
        match self {
            Self::HorizontalBar => vec![
                Position::new(0, 0),
                Position::new(1, 0),
                Position::new(2, 0),
                Position::new(3, 0),
            ],
            Self::Plus => vec![
                Position::new(0, 1),
                Position::new(2, 1),
                Position::new(1, 2),
            ],
            Self::InvertedL => vec![
                Position::new(2, 0),
                Position::new(2, 1),
                Position::new(0, 2),
                Position::new(1, 2),
                Position::new(2, 2),
            ],
            Self::VerticalBar => vec![Position::new(0, 3)],
            Self::Square => vec![Position::new(0, 1), Position::new(1, 1)],
        }
    }
}

#[derive(Debug)]
enum Direction {
    Right,
    Left,
}

#[derive(Clone, Debug, PartialEq)]
enum CellState {
    Empty,
    FallingRock,
    StoppedRock,
}

impl fmt::Display for CellState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            CellState::Empty => ".",
            CellState::FallingRock => "@",
            CellState::StoppedRock => "#",
        })
    }
}

struct Chamber {
    states: Vec<Vec<CellState>>,
    first_rock_at: usize,
}

impl fmt::Debug for Chamber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut should_skip = true;
        for (i, row) in self.states.iter().enumerate() {
            if should_skip && row.iter().all(|s| *s == CellState::Empty) {
                continue;
            }
            should_skip = false;
            f.write_str("\n")?;
            f.write_str("|")?;
            for content in row {
                f.write_str(&content.to_string())?;
            }
            write!(f, "| {}", i)?;
        }
        f.write_str("\n+-------+\n")?;
        Ok(())
    }
}

impl Chamber {
    fn new(height: usize, width: usize) -> Chamber {
        let states = vec![vec![CellState::Empty; width]; height];
        let first_rock_at = states.len();
        Chamber {
            states,
            first_rock_at,
        }
    }

    #[allow(dead_code)]
    fn trace_falling_rock(&mut self, rock: &Rock, position: &Position) {
        for p in rock.shape() {
            let x = position.x + p.x;
            let y = position.y + p.y;
            self.states[y][x] = CellState::FallingRock;
        }
        println!("{:?}", self);
        for p in rock.shape() {
            let x = position.x + p.x;
            let y = position.y + p.y;
            self.states[y][x] = CellState::Empty;
        }
    }

    fn can_move_down(&self, rock: &Rock, position: &Position) -> bool {
        rock.shape().iter().all(|p| {
            let x = p.x + position.x;
            let y = p.y + position.y + 1;
            y < self.states.len() && self.states[y][x] == CellState::Empty
        })
    }

    fn can_move_left(&self, rock: &Rock, position: &Position) -> bool {
        rock.shape().iter().all(|p| {
            let x = p.x + position.x - 1;
            let y = p.y + position.y;
            self.states[y][x] == CellState::Empty
        })
    }

    fn can_move_right(&self, rock: &Rock, position: &Position) -> bool {
        rock.shape().iter().all(|p| {
            let x = p.x + position.x + 1;
            let y = p.y + position.y;
            self.states[y][x] == CellState::Empty
        })
    }

    fn execute_move(&mut self, mv: &Direction, rock: &Rock, position: &Position) -> Position {
        match mv {
            Direction::Left => {
                if position.x == 0 || !self.can_move_left(rock, position) {
                    // println!("Jet of gas pushes rock left, but nothing happens:");
                    position.clone()
                } else {
                    // println!("Jet of gas pushes rock left:");
                    Position::new(position.x - 1, position.y)
                }
            }
            Direction::Right => {
                if position.x + rock.width() == WIDTH || !self.can_move_right(rock, position) {
                    // println!("Jet of gas pushes rock right, but nothing happens:");
                    position.clone()
                } else {
                    // println!("Jet of gas pushes rock right:");
                    Position::new(position.x + 1, position.y)
                }
            }
        }
    }

    fn rest_rock(&mut self, rock: &Rock, position: &Position) {
        for p in rock.shape() {
            let x = position.x + p.x;
            let y = position.y + p.y;
            self.states[y][x] = CellState::StoppedRock;
            self.first_rock_at = self.first_rock_at.min(position.y + p.y)
        }
    }

    fn simulate(&mut self, moves: &[Direction]) -> Result<usize, ()> {
        let rocks = vec![
            Rock::HorizontalBar,
            Rock::Plus,
            Rock::InvertedL,
            Rock::VerticalBar,
            Rock::Square,
        ];
        let mut moves = moves.iter().cycle();
        for rock in rocks.iter().cycle().take(NUM_ROCKS) {
            let mut position = Position::new(2, self.first_rock_at - 3 - rock.height());
            // println!("A new rock begins falling: {:?}", rock);
            loop {
                // self.trace_falling_rock(rock, &position);
                position = self.execute_move(moves.next().unwrap(), rock, &position);
                // self.trace_falling_rock(rock, &position);

                if !self.can_move_down(rock, &position) {
                    self.rest_rock(rock, &position);
                    // println!("Rock falls 1 unit, causing it to come to rest:");
                    // println!("{:?}", self);
                    break;
                }
                position.y += 1;
                // println!("Rock falls 1 unit:");
            }
        }
        // println!("{:?}", self);
        Ok(self.states.len() - self.first_rock_at)
    }
}

pub fn tower_height(s: &str) -> usize {
    let moves: Vec<Direction> = s
        .trim()
        .chars()
        .map(|c| match c {
            '<' => Direction::Left,
            '>' => Direction::Right,
            _ => unreachable!(),
        })
        .collect();
    let mut chamber = Chamber::new(HEIGHT, WIDTH);
    chamber.simulate(&moves).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

    #[test]
    fn test_tower_height() {
        assert_eq!(tower_height(INPUT), 3068);
    }
}
