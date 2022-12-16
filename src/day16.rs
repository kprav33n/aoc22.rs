use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::str::FromStr;

#[derive(Debug)]
struct ReportEntry {
    valve: String,
    flow_rate: i64,
    leads_to: Vec<String>,
}

#[derive(Debug)]
struct ParseReportEntryError;

impl From<std::num::ParseIntError> for ParseReportEntryError {
    fn from(_: std::num::ParseIntError) -> ParseReportEntryError {
        ParseReportEntryError
    }
}

impl FromStr for ReportEntry {
    type Err = ParseReportEntryError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"Valve (\w*) has flow rate=(\d*); tunnel[s]* lead[s]* to valve[s]* (.*)"
            )
            .unwrap();
        }

        let captures = RE.captures(s).ok_or(ParseReportEntryError)?;
        let valve: String = captures
            .get(1)
            .ok_or(ParseReportEntryError)?
            .as_str()
            .to_string();
        let flow_rate: i64 = captures
            .get(2)
            .ok_or(ParseReportEntryError)?
            .as_str()
            .parse()?;
        let leads_to: Vec<String> = captures
            .get(3)
            .ok_or(ParseReportEntryError)?
            .as_str()
            .to_string()
            .split(", ")
            .map(String::from)
            .collect();
        Ok(ReportEntry {
            valve,
            flow_rate,
            leads_to,
        })
    }
}

#[derive(Debug, Clone)]
struct Valve {
    id: i64,
    flow_rate: i64,
    leads_to: Vec<i64>,
}

impl Valve {
    fn from_report_entry(entry: &ReportEntry, valve_id_table: &HashMap<String, i64>) -> Self {
        Valve {
            id: valve_id_table[&entry.valve],
            flow_rate: entry.flow_rate,
            leads_to: entry.leads_to.iter().map(|v| valve_id_table[v]).collect(),
        }
    }
}

#[derive(Debug)]
struct ValveTable {
    ids: HashMap<String, i64>,
    valves: HashMap<i64, Valve>,
}

impl ValveTable {
    fn from_report_entries(entries: &[ReportEntry]) -> Self {
        let mut names: Vec<String> = entries.iter().map(|e| e.valve.clone()).collect();
        names.sort();
        let ids: HashMap<String, i64> = names
            .iter()
            .enumerate()
            .map(|(i, n)| (n.clone(), 1 << i))
            .collect();
        let valves: HashMap<i64, Valve> = entries
            .iter()
            .map(|e| (ids[&e.valve], Valve::from_report_entry(e, &ids)))
            .collect();
        ValveTable { ids, valves }
    }

    fn get_by_name(&self, name: &String) -> Valve {
        self.valves[&self.ids[name]].clone()
    }

    // FIXME: This is still crazy slow. Took 175s on M1 Max. Optimize.
    // Irrespective, this won't cut it for part 2.
    // Courtesy: u/morgoth1145
    fn max_released_pressure(&self) -> i64 {
        const DURATION: i64 = 30;
        let mut states: HashSet<(i64, i64, i64)> = HashSet::new();
        states.insert((self.get_by_name(&String::from("AA")).id, 0, 0));
        // let mut states = vec![(self.get_by_name(&String::from("AA")).id, 0, 0)];
        let mut best: HashMap<(i64, i64), i64> = HashMap::new();
        for t in 1..=DURATION {
            // println!("time: {}, num_states: {}", t, states.len());
            // println!(
            //     "time: {}, num_states: {}, states: {:?}",
            //     t,
            //     states.len(),
            //     states
            // );
            let mut next_states: HashSet<(i64, i64, i64)> = HashSet::new();
            // let mut next_states: Vec<(i64, i64, i64)> = Vec::new();
            for (current, opened, pressure) in &states {
                if let Some(best_pressure) = best.get(&(*current, *pressure)) {
                    if best_pressure > pressure {
                        continue;
                    }
                }
                best.insert((*current, *pressure), *pressure);

                let valve = &self.valves[current];
                if current & opened == 0 && valve.flow_rate > 0 {
                    next_states.insert((
                        *current,
                        current | opened,
                        pressure + valve.flow_rate * (DURATION - t),
                    ));
                    // next_states.push((
                    //     *current,
                    //     current | opened,
                    //     pressure + valve.flow_rate * (DURATION - t),
                    // ));
                }
                for next in &valve.leads_to {
                    next_states.insert((*next, *opened, *pressure));
                    // next_states.push((*next, *opened, *pressure));
                }
            }
            states = next_states;
        }
        *states
            .iter()
            .map(|(_, _, pressure)| pressure)
            .max()
            .unwrap()
    }
}

pub fn max_released_pressure(s: &str) -> i64 {
    let entries: Vec<ReportEntry> = s
        .trim()
        .lines()
        .map(|s| {
            s.parse()
                .unwrap_or_else(|_| panic!("failed to parse entry: {}", s))
        })
        .collect();
    ValveTable::from_report_entries(&entries).max_released_pressure()
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";

    #[test]
    fn test_max_released_pressure() {
        assert_eq!(max_released_pressure(INPUT), 1651);
    }
}
