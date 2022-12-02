use std::collections::BinaryHeap;

/// Find the maximum total calories carried by any elf.
pub fn max_total_calories(s: &str) -> i64 {
    s.trim()
        .split("\n\n")
        .map(|elf| {
            elf.trim()
                .split('\n')
                .filter_map(|item| item.trim().parse::<i64>().ok())
                .sum()
        })
        .max()
        .unwrap_or(0)
}

/// Find the maximum total calories carried by any elf.
pub fn max3_total_calories(s: &str) -> i64 {
    let mut cals: BinaryHeap<i64> = s
        .trim()
        .split("\n\n")
        .map(|elf| {
            elf.trim()
                .split('\n')
                .filter_map(|item| item.trim().parse::<i64>().ok())
                .sum()
        })
        .collect();
    (0..3).filter_map(|_| cals.pop()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_total_calories() {
        let s = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";
        assert_eq!(max_total_calories(s), 24000);
    }

    #[test]
    fn test_max3_total_calories() {
        let s = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";
        assert_eq!(max3_total_calories(s), 45000);
    }
}
