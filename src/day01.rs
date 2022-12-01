/// Find the maximum total calories carried by any elf.
pub fn max_total_calories(s: &str) -> i64 {
    s.trim()
        .split("\n\n")
        .map(|elf| {
            elf.trim()
                .split('\n')
                .map(|item| item.trim().parse::<i64>().unwrap())
                .sum()
        })
        .max()
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_total_calories_from_problem() {
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
}
