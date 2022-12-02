use std::str::FromStr;

#[derive(Debug, Eq, PartialEq)]
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

#[derive(Debug, Eq, PartialEq)]
enum Outcome {
    Win,
    Loss,
    Tie,
}

impl Outcome {
    fn new(h1: &Hand, h2: &Hand) -> Outcome {
        match (h1, h2) {
            (Hand::Scissors, Hand::Rock)
            | (Hand::Paper, Hand::Scissors)
            | (Hand::Rock, Hand::Paper) => Outcome::Win,
            (a, b) if (a == b) => Outcome::Tie,
            _ => Outcome::Loss,
        }
    }

    fn score(&self) -> i64 {
        match &self {
            Outcome::Win => 6,
            Outcome::Tie => 3,
            Outcome::Loss => 0,
        }
    }

    fn hand_score(h1: &Hand, h2: &Hand) -> i64 {
        let outcome = Outcome::new(h1, h2);
        outcome.score()
            + match h2 {
                Hand::Rock => 1,
                Hand::Paper => 2,
                Hand::Scissors => 3,
            }
    }

    fn hand_score_from_string(s: &str) -> i64 {
        let v = s.split(' ').collect::<Vec<&str>>();
        let h1: Hand = v[0].parse().unwrap();
        let h2: Hand = v[1].parse().unwrap();
        let score = Self::hand_score(&h1, &h2);
        score
    }
}

pub fn total_score(s: &str) -> i64 {
    s.trim()
        .split('\n')
        .map(Outcome::hand_score_from_string)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total_score() {
        let s = "A Y
B X
C Z";
        assert_eq!(total_score(s), 15);
    }
}
