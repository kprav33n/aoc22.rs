fn parse_stack_row(n: usize, s: &str) -> Vec<Option<char>> {
    let width = n * 4 - 1;
    let padded: Vec<char> = format!("{:<width$}", s).chars().collect();
    let mut v: Vec<Option<char>> = Vec::new();
    for i in (1..width).step_by(4) {
        let ch = padded[i];
        if ch == ' ' {
            v.push(None)
        } else {
            v.push(Some(ch));
        }
    }
    v
}

fn parse_step(s: &str) -> (usize, usize, usize) {
    let v: Vec<&str> = s.split(' ').collect();
    (
        v[1].parse().unwrap(),
        v[3].parse().unwrap(),
        v[5].parse().unwrap(),
    )
}

pub fn top_of_stack(s: &str) -> String {
    let v: Vec<&str> = s.split("\n\n").collect();
    let mut pos: Vec<&str> = v[0].split('\n').collect();
    let label_row = pos.pop().unwrap();
    let num_stacks = (label_row.len() + 2) / 4;
    let stack_rows: Vec<Vec<Option<char>>> = pos
        .iter()
        .rev()
        .map(|s| parse_stack_row(num_stacks, s))
        .collect();
    let mut stacks: Vec<Vec<char>> = vec![Vec::new(); num_stacks];
    for row in stack_rows {
        for (i, column) in row.iter().enumerate() {
            if let Some(x) = column {
                stacks[i].push(*x);
            }
        }
    }
    let steps = v[1].trim().split('\n').map(parse_step);
    for (n, from, to) in steps {
        for _ in 0..n {
            let ch = stacks[from - 1].pop();
            stacks[to - 1].push(ch.unwrap());
        }
    }

    let mut tos: Vec<char> = Vec::new();
    for stack in stacks {
        tos.push(*stack.last().unwrap());
    }
    tos.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_top_of_stack() {
        let s = "    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";
        assert_eq!(top_of_stack(s), "CMZ");
    }
}
