use std::collections::HashSet;

fn priority(ch: char) -> i64 {
    let v = (ch as i64) - 96;
    if v > 0 {
        v
    } else {
        v + 58
    }
}

fn parse_line(s: &str) -> i64 {
    let (first, last) = s.split_at(s.len() / 2);
    let s1 = first.chars().collect::<HashSet<_>>();
    let s2 = &last.chars().collect::<HashSet<_>>();
    let mut ixn = s1.intersection(s2);
    let ch = ixn.next().unwrap();
    priority(*ch)
}

pub fn sum_priorities_p1(s: &str) -> i64 {
    s.trim().split_terminator('\n').map(parse_line).sum()
}

pub fn sum_priorities_p2(s: &str) -> i64 {
    let v: Vec<&str> = s.lines().collect();
    let chunks: Vec<&[&str]> = v.chunks(3).collect();
    let mut sum = 0;
    for chunk in chunks {
        let mut s0: HashSet<char> = chunk[0].chars().collect();
        let s1: HashSet<char> = chunk[1].chars().collect();
        let s2: HashSet<char> = chunk[2].chars().collect();
        s0.retain(|e| s1.contains(e));
        s0.retain(|e| s2.contains(e));
        let ch = s0.iter().next().unwrap();
        sum += priority(*ch);
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_priorities_p1() {
        let s = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
        assert_eq!(sum_priorities_p1(s), 157);
    }

    #[test]
    fn test_sum_priorities_p2() {
        let s = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
        assert_eq!(sum_priorities_p2(s), 70);
    }
}
