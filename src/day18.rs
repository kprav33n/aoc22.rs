use std::collections::HashSet;
use std::num::ParseIntError;
use std::ops::Sub;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, PartialOrd)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Point {
    fn abs(&self) -> Self {
        Point {
            x: self.x.abs(),
            y: self.y.abs(),
            z: self.z.abs(),
        }
    }

    fn is_adjacent(&self, other: &Point) -> bool {
        let abs = (*self - *other).abs();
        abs.x + abs.y + abs.z == 1
    }
}

#[derive(Debug)]
struct ParsePointError;

impl From<ParseIntError> for ParsePointError {
    fn from(_: ParseIntError) -> Self {
        Self
    }
}

impl FromStr for Point {
    type Err = ParsePointError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens: Vec<&str> = s.split(',').collect();
        if tokens.len() != 3 {
            return Err(ParsePointError);
        }
        Ok(Point {
            x: tokens[0].parse()?,
            y: tokens[1].parse()?,
            z: tokens[2].parse()?,
        })
    }
}

pub fn surface_area(s: &str) -> usize {
    let points: Vec<Point> = s
        .trim()
        .split('\n')
        .map(|s| {
            s.parse()
                .unwrap_or_else(|_| panic!("failed to parse point: {}", s))
        })
        .collect();
    let mut adjacent_pairs: HashSet<(Point, Point)> = HashSet::new();
    for i in 0..points.len() {
        for j in 0..points.len() {
            if i == j {
                continue;
            }
            if points[i].is_adjacent(&points[j]) {
                adjacent_pairs.insert(if points[i] < points[j] {
                    (points[i], points[j])
                } else {
                    (points[j], points[i])
                });
            }
        }
    }
    points.len() * 6 - adjacent_pairs.len() * 2
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5";

    #[test]
    fn test_surface_area() {
        assert_eq!(surface_area(INPUT), 64);
    }
}
