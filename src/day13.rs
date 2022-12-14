// NOTE: This solution was implemented in Python and will be reimplemented in
// Rust later.
pub fn tbd(_s: &str) -> i64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

    #[test]
    fn test_tbd() {
        assert_eq!(tbd(INPUT), 0);
    }
}
