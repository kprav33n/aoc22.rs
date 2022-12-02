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
    fn from(h1: &Hand, h2: &Hand) -> Outcome {
        match (h1, h2) {
            (Hand::Scissors, Hand::Rock)
            | (Hand::Paper, Hand::Scissors)
            | (Hand::Rock, Hand::Paper) => Outcome::Win,
            (a, b) if (a == b) => Outcome::Draw,
            _ => Outcome::Loss,
        }
    }

    fn score(&self) -> i64 {
        match &self {
            Outcome::Win => 6,
            Outcome::Draw => 3,
            Outcome::Loss => 0,
        }
    }

    fn hand_score(h1: &Hand, h2: &Hand) -> i64 {
        let outcome = Outcome::from(h1, h2);
        outcome.score()
            + match h2 {
                Hand::Rock => 1,
                Hand::Paper => 2,
                Hand::Scissors => 3,
            }
    }

    fn hand_response(h: Hand, o: &Outcome) -> Hand {
        match (h, o) {
            (x, Outcome::Draw) => x,
            (Hand::Rock, Outcome::Win) => Hand::Paper,
            (Hand::Paper, Outcome::Win) => Hand::Scissors,
            (Hand::Scissors, Outcome::Win) => Hand::Rock,
            (Hand::Rock, Outcome::Loss) => Hand::Scissors,
            (Hand::Paper, Outcome::Loss) => Hand::Rock,
            (Hand::Scissors, Outcome::Loss) => Hand::Paper,
        }
    }

    fn hand_score_from_string_p1(s: &str) -> i64 {
        let v = s.split(' ').collect::<Vec<&str>>();
        let h1: Hand = v[0].parse().unwrap();
        let h2: Hand = v[1].parse().unwrap();
        Self::hand_score(&h1, &h2)
    }

    fn hand_score_from_string_p2(s: &str) -> i64 {
        let v = s.split(' ').collect::<Vec<&str>>();
        let h1: Hand = v[0].parse().unwrap();
        let o: Outcome = v[1].parse().unwrap();
        let h2 = Self::hand_response(h1, &o);
        Self::hand_score(&h1, &h2)
    }
}

pub fn total_score_p1(s: &str) -> i64 {
    s.trim()
        .split('\n')
        .map(Outcome::hand_score_from_string_p1)
        .sum()
}

pub fn total_score_p2(s: &str) -> i64 {
    s.trim()
        .split('\n')
        .map(Outcome::hand_score_from_string_p2)
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
