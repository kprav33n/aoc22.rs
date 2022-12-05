// Parse a stack row and return list of crates.
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

// Parse a step and return number of crates, from stack and to stack.
fn parse_step(s: &str) -> (usize, usize, usize) {
    let v: Vec<&str> = s.split(' ').collect();
    (
        v[1].parse().unwrap(),
        v[3].parse().unwrap(),
        v[5].parse().unwrap(),
    )
}

// Parse stack representation and return a vector of stacks.
fn parse_stacks(s: &str) -> Vec<Vec<char>> {
    let mut stack_lines: Vec<&str> = s.split('\n').collect();
    let label_line = stack_lines.pop().unwrap();
    let num_stacks = (label_line.len() + 2) / 4;
    let stack_rows: Vec<Vec<Option<char>>> = stack_lines
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
    stacks
}

// An Executor can execute the given steps on the given stacks.
type Executor = fn(&Vec<(usize, usize, usize)>, &mut [Vec<char>]);

// Execute steps according to part 1 of the problem definition.
fn execute_steps_p1(steps: &Vec<(usize, usize, usize)>, stacks: &mut [Vec<char>]) {
    for (n, from, to) in steps {
        for _ in 0..*n {
            let ch = stacks[from - 1].pop();
            stacks[to - 1].push(ch.unwrap());
        }
    }
}

// Execute steps according to part 2 of the problem defiition.
fn execute_steps_p2(steps: &Vec<(usize, usize, usize)>, stacks: &mut [Vec<char>]) {
    for (n, from, to) in steps {
        let from_len = stacks[from - 1].len();
        let mut drained: Vec<char> = stacks[from - 1].drain(from_len - n..).collect();
        stacks[to - 1].append(&mut drained);
    }
}

// Parse stacks, steps and compute top of the stack after executing steps.
pub fn top_of_stack(s: &str, executor: Executor) -> String {
    let Some((stacks_str, steps_str)) = s.split_once("\n\n") else {
        unreachable!();
    };
    let mut stacks: Vec<Vec<char>> = parse_stacks(stacks_str);
    let steps = steps_str.trim().split('\n').map(parse_step).collect();
    executor(&steps, &mut stacks);
    stacks.iter().map(|v| v.last().unwrap()).collect()
}

// Parse stacks, steps and compute top of tack after executing steps according
// to part 1 of the problem.
pub fn top_of_stack_p1(s: &str) -> String {
    top_of_stack(s, execute_steps_p1)
}

// Parse stacks, steps and compute top of tack after executing steps according
// to part 2 of the problem.
pub fn top_of_stack_p2(s: &str) -> String {
    top_of_stack(s, execute_steps_p2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_top_of_stack_p1() {
        let s = "    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";
        assert_eq!(top_of_stack_p1(s), "CMZ");
    }

    #[test]
    fn test_top_of_stack_p2() {
        let s = "    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";
        assert_eq!(top_of_stack_p2(s), "MCD");
    }
}
