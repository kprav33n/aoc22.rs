use rsmt2::Solver;
use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;

#[derive(Clone)]
enum Operation {
    Multiply,
    Divide,
    Add,
    Subtract,
}

impl FromStr for Operation {
    type Err = ();

    fn from_str(s: &str) -> Result<Operation, Self::Err> {
        match s {
            "*" => Ok(Self::Multiply),
            "/" => Ok(Self::Divide),
            "+" => Ok(Self::Add),
            "-" => Ok(Self::Subtract),
            _ => Err(()),
        }
    }
}

impl fmt::Display for Operation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Multiply => write!(f, "*"),
            Self::Divide => write!(f, "/"),
            Self::Add => write!(f, "+"),
            Self::Subtract => write!(f, "-"),
        }
    }
}

impl Operation {
    fn execute(&self, left: i64, right: i64) -> i64 {
        match self {
            Self::Multiply => left * right,
            Self::Divide => left / right,
            Self::Add => left + right,
            Self::Subtract => left - right,
        }
    }
}

#[derive(Clone)]
enum Value {
    Integer(i64),
    Op(Operation, String, String),
}

impl FromStr for Value {
    type Err = ();

    fn from_str(s: &str) -> Result<Value, Self::Err> {
        let tokens: Vec<&str> = s.trim().split(' ').collect();
        if tokens.len() == 1 {
            Ok(Value::Integer(tokens[0].parse().ok().ok_or(())?))
        } else if tokens.len() == 3 {
            Ok(Value::Op(
                tokens[1].parse().ok().ok_or(())?,
                tokens[0].to_string(),
                tokens[2].to_string(),
            ))
        } else {
            Err(())
        }
    }
}

struct Job {
    monkey: String,
    value: Value,
}

impl FromStr for Job {
    type Err = ();

    fn from_str(s: &str) -> Result<Job, Self::Err> {
        let (monkey, value) = s.trim().split_once(':').ok_or(())?;
        let monkey = monkey.to_string();
        let value: Value = value.parse()?;
        Ok(Job { monkey, value })
    }
}

struct JobTable {
    jobs: HashMap<String, Job>,
}

impl FromStr for JobTable {
    type Err = ();

    fn from_str(s: &str) -> Result<JobTable, Self::Err> {
        let mut jobs: HashMap<String, Job> = HashMap::new();
        for line in s.trim().lines() {
            let job: Job = line.parse()?;
            jobs.insert(job.monkey.clone(), job);
        }
        Ok(JobTable { jobs })
    }
}

impl JobTable {
    fn yells(&self, monkey: &str) -> i64 {
        match &self.jobs[monkey].value {
            Value::Integer(v) => *v,
            Value::Op(operation, left, right) => {
                operation.execute(self.yells(left), self.yells(right))
            }
        }
    }

    fn i_yell(&self) -> i64 {
        let root: String = "root".to_string();
        let Value::Op(_, left, right) = self.jobs[&root].value.clone() else {
            unreachable!();
        };
        let equation = format!("(= {} {})", self.equation(&left), self.equation(&right));
        let mut solver = Solver::default_z3(()).unwrap();
        solver.declare_const("x", "Int").unwrap();
        solver.assert(equation).unwrap();
        assert!(solver.check_sat().unwrap());
        let model = solver.get_model().unwrap();
        assert!(model.len() == 1);
        assert!(model[0].0 == "x");
        model[0].3.parse().unwrap()
    }

    fn equation(&self, monkey: &str) -> String {
        if monkey == "humn" {
            return "x".to_string();
        }

        match &self.jobs[monkey].value {
            Value::Integer(v) => v.to_string(),
            Value::Op(operation, left, right) => {
                format!(
                    "({} {} {})",
                    operation,
                    self.equation(left),
                    self.equation(right)
                )
            }
        }
    }
}

pub fn root_yells(s: &str) -> i64 {
    let table: JobTable = s.parse().unwrap();
    table.yells("root")
}

pub fn i_yell(s: &str) -> i64 {
    let table: JobTable = s.parse().unwrap();
    table.i_yell()
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32";

    #[test]
    fn test_root_yells() {
        assert_eq!(root_yells(INPUT), 152);
    }

    #[test]
    fn test_i_yell() {
        assert_eq!(i_yell(INPUT), 301);
    }
}
