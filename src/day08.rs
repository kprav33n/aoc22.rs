pub fn num_visible_trees(s: &str) -> usize {
    let trees: Vec<Vec<u8>> = s
        .trim()
        .split('\n')
        .map(|l| l.as_bytes().iter().map(|b| b - '0' as u8).collect())
        .collect();
    let num_rows = trees.len();
    let num_columns = trees[0].len();
    let mut visible = vec![vec![false; num_columns]; num_rows];

    // Initialize perimeter.
    for i in 0..num_rows {
        visible[i][0] = true;
        visible[i][num_columns - 1] = true;
    }
    for j in 0..num_columns {
        visible[0][j] = true;
        visible[num_rows - 1][j] = true;
    }

    for i in 1..(num_rows - 1) {
        for j in 1..(num_columns - 1) {
            // From top?
            let mut v = true;
            for k in (0..i).rev() {
                if trees[i][j] <= trees[k][j] {
                    v = false;
                    break;
                }
            }
            if v {
                visible[i][j] = true;
                continue;
            }

            // From left?
            let mut v = true;
            for k in (0..j).rev() {
                if trees[i][j] <= trees[i][k] {
                    v = false;
                    break;
                }
            }
            if v {
                visible[i][j] = true;
                continue;
            }

            // From bottom?
            let mut v = true;
            for k in i + 1..num_rows {
                if trees[i][j] <= trees[k][j] {
                    v = false;
                    break;
                }
            }
            if v {
                visible[i][j] = true;
                continue;
            }

            // From right?
            let mut v = true;
            for k in j + 1..num_columns {
                if trees[i][j] <= trees[i][k] {
                    v = false;
                    break;
                }
            }
            if v {
                visible[i][j] = true;
                continue;
            }
        }
    }

    let mut count = 0;
    for i in 0..num_rows {
        for j in 0..num_columns {
            if visible[i][j] {
                count += 1;
            }
        }
    }
    count
}

pub fn highest_scenic_score(s: &str) -> usize {
    let trees: Vec<Vec<u8>> = s
        .trim()
        .split('\n')
        .map(|l| l.as_bytes().iter().map(|b| b - '0' as u8).collect())
        .collect();
    let num_rows = trees.len();
    let num_columns = trees[0].len();
    let mut score = vec![vec![0; num_columns]; num_rows];

    for i in 0..num_rows {
        for j in 0..num_columns {
            let mut top = 0;
            for k in (0..i).rev() {
                top += 1;
                if trees[i][j] <= trees[k][j] {
                    break;
                }
            }

            let mut left = 0;
            for k in (0..j).rev() {
                left += 1;
                if trees[i][j] <= trees[i][k] {
                    break;
                }
            }

            let mut bottom = 0;
            for k in i + 1..num_rows {
                bottom += 1;
                if trees[i][j] <= trees[k][j] {
                    break;
                }
            }

            let mut right = 0;
            for k in j + 1..num_columns {
                right += 1;
                if trees[i][j] <= trees[i][k] {
                    break;
                }
            }
            score[i][j] = top * left * bottom * right;
        }
    }

    let mut max = 0;
    for i in 0..num_rows {
        for j in 0..num_columns {
            if score[i][j] > max {
                max = score[i][j]
            }
        }
    }
    max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_visible_trees() {
        let input = "30373
25512
65332
33549
35390";
        assert_eq!(num_visible_trees(input), 21)
    }

    #[test]
    fn test_highest_scenic_score() {
        let input = "30373
25512
65332
33549
35390";
        assert_eq!(highest_scenic_score(input), 8)
    }
}
