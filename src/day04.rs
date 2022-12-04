use std::str::FromStr;

struct Range {
    start: i64,
    end: i64,
}

impl FromStr for Range {
    type Err = ();

    fn from_str(s: &str) -> Result<Range, Self::Err> {
        let v: Vec<&str> = s.trim().split('-').collect();
        if v.len() != 2 {
            return Err(());
        }

        if let (Ok(x), Ok(y)) = (v[0].parse::<i64>(), v[1].parse::<i64>()) {
            Ok(Range { start: x, end: y })
        } else {
            Err(())
        }
    }
}

impl Range {
    fn does_fully_contain(&self, r: &Range) -> bool {
        (r.start >= self.start && r.end <= self.end) || (self.start >= r.start && self.end <= r.end)
    }

    fn does_overlap(&self, r: &Range) -> bool {
        let x = if self.start < r.start { self } else { r };
        let y = if self.start > r.start { self } else { r };
        x.end >= y.start
    }
}

fn parse_line(s: &str) -> (Range, Range) {
    let v: Vec<&str> = s.trim().split(',').collect();
    (v[0].parse().unwrap(), v[1].parse().unwrap())
}

pub fn num_fully_contained(s: &str) -> i64 {
    s.trim()
        .split_terminator('\n')
        .map(parse_line)
        .map(|(r1, r2)| r1.does_fully_contain(&r2))
        .filter(|b| *b)
        .count()
        .try_into()
        .unwrap()
}

pub fn num_overlapping(s: &str) -> i64 {
    s.trim()
        .split_terminator('\n')
        .map(parse_line)
        .map(|(r1, r2)| r1.does_overlap(&r2))
        .filter(|b| *b)
        .count()
        .try_into()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_fully_contained() {
        let s = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";
        assert_eq!(num_fully_contained(s), 2);
    }

    #[test]
    fn test_num_overlapping() {
        let s = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";
        assert_eq!(num_overlapping(s), 4);
    }
}
