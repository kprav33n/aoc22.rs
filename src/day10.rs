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

pub fn sum_of_signal_strengths(s: &str) -> i64 {
    let instructions: Vec<Instruction> = s.trim().split('\n').map(|s| s.parse().unwrap()).collect();
    let mut signal = 1;
    let mut signals: Vec<i64> = vec![0];
    for instruction in &instructions {
        match instruction {
            Instruction::Noop => {
                signals.push(signal);
            }
            Instruction::AddX(n) => {
                signals.push(signal);
                signals.push(signal);
                signal += n;
            }
        }
    }
    signals.push(signal);

    let mut sum = 0;
    let starting = 20;
    let every = 40;
    for (i, signal) in signals.iter().enumerate().skip(starting) {
        if (i == starting) || (i - starting) % every == 0 {
            sum += i as i64 * signal;
        }
    }
    sum
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

    #[test]
    fn test_num_tail_positions_p1() {
        let input = "addx 15
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
        assert_eq!(sum_of_signal_strengths(input), 13140)
    }
}
