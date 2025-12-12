use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

const DAY: &str = "09"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
"; // TODO: Add the test input

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    pub fn is_in_area(
        &self,
        points: &HashSet<&Point>,
        x_points: &HashMap<usize, Vec<&Point>>,
        y_points: &HashMap<usize, Vec<&Point>>,
        memo: &mut HashMap<Point, bool>,
    ) -> bool {
        if let Some(&value) = memo.get(self) {
            return value;
        }
        if points.contains(self) {
            memo.insert(*self, true);
            return true;
        }
        if let Some(interval) = x_points.get(&self.x) {
            if self.is_in_interval(interval[0], interval[1]) {
                memo.insert(*self, true);
                return true;
            }
        }
        if let Some(interval) = y_points.get(&self.y) {
            if self.is_in_interval(interval[0], interval[1]) {
                memo.insert(*self, true);
                return true;
            }
        }

        if x_points
            .iter()
            .filter(|(&x, _)| x < self.x)
            .all(|(&x, interval)| {
                let p = Point { x, y: self.y };
                !p.is_in_interval(interval[0], interval[1])
            })
        {
            memo.insert(*self, false);
            return false;
        }

        if x_points
            .iter()
            .filter(|(&x, _)| x > self.x)
            .all(|(&x, interval)| {
                let p = Point { x, y: self.y };
                !p.is_in_interval(interval[0], interval[1])
            })
        {
            memo.insert(*self, false);
            return false;
        }

        if y_points
            .iter()
            .filter(|(&y, _)| y < self.y)
            .all(|(&y, interval)| {
                let p = Point { x: self.x, y };
                !p.is_in_interval(interval[0], interval[1])
            })
        {
            memo.insert(*self, false);
            return false;
        }

        if y_points
            .iter()
            .filter(|(&y, _)| y > self.y)
            .all(|(&y, interval)| {
                let p = Point { x: self.x, y };
                !p.is_in_interval(interval[0], interval[1])
            })
        {
            memo.insert(*self, false);
            return false;
        }

        memo.insert(*self, true);
        true
    }

    pub fn is_in_interval(&self, a: &Point, b: &Point) -> bool {
        if a.x == b.x {
            let min_y = a.y.min(b.y);
            let max_y = a.y.max(b.y);
            min_y <= self.y && self.y <= max_y
        } else if a.y == b.y {
            let min_x = a.x.min(b.x);
            let max_x = a.x.max(b.x);
            min_x <= self.x && self.x <= max_x
        } else {
            false
        }
    }
}

impl FromStr for Point {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let point = s
            .split(',')
            .map(|x| x.parse::<usize>().map_err(|x| Error::msg(x.to_string())))
            .collect::<Result<Vec<usize>>>()?;

        Ok(Point {
            x: point
                .get(0)
                .map_or(Err(Error::msg("no element")), |x| Ok(*x))?,
            y: point
                .get(1)
                .map_or(Err(Error::msg("no element")), |x| Ok(*x))?,
        })
    }
}

fn square(a: Point, b: Point) -> usize {
    (a.x.abs_diff(b.x) + 1) * (a.y.abs_diff(b.y) + 1)
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        // TODO: Solve Part 1 of the puzzle
        let points = reader
            .lines()
            .flatten()
            .map(|x| x.parse::<Point>())
            .collect::<Result<Vec<_>>>()?;
        let mut answer = 0;
        for i in 0..points.len() {
            for j in (i + 1)..points.len() {
                answer = answer.max(square(points[i], points[j]));
            }
        }

        Ok(answer as usize)
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(50, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let points = reader
            .lines()
            .flatten()
            .map(|x| x.parse::<Point>())
            .collect::<Result<Vec<_>>>()?;

        let points_set: HashSet<&Point> = HashSet::from_iter(points.iter());
        let mut x_points = HashMap::new();
        let mut y_points = HashMap::new();

        for point in points.iter() {
            x_points.entry(point.x).or_insert(vec![]).push(point);
            y_points.entry(point.y).or_insert(vec![]).push(point);
        }

        let mut answer = 0;
        let mut memo = HashMap::new();
        for i in 0..points.len() {
            for j in (i + 1)..points.len() {
                let mut able_to_build = true;
                let min_x = points[i].x.min(points[j].x);
                let max_x = points[i].x.max(points[j].x);
                let min_y = points[i].y.min(points[j].y);
                let max_y = points[i].y.max(points[j].y);

                let a = Point { x: min_x, y: min_y };
                if !a.is_in_area(&points_set, &x_points, &y_points, &mut memo) {
                    continue;
                }
                let b = Point { x: min_x, y: max_y };
                if !b.is_in_area(&points_set, &x_points, &y_points, &mut memo) {
                    continue;
                }
                let c = Point { x: max_x, y: min_y };
                if !c.is_in_area(&points_set, &x_points, &y_points, &mut memo) {
                    continue;
                }
                let d = Point { x: max_x, y: max_y };
                if !d.is_in_area(&points_set, &x_points, &y_points, &mut memo) {
                    continue;
                }

                for (x, interval) in x_points.iter() {
                    if *x <= min_x || *x >= max_x {
                        continue;
                    }
                    let p = Point { x: *x, y: min_y };
                    if p != *interval[0]
                        && p != *interval[1]
                        && p.is_in_interval(interval[0], interval[1])
                    {
                        able_to_build = false;
                        break;
                    }
                    let p = Point { x: *x, y: max_y };
                    if p != *interval[0]
                        && p != *interval[1]
                        && p.is_in_interval(interval[0], interval[1])
                    {
                        able_to_build = false;
                        break;
                    }
                }

                if !able_to_build {
                    continue;
                }

                for (y, interval) in y_points.iter() {
                    if *y <= min_y || *y >= max_y {
                        continue;
                    }
                    let p = Point { x: min_x, y: *y };
                    if p != *interval[0]
                        && p != *interval[1]
                        && p.is_in_interval(interval[0], interval[1])
                    {
                        able_to_build = false;
                        break;
                    }
                    let p = Point { x: max_x, y: *y };
                    if p != *interval[0]
                        && p != *interval[1]
                        && p.is_in_interval(interval[0], interval[1])
                    {
                        able_to_build = false;
                        break;
                    }
                }

                if !able_to_build {
                    continue;
                }

                answer = answer.max(square(points[i], points[j]));
            }
        }

        Ok(answer as usize)
    }

    assert_eq!(24, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
