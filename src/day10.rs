use std::str::FromStr;

#[derive(Debug)]
enum Instruction {
    Noop,
    AddX(i64),
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Instruction, Self::Err> {
        let v: Vec<&str> = s.split(' ').collect();

        match v[0] {
            "noop" => Ok(Instruction::Noop),
            "addx" => {
                let Ok(n) = v[1].parse() else {
                            return Err(());
                        };

                Ok(Instruction::AddX(n))
            }
            _ => Err(()),
        }
    }
}

// Return register values after executing the given program.
fn register_values(s: &str) -> Vec<i64> {
    let instructions: Vec<Instruction> = s.trim().split('\n').map(|s| s.parse().unwrap()).collect();
    let mut x = 1;
    let mut values: Vec<i64> = vec![0];
    for instruction in &instructions {
        match instruction {
            Instruction::Noop => {
                values.push(x);
            }
            Instruction::AddX(n) => {
                values.push(x);
                values.push(x);
                x += n;
            }
        }
    }
    values
}

// Compute the sum of signal strengths for the given program.
pub fn sum_of_signal_strengths(s: &str) -> i64 {
    register_values(s)
        .iter()
        .enumerate()
        .skip(20)
        .step_by(40)
        .fold(0, |acc, (i, signal)| acc + i as i64 * signal)
}

// Render image on CRT based on the given program.
pub fn render_image(s: &str) -> String {
    let values = register_values(s);
    const CRT_WIDTH: usize = 40;
    let crt_height = (values.len() - 1) / CRT_WIDTH;
    let mut screen = vec![vec!['#'; CRT_WIDTH]; crt_height];
    for (i, &value) in values.iter().skip(1).enumerate().skip(1) {
        let (row, col) = (i / CRT_WIDTH, i % CRT_WIDTH);
        screen[row][col] = if (col as i64 - value).abs() <= 1 {
            '#'
        } else {
            '.'
        }
    }
    screen
        .iter()
        .map(|v| v.iter().collect::<String>())
        .collect::<Vec<String>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn warmup() {
        let input = "noop
addx 3
addx -5";
        assert_eq!(sum_of_signal_strengths(input), 0);
    }

    const INPUT: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

    #[test]
    fn test_sum_of_signal_strengths() {
        assert_eq!(sum_of_signal_strengths(INPUT), 13140)
    }

    #[test]
    fn test_render_image() {
        let output = "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....";
        assert_eq!(render_image(INPUT), output)
    }
}
