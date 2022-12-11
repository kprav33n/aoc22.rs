use std::collections::VecDeque;

pub struct Monkey {
    items: VecDeque<i64>,
    operation: fn(i64) -> i64,
    test: fn(i64) -> bool,
    on_true: usize,
    on_false: usize,
    num_inspections: usize,
}

impl Monkey {
    fn turn(&mut self, manage: fn(i64) -> i64) -> Vec<(usize, i64)> {
        let mut throws: Vec<(usize, i64)> = Vec::new();
        while !self.items.is_empty() {
            let item = self.items.pop_front().unwrap();
            let mut new = (self.operation)(item);
            new = manage(new);
            if (self.test)(new) {
                throws.push((self.on_true, new));
            } else {
                throws.push((self.on_false, new));
            }
            self.num_inspections += 1;
        }
        throws
    }
}

pub fn input() -> Vec<Monkey> {
    vec![
        Monkey {
            items: VecDeque::from_iter(vec![66, 79]),
            operation: |old| old * 11,
            test: |worry| worry % 7 == 0,
            on_true: 6,
            on_false: 7,
            num_inspections: 0,
        },
        Monkey {
            items: VecDeque::from_iter(vec![84, 94, 94, 81, 98, 75]),
            operation: |old| old * 17,
            test: |worry| worry % 13 == 0,
            on_true: 5,
            on_false: 2,
            num_inspections: 0,
        },
        Monkey {
            items: VecDeque::from_iter(vec![85, 79, 59, 64, 79, 95, 67]),
            operation: |old| old + 8,
            test: |worry| worry % 5 == 0,
            on_true: 4,
            on_false: 5,
            num_inspections: 0,
        },
        Monkey {
            items: VecDeque::from_iter(vec![70]),
            operation: |old| old + 3,
            test: |worry| worry % 19 == 0,
            on_true: 6,
            on_false: 0,
            num_inspections: 0,
        },
        Monkey {
            items: VecDeque::from_iter(vec![57, 69, 78, 78]),
            operation: |old| old + 4,
            test: |worry| worry % 2 == 0,
            on_true: 0,
            on_false: 3,
            num_inspections: 0,
        },
        Monkey {
            items: VecDeque::from_iter(vec![65, 92, 60, 74, 72]),
            operation: |old| old + 7,
            test: |worry| worry % 11 == 0,
            on_true: 3,
            on_false: 4,
            num_inspections: 0,
        },
        Monkey {
            items: VecDeque::from_iter(vec![77, 91, 91]),
            operation: |old| old * old,
            test: |worry| worry % 17 == 0,
            on_true: 1,
            on_false: 7,
            num_inspections: 0,
        },
        Monkey {
            items: VecDeque::from_iter(vec![76, 58, 57, 55, 67, 77, 54, 99]),
            operation: |old| old + 6,
            test: |worry| worry % 3 == 0,
            on_true: 2,
            on_false: 1,
            num_inspections: 0,
        },
    ]
}

pub fn monkey_business_level(monkeys: &mut Vec<Monkey>) -> usize {
    for _ in 0..20 {
        for i in 0..monkeys.len() {
            let moves = monkeys[i].turn(|worry| worry / 3);
            for (to, item) in moves {
                monkeys[to].items.push_back(item);
            }
        }
    }
    let mut inspections: Vec<usize> = monkeys.iter().map(|m| m.num_inspections).collect();
    inspections.sort_by(|a, b| b.cmp(a));
    inspections[0] * inspections[1]
}

pub fn monkey_business_level_p2(monkeys: &mut Vec<Monkey>) -> usize {
    for _ in 0..10000 {
        for i in 0..monkeys.len() {
            // let moves = monkeys[i].turn(|worry| worry % (23 * 19 * 13 * 17));
            let moves = monkeys[i].turn(|worry| worry % (7 * 13 * 5 * 19 * 2 * 11 * 17 * 3));
            for (to, item) in moves {
                monkeys[to].items.push_back(item);
            }
        }
        let inspections: Vec<usize> = monkeys.iter().map(|m| m.num_inspections).collect();
        println!("{:?}", inspections);
    }
    let mut inspections: Vec<usize> = monkeys.iter().map(|m| m.num_inspections).collect();
    inspections.sort_by(|a, b| b.cmp(a));
    inspections[0] * inspections[1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_monkey_throw() {
        let mut monkey = Monkey {
            items: VecDeque::from_iter(vec![79, 98]),
            operation: |old| old * 19,
            test: |worry| worry % 23 == 0,
            on_true: 2,
            on_false: 3,
            num_inspections: 0,
        };
        assert_eq!(monkey.turn(|worry| worry / 3), vec![(3, 500), (3, 620)]);
    }

    fn test_input() -> Vec<Monkey> {
        vec![
            Monkey {
                items: VecDeque::from_iter(vec![79, 98]),
                operation: |old| old * 19,
                test: |worry| worry % 23 == 0,
                on_true: 2,
                on_false: 3,
                num_inspections: 0,
            },
            Monkey {
                items: VecDeque::from_iter(vec![54, 65, 75, 74]),
                operation: |old| old + 6,
                test: |worry| worry % 19 == 0,
                on_true: 2,
                on_false: 0,
                num_inspections: 0,
            },
            Monkey {
                items: VecDeque::from_iter(vec![79, 60, 97]),
                operation: |old| old * old,
                test: |worry| worry % 13 == 0,
                on_true: 1,
                on_false: 3,
                num_inspections: 0,
            },
            Monkey {
                items: VecDeque::from_iter(vec![74]),
                operation: |old| old + 3,
                test: |worry| worry % 17 == 0,
                on_true: 0,
                on_false: 1,
                num_inspections: 0,
            },
        ]
    }

    #[test]
    fn test_monkey_business_level() {
        // TODO: Parse later.
        //         let input = "Monkey 0:
        //   Starting items: 79, 98
        //   Operation: new = old * 19
        //   Test: divisible by 23
        //     If true: throw to monkey 2
        //     If false: throw to monkey 3

        // Monkey 1:
        //   Starting items: 54, 65, 75, 74
        //   Operation: new = old + 6
        //   Test: divisible by 19
        //     If true: throw to monkey 2
        //     If false: throw to monkey 0

        // Monkey 2:
        //   Starting items: 79, 60, 97
        //   Operation: new = old * old
        //   Test: divisible by 13
        //     If true: throw to monkey 1
        //     If false: throw to monkey 3

        // Monkey 3:
        //   Starting items: 74
        //   Operation: new = old + 3
        //   Test: divisible by 17
        //     If true: throw to monkey 0
        //     If false: throw to monkey 1";

        assert_eq!(monkey_business_level(&mut test_input()), 10605)
    }

    #[test]
    fn test_monkey_business_level_p2() {
        assert_eq!(monkey_business_level_p2(&mut test_input()), 2713310158)
    }
}
