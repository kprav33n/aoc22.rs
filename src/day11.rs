use std::collections::VecDeque;
use std::str::FromStr;

enum Operand {
    Old,
    Value(i64),
}

enum Operator {
    Add(Operand),
    Multiply(Operand),
}

pub struct Monkey {
    items: VecDeque<i64>,
    operation: Operator,
    test_div_by: i64,
    on_true: usize,
    on_false: usize,
    num_inspections: usize,
}

impl FromStr for Monkey {
    type Err = ();

    fn from_str(s: &str) -> Result<Monkey, Self::Err> {
        let lines: Vec<&str> = s.trim().lines().skip(1).collect();

        let items_str = lines[0].trim().strip_prefix("Starting items: ").unwrap();
        let items: Vec<i64> = items_str.split(", ").map(|i| i.parse().unwrap()).collect();

        let expr_str = lines[1].trim().strip_prefix("Operation: new = ").unwrap();
        let (op_str, val_str) = expr_str.rsplit_once(' ').unwrap();
        let operation = match val_str {
            "old" => match op_str {
                "old +" => Operator::Add(Operand::Old),
                "old *" => Operator::Multiply(Operand::Old),
                _ => unreachable!(),
            },
            v => match op_str {
                "old +" => Operator::Add(Operand::Value(v.parse().unwrap())),
                "old *" => Operator::Multiply(Operand::Value(v.parse().unwrap())),
                _ => unreachable!(),
            },
        };

        let div_str = lines[2].trim().strip_prefix("Test: divisible by ").unwrap();
        let test_div_by: i64 = div_str.parse().unwrap();

        let on_true = lines[3]
            .trim()
            .strip_prefix("If true: throw to monkey ")
            .unwrap()
            .parse()
            .unwrap();
        let on_false = lines[4]
            .trim()
            .strip_prefix("If false: throw to monkey ")
            .unwrap()
            .parse()
            .unwrap();

        Ok(Monkey {
            items: VecDeque::from_iter(items),
            operation,
            test_div_by,
            on_true,
            on_false,
            num_inspections: 0,
        })
    }
}

impl Monkey {
    fn turn<F>(&mut self, manage: F) -> Vec<(usize, i64)>
    where
        F: Fn(i64) -> i64,
    {
        let mut throws: Vec<(usize, i64)> = Vec::new();
        while !self.items.is_empty() {
            let item = self.items.pop_front().unwrap();
            let mut new = match &self.operation {
                Operator::Add(o) => match o {
                    Operand::Old => item + item,
                    Operand::Value(v) => item + v,
                },
                Operator::Multiply(o) => match o {
                    Operand::Old => item * item,
                    Operand::Value(v) => item * v,
                },
            };
            new = manage(new);
            if new % self.test_div_by == 0 {
                throws.push((self.on_true, new));
            } else {
                throws.push((self.on_false, new));
            }
            self.num_inspections += 1;
        }
        throws
    }
}

pub fn monkey_business_level(s: &str, rounds: usize) -> usize {
    let mut monkeys: Vec<Monkey> = s.trim().split("\n\n").map(|s| s.parse().unwrap()).collect();
    let factor: i64 = monkeys.iter().map(|m| m.test_div_by).product();
    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            let moves = if rounds > 20 {
                monkeys[i].turn(|worry| worry % factor)
            } else {
                monkeys[i].turn(|worry| worry / 3)
            };
            for (to, item) in moves {
                monkeys[to].items.push_back(item);
            }
        }
    }
    let mut inspections: Vec<usize> = monkeys.iter().map(|m| m.num_inspections).collect();
    inspections.sort_by(|a, b| b.cmp(a));
    inspections[0] * inspections[1]
}

pub fn monkey_business_level_p1(s: &str) -> usize {
    monkey_business_level(s, 20)
}

pub fn monkey_business_level_p2(s: &str) -> usize {
    monkey_business_level(s, 10000)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_monkey_throw() {
        let mut monkey = Monkey {
            items: VecDeque::from_iter(vec![79, 98]),
            operation: Operator::Multiply(Operand::Value(19)),
            test_div_by: 23,
            on_true: 2,
            on_false: 3,
            num_inspections: 0,
        };
        assert_eq!(monkey.turn(|worry| worry / 3), vec![(3, 500), (3, 620)]);
    }

    static INPUT: &str = "Monkey 0:
          Starting items: 79, 98
          Operation: new = old * 19
          Test: divisible by 23
            If true: throw to monkey 2
            If false: throw to monkey 3

        Monkey 1:
          Starting items: 54, 65, 75, 74
          Operation: new = old + 6
          Test: divisible by 19
            If true: throw to monkey 2
            If false: throw to monkey 0

        Monkey 2:
          Starting items: 79, 60, 97
          Operation: new = old * old
          Test: divisible by 13
            If true: throw to monkey 1
            If false: throw to monkey 3

        Monkey 3:
          Starting items: 74
          Operation: new = old + 3
          Test: divisible by 17
            If true: throw to monkey 0
            If false: throw to monkey 1";

    #[test]
    fn test_monkey_business_level_p1() {
        assert_eq!(monkey_business_level_p1(INPUT), 10605)
    }

    #[test]
    fn test_monkey_business_level_p2() {
        assert_eq!(monkey_business_level_p2(INPUT), 2713310158)
    }
}
