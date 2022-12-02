use std::str::FromStr;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}

impl FromStr for Hand {
    type Err = ();

    fn from_str(input: &str) -> Result<Hand, Self::Err> {
        match input {
            "A" => Ok(Hand::Rock),
            "B" => Ok(Hand::Paper),
            "C" => Ok(Hand::Scissors),

            "X" => Ok(Hand::Rock),
            "Y" => Ok(Hand::Paper),
            "Z" => Ok(Hand::Scissors),

            _ => Err(()),
        }
    }
}

impl Hand {
    // Return the bonus score for this hand.
    fn bonus(&self) -> i64 {
        match &self {
            Hand::Rock => 1,
            Hand::Paper => 2,
            Hand::Scissors => 3,
        }
    }

    // Return the score for this Hand played against the given Hand.
    fn score_against(self, h: Hand) -> i64 {
        let outcome = Outcome::from(h, self);
        outcome.score() + self.bonus()
    }
}

#[derive(Debug, Eq, PartialEq)]
enum Outcome {
    Win,
    Loss,
    Draw,
}

impl FromStr for Outcome {
    type Err = ();

    fn from_str(input: &str) -> Result<Outcome, Self::Err> {
        match input {
            "X" => Ok(Outcome::Loss),
            "Y" => Ok(Outcome::Draw),
            "Z" => Ok(Outcome::Win),

            _ => Err(()),
        }
    }
}

impl Outcome {
    // Create the Outcome of two Hands.
    fn from(h1: Hand, h2: Hand) -> Outcome {
        match (h1, h2) {
            (Hand::Scissors, Hand::Rock)
            | (Hand::Paper, Hand::Scissors)
            | (Hand::Rock, Hand::Paper) => Outcome::Win,
            (a, b) if (a == b) => Outcome::Draw,
            _ => Outcome::Loss,
        }
    }

    // Return the score for this outcome.
    fn score(&self) -> i64 {
        match &self {
            Outcome::Win => 6,
            Outcome::Draw => 3,
            Outcome::Loss => 0,
        }
    }

    // Return a response for given Hand that will produce this outcome.
    fn response(&self, h: Hand) -> Hand {
        match (h, &self) {
            (x, Outcome::Draw) => x,
            (Hand::Rock, Outcome::Win) => Hand::Paper,
            (Hand::Paper, Outcome::Win) => Hand::Scissors,
            (Hand::Scissors, Outcome::Win) => Hand::Rock,
            (Hand::Rock, Outcome::Loss) => Hand::Scissors,
            (Hand::Paper, Outcome::Loss) => Hand::Rock,
            (Hand::Scissors, Outcome::Loss) => Hand::Paper,
        }
    }

    // Return the score when played against the given Hand for this outcome.
    fn score_against(&self, h: Hand) -> i64 {
        let other = &self.response(h);
        self.score() + other.bonus()
    }
}

// Parse the line and return a tuple of Hands.
fn parse_line_p1(s: &str) -> (Hand, Hand) {
    let v: Vec<&str> = s.split_whitespace().collect();
    (v[0].parse().unwrap(), v[1].parse().unwrap())
}

/// Find the total score.
pub fn total_score_p1(s: &str) -> i64 {
    s.trim()
        .split('\n')
        .map(parse_line_p1)
        .map(|(h1, h2)| h2.score_against(h1))
        .sum()
}

// Parse the line and return a tuple of Hand and Outcome.
fn parse_line_p2(s: &str) -> (Hand, Outcome) {
    let v: Vec<&str> = s.split_whitespace().collect();
    (v[0].parse().unwrap(), v[1].parse().unwrap())
}

/// Find the total score.
pub fn total_score_p2(s: &str) -> i64 {
    s.trim()
        .split('\n')
        .map(parse_line_p2)
        .map(|(h, o)| o.score_against(h))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total_score_p1() {
        let s = "A Y
B X
C Z";
        assert_eq!(total_score_p1(s), 15);
    }

    #[test]
    fn test_total_score_p2() {
        let s = "A Y
B X
C Z";
        assert_eq!(total_score_p2(s), 12);
    }
}
