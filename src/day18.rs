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

    fn neighbors(&self) -> [Point; 6] {
        [
            Point {
                x: self.x - 1,
                y: self.y,
                z: self.z,
            },
            Point {
                x: self.x + 1,
                y: self.y,
                z: self.z,
            },
            Point {
                x: self.x,
                y: self.y - 1,
                z: self.z,
            },
            Point {
                x: self.x,
                y: self.y + 1,
                z: self.z,
            },
            Point {
                x: self.x,
                y: self.y,
                z: self.z - 1,
            },
            Point {
                x: self.x,
                y: self.y,
                z: self.z + 1,
            },
        ]
    }

    fn is_within(&self, bound: i64) -> bool {
        [self.x, self.y, self.z]
            .iter()
            .all(|&d| -1 <= d && d <= bound)
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

// FIXME: The result is off by one for example and way off for the actual input.
// pub fn external_surface_area(s: &str) -> usize {
//     let mut points: Vec<Point> = s
//         .trim()
//         .split('\n')
//         .map(|s| {
//             s.parse()
//                 .unwrap_or_else(|_| panic!("failed to parse point: {}", s))
//         })
//         .collect();
//     let (min_x, max_x, min_y, max_y, min_z, max_z) = points.iter().fold(
//         (i64::MAX, i64::MIN, i64::MAX, i64::MIN, i64::MAX, i64::MIN),
//         |(nx, xx, ny, xy, nz, xz), p| {
//             (
//                 p.x.min(nx),
//                 p.x.max(xx),
//                 p.y.min(ny),
//                 p.y.max(xy),
//                 p.z.min(nz),
//                 p.z.max(xz),
//             )
//         },
//     );
//     println!("bounds: {:?}", (min_x, max_x, min_y, max_y, min_z, max_z));

//     let mut adjacent_pairs: HashMap<Point, HashMap<Point, bool>> = HashMap::new();
//     for i in 0..points.len() {
//         for j in 0..points.len() {
//             if i == j || !points[i].is_adjacent(&points[j]) {
//                 continue;
//             }
//             adjacent_pairs
//                 .entry(points[i])
//                 .or_insert_with(|| HashMap::new())
//                 .insert(points[j], true);
//             adjacent_pairs
//                 .entry(points[j])
//                 .or_insert_with(|| HashMap::new())
//                 .insert(points[i], true);
//         }
//     }

//     points.retain(|p| {
//         p.x == min_x || p.x == max_x || p.y == min_y || p.y == max_y || p.z == min_z || p.z == max_z
//     });

//     points
//         .iter()
//         .map(|p| match adjacent_pairs.get(p) {
//             None => return 6,
//             Some(m) => {
//                 println!("{:?}, {:?}", p, m);
//                 return 6 - m.len();
//             }
//         })
//         .sum()
// }

pub fn external_surface_area(s: &str) -> usize {
    let points: HashSet<Point> = s
        .trim()
        .lines()
        .map(|s| {
            s.parse()
                .unwrap_or_else(|_| panic!("failed to parse point: {}", s))
        })
        .collect();
    let bound = points
        .iter()
        .fold(i64::MIN, |max, p| p.x.max(p.y.max(p.z.max(max))))
        + 1;
    // DFS idea, courtesy: u/SuperSmurfen
    let mut discovered: HashSet<Point> = HashSet::new();
    let mut stack = vec![Point { x: 0, y: 0, z: 0 }];
    while !stack.is_empty() {
        for point in stack.pop().unwrap().neighbors() {
            if !points.contains(&point) && !discovered.contains(&point) && point.is_within(bound) {
                discovered.insert(point);
                stack.push(point);
            }
        }
    }
    points
        .iter()
        .flat_map(|&p| p.neighbors())
        .filter(|n| discovered.contains(n))
        .count()
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

    #[test]
    fn test_external_surface_area() {
        assert_eq!(external_surface_area(INPUT), 58);
    }
}
