use std::collections::HashSet;

fn parse_line(s: &str) -> i64 {
    let (first, last) = s.split_at(s.len() / 2);
    let s1 = first.chars().collect::<HashSet<_>>();
    let s2 = &last.chars().collect::<HashSet<_>>();
    let mut ixn = s1.intersection(s2);
    let ch = ixn.next().unwrap();
    let p = (*ch as i64) - 96;
    if p > 0 {
        p
    } else {
        p + 58
    }
}

pub fn sum_priorities(s: &str) -> i64 {
    s.trim().split_terminator("\n").map(parse_line).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_priorities() {
        let s = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
        assert_eq!(sum_priorities(s), 157);
    }
}
