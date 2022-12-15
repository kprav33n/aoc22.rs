use std::fmt;
use std::str::FromStr;

#[derive(Clone, Debug)]
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

    fn neighbors_within(&self, distance: i64) -> Vec<Position> {
        let mut neighbors = Vec::new();
        for y in self.y - distance..=self.y + distance {
            let offset = distance - (y - self.y).abs();
            for x in self.x - offset..=self.x + offset {
                neighbors.push(Position::new(x, y));
            }
        }
        neighbors
    }
}

#[derive(Clone, Debug)]
enum Content {
    Unknown,
    Empty,
    Sensor,
    Beacon,
}

impl fmt::Display for Content {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Content::Unknown => ".",
            Content::Empty => "#",
            Content::Sensor => "S",
            Content::Beacon => "B",
        })
    }
}

#[derive(Clone, Debug)]
struct ReportEntry {
    sensor: Position,
    beacon: Position,
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
        Ok(ReportEntry {
            sensor: Position::new(sx[12..].parse::<i64>()?, sy[3..].parse::<i64>()?),
            beacon: Position::new(bx[24..].parse::<i64>()?, by[3..].parse::<i64>()?),
        })
    }
}

struct Map {
    report: Vec<ReportEntry>,
    contents: Vec<Vec<Content>>,
    min_x: i64,
    min_y: i64,
}

impl fmt::Debug for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, row) in self.contents.iter().enumerate() {
            if i > 0 {
                f.write_str("\n")?;
            }
            for content in row {
                f.write_str(&content.to_string())?;
            }
        }
        Ok(())
    }
}

impl Map {
    pub fn from_report(report: Vec<ReportEntry>) -> Self {
        let boundary = (i64::MAX, i64::MAX, i64::MIN, i64::MIN);
        let boundary = report
            .iter()
            .fold(boundary, |(min_x, min_y, max_x, max_y), entry| {
                (
                    min_x.min(entry.sensor.x).min(entry.beacon.x),
                    min_y.min(entry.sensor.y).min(entry.beacon.y),
                    max_x.max(entry.sensor.x).max(entry.beacon.x),
                    max_y.max(entry.sensor.y).max(entry.beacon.y),
                )
            });
        let max_distance = report
            .iter()
            .map(|e| e.beacon.manhattan_distance(&e.sensor))
            .max()
            .unwrap();
        let (min_x, min_y) = (boundary.0 - max_distance - 1, boundary.1 - max_distance - 1);
        let (max_x, max_y) = (boundary.2 + max_distance + 1, boundary.3 + max_distance + 1);

        let rows = (max_y - min_y) as usize;
        let columns = (max_x - min_x) as usize;
        let contents = vec![vec![Content::Unknown; columns]; rows];
        let mut map = Map {
            report,
            contents,
            min_x,
            min_y,
        };
        map.scan();
        map
    }

    pub fn index(&self, position: &Position) -> (usize, usize) {
        let (column, row) = ((position.x - self.min_x), (position.y - self.min_y));
        (row as usize, column as usize)
    }

    pub fn put(&mut self, position: &Position, content: Content) {
        let (row, column) = self.index(position);
        self.contents[row][column] = content;
    }

    pub fn get(&self, position: &Position) -> Content {
        let (row, column) = self.index(position);
        self.contents[row][column].clone()
    }

    pub fn scan(&mut self) {
        let entries = self.report.clone();
        for entry in &entries {
            self.put(&entry.sensor, Content::Sensor);
            self.put(&entry.beacon, Content::Beacon);
        }

        for entry in &entries {
            let distance = entry.sensor.manhattan_distance(&entry.beacon);
            let neighbors = entry.sensor.neighbors_within(distance);
            for neighbor in neighbors {
                if let Content::Unknown = self.get(&neighbor) {
                    self.put(&neighbor, Content::Empty);
                }
            }
        }
    }

    pub fn num_empty_positions(&self, row: i64) -> usize {
        let mut count = 0;
        for content in &self.contents[(row - self.min_y) as usize] {
            if let Content::Empty = content {
                count += 1;
            }
        }
        count
    }
}

pub fn num_empty_positions(s: &str, row: i64) -> usize {
    let report: Vec<ReportEntry> = s
        .trim()
        .lines()
        .map(|s| s.parse().expect(&format!("failed to parse entry: {}", s)))
        .collect();
    Map::from_report(report).num_empty_positions(row)
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
