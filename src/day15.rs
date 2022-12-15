use std::str::FromStr;

#[derive(Clone, Debug, Eq, PartialEq)]
struct Position {
    x: i64,
    y: i64,
}

impl Position {
    fn new(x: i64, y: i64) -> Self {
        Position { x, y }
    }

    fn manhattan_distance(&self, other: &Position) -> i64 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

#[derive(Clone, Debug)]
struct ReportEntry {
    sensor: Position,
    beacon: Position,
    distance: i64,
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
        let (sensor, beacon) = s.split_once(':').ok_or(ParseReportEntryError)?;
        let (sx, sy) = sensor.split_once(',').ok_or(ParseReportEntryError)?;
        let (bx, by) = beacon.split_once(',').ok_or(ParseReportEntryError)?;
        Ok(ReportEntry::new(
            Position::new(sx[12..].parse::<i64>()?, sy[3..].parse::<i64>()?),
            Position::new(bx[24..].parse::<i64>()?, by[3..].parse::<i64>()?),
        ))
    }
}

impl ReportEntry {
    fn new(sensor: Position, beacon: Position) -> Self {
        let distance = sensor.manhattan_distance(&beacon);
        ReportEntry {
            sensor,
            beacon,
            distance,
        }
    }

    fn is_in_range(&self, position: &Position) -> bool {
        if position.clone() == self.beacon {
            return false;
        }
        self.sensor.manhattan_distance(position) <= self.distance
    }
}

// Return the number of empty positions in the given row.
pub fn num_empty_positions(s: &str, row: i64) -> usize {
    let report: Vec<ReportEntry> = s
        .trim()
        .lines()
        .map(|s| s.parse().unwrap_or_else(|_| panic!("failed to parse entry: {}", s)))
        .collect();
    let (min_x, max_x) = report
        .iter()
        .map(|entry| {
            (
                entry.sensor.x - entry.distance,
                entry.sensor.x + entry.distance,
            )
        })
        .fold((i64::MAX, i64::MIN), |(min, max), (l, r)| {
            (l.min(min), r.max(max))
        });
    (min_x..=max_x)
        .filter(|&x| {
            report
                .iter()
                .any(|entry| entry.is_in_range(&Position::new(x, row)))
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";

    #[test]
    fn test_num_empty_positions() {
        assert_eq!(num_empty_positions(INPUT, 10), 26);
    }
}
