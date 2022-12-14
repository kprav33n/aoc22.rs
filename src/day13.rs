use serde_json::Value;
use std::cmp::Ordering;

// Establishes Ordering for the given two Values.
fn compare(left: Value, right: Value) -> Ordering {
    match (left.clone(), right.clone()) {
        (Value::Number(l), Value::Number(r)) => l.as_i64().unwrap().cmp(&r.as_i64().unwrap()),
        (Value::Array(l), Value::Array(r)) => {
            for (lv, rv) in l.iter().zip(r.iter()) {
                let order = compare(lv.clone(), rv.clone());
                if order != Ordering::Equal {
                    return order;
                }
            }
            l.len().cmp(&r.len())
        }
        (Value::Number(_), r) => compare(Value::Array(vec![left]), r),
        (l, Value::Number(_)) => compare(l, Value::Array(vec![right])),
        _ => unreachable!(),
    }
}

// Return the sum of indices of pairs who are in the right order.
pub fn sum_right_indices(s: &str) -> usize {
    s.trim()
        .split("\n\n")
        .map(|s| {
            let Some((left, right)) = s.split_once('\n') else {
                unreachable!();
        };
            let (Ok(left), Ok(right)) = (serde_json::from_str(left),
                                         serde_json::from_str(right)) else {
                unreachable!();
        };
            compare(left, right)
        })
        .enumerate()
        .fold(0, |acc, (i, ord)| {
            if ord == Ordering::Less {
                acc + i + 1
            } else {
                acc
            }
        })
}

// Return the decoder key.
pub fn decoder_key(s: &str) -> usize {
    let mut packets: Vec<Value> = s
        .trim()
        .replace("\n\n", "\n")
        .lines()
        .map(|s| serde_json::from_str(s).unwrap())
        .collect();
    let m1: Value = serde_json::from_str("[[2]]").unwrap();
    let m2: Value = serde_json::from_str("[[6]]").unwrap();
    packets.extend([m1.clone(), m2.clone()]);
    packets.sort_by(|l, r| compare(l.clone(), r.clone()));

    packets.iter().enumerate().fold(1, |acc, (i, v)| {
        if v.clone() == m1 || v.clone() == m2 {
            acc * (i + 1)
        } else {
            acc
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compare() {
        let tests = [
            // Integers.
            ("2", "3", Ordering::Less),
            ("4", "3", Ordering::Greater),
            ("4", "4", Ordering::Equal),
            // Lists.
            ("[1, 1, 3, 1, 1]", "[1, 1, 5, 1, 1]", Ordering::Less),
            ("[1, 1, 6, 1, 1]", "[1, 1, 5, 1, 1]", Ordering::Greater),
            ("[1, 1, 6, 1, 1]", "[1, 1, 6, 1, 1]", Ordering::Equal),
            ("[1, 1, 6, 1]", "[1, 1, 6, 1, 1]", Ordering::Less),
            // Mixed.
            ("[9]", "[[8, 7, 6]]", Ordering::Greater),
            ("[[4, 4], 4, 4]", "[[4, 4], 4, 4, 4]", Ordering::Less),
            ("[[[]]]", "[[]]", Ordering::Greater),
            (
                "[1, [2, [3, [4, [5, 6, 7]]]], 8, 9]",
                "[1, [2, [3, [4, [5, 6, 0]]]], 8, 9]",
                Ordering::Greater,
            ),
            ("[2, 3, 4]", "4", Ordering::Less),
        ];

        for (i, (left, right, order)) in tests.iter().enumerate() {
            let (Ok(left), Ok(right)) = (serde_json::from_str(left),
                                         serde_json::from_str(right)) else {
                unreachable!();
            };
            assert_eq!(compare(left, right), order.clone(), "test {}", i);
        }
    }

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
    fn test_sum_right_indices() {
        assert_eq!(sum_right_indices(INPUT), 13);
    }

    #[test]
    fn test_decoder_key() {
        assert_eq!(decoder_key(INPUT), 140);
    }
}
