fn sum_grove_coordinates(s: &str, key: i64, num_iterations: usize) -> i64 {
    let nums: Vec<i64> = s
        .trim()
        .lines()
        .map(|s| s.parse::<i64>().unwrap() * key)
        .collect();
    let mut idxs: Vec<usize> = (0..nums.len()).collect();

    for _ in 0..num_iterations {
        for (i, &n) in nums.iter().enumerate() {
            let j = idxs.iter().position(|&n| n == i).unwrap();
            idxs.remove(j);
            let to = (j as i64 + n).rem_euclid(idxs.len() as i64) as usize;
            idxs.insert(to, i);
        }
    }

    let num_0_at = nums.iter().position(|&n| n == 0).unwrap();
    let idx_0_at = idxs.iter().position(|&n| n == num_0_at).unwrap();
    [1000, 2000, 3000]
        .iter()
        .map(|p| {
            let i = (idx_0_at + p) % nums.len();
            nums[idxs[i]]
        })
        .sum()
}

pub fn sum_grove_coordinates_p1(s: &str) -> i64 {
    sum_grove_coordinates(s, 1, 1)
}

pub fn sum_grove_coordinates_p2(s: &str) -> i64 {
    sum_grove_coordinates(s, 811589153, 10)
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "1
2
-3
3
-2
0
4";

    #[test]
    fn test_sum_grove_coordinates() {
        assert_eq!(sum_grove_coordinates_p1(INPUT), 3);
    }

    #[test]
    fn test_sum_grove_coordinates_p2() {
        assert_eq!(sum_grove_coordinates_p2(INPUT), 1623178306);
    }
}
