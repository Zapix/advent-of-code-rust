use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};
use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::os::unix::raw::uid_t;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;
use crate::Cell::Antinode;

const DAY: &str = "08"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
"; // TODO: Add the test input


#[derive(Copy, Clone, Hash, Eq, PartialEq)]
enum Cell {
    Antenna(char),
    FreeCell,
    Antinode,
}

impl Display for Cell {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let char = match self {
            Cell::FreeCell => '.',
            Cell::Antinode => '#',
            Cell::Antenna(ch) => *ch,
        };
        write!(f, "{}", char)
    }
}

trait SafeGetter<T> {
    fn try_get(&self, x: T, y: T) -> Result<Cell>;
}

struct Grid {
    grid: Vec<Vec<Cell>>
}

impl Grid {
    pub fn from<R: BufRead>(reader: R) -> Self {
        let grid = reader
            .lines()
            .flatten()
            .map(
                |x| x
                    .chars()
                    .map(|x| match x {
                        '.' => Cell::FreeCell,
                        '#' => Cell::Antinode,
                        ch => Cell::Antenna(ch),
                    })
                    .collect::<Vec<_>>()
            )
            .collect::<Vec<_>>();
        Self {
            grid
        }
    }

    fn set_type(&mut self, i: usize, j: usize, cell: Cell) {
        self.grid[i][j] = cell;
    }

    fn rows(&self) -> usize {
        self.grid.len()
    }

    fn columns(&self) -> usize {
        match self.grid.get(0) {
            None => 0,
            Some(x) => x.len()
        }
    }
}


impl SafeGetter<usize> for Grid {
    fn try_get(&self, x: usize, y: usize) -> Result<Cell> {
        let cell = self.grid.get(x).context("no such row")?.get(y).context("no such column")?;
        Ok(*cell)
    }
}

impl SafeGetter<i32> for Grid {
    fn try_get(&self, x: i32, y: i32) -> Result<Cell> {
        let x = usize::try_from(x).context("no such row")?;
        let y = usize::try_from(y).context("no such column")?;
        let cell = self.grid.get(x).context("no such row")?.get(y).context("no such column")?;
        Ok(*cell)
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let lines = self
            .grid
            .iter()
            .map(
                |x|
                    x
                        .iter()
                        .map(|c| c.to_string())
                        .collect::<Vec<_>>()
                        .join("")
            )
            .collect::<Vec<_>>()
            .join("\r\n");

        writeln!(f, "{}", lines)
    }
}

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
struct Point(i32, i32);


impl Point {
    fn from(x: usize, y: usize) -> Self {
        Point(x as i32, y as i32)
    }

    fn from_float(x: f64, y: f64) -> Self {
        Point(
            x as i32,
            y as i32,
        )
    }

    fn x<T: From<i32>>(&self) -> T {
        T::from(self.0)
    }

    fn y<T: From<i32>>(&self) -> T {
        T::from(self.1)
    }

    fn distance(&self, other: &Point) -> f64 {
        ((self.x::<f64>() - other.x::<f64>()).powi(2) + (self.y::<f64>() - other.y::<f64>()).powi(2)).sqrt()
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{:})", &self.0, &self.1)
    }
}

struct Line {
    a: Option<f64>,
    b: f64,
}

impl Line {
    fn from(p1: &Point, p2: &Point) -> Self {
        let a = if p2.x::<i32>() != p1.x::<i32>() {
            Some((p2.y::<f64>() - p1.y::<f64>()) / (p2.x::<f64>() - p1.x::<f64>()))
        } else {
            None
        };
        let b = match a {
            None => p1.x(),
            Some(a) => p1.y::<f64>() - a * p1.x::<f64>()
        };
        Self { a, b }
    }

    fn tan_a(&self) -> f64 {
        self.a.unwrap_or(f64::NAN)
    }

    fn sin_a(&self) -> f64 {
        (self.tan_a().powi(2) / (self.tan_a().powi(2) + 1f64)).sqrt()
    }

    fn cos_a(&self) -> f64 {
        (1f64 - self.sin_a().powi(2)).sqrt()
    }

    fn line_from(&self, p1: &Point, distance: f64) -> Point {
        let (x, y) = match self.a {
            None => {
                (p1.x::<f64>(), p1.y::<f64>() + distance)
            },
            Some(a) => {
                let x = p1.x::<f64>() + distance * self.cos_a();
                let y = a * x + self.b;
                (x.round(), y.round())
            }
        };
        Point::from_float(x, y)
    }

    fn point_for_x(&self, x: i32) -> Result<Point> {
        let y = self.a.context("vertical line")? * (x as f64) + self.b;
        Ok(Point::from_float(x as f64, y))
    }
}

impl Display for Line {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.a {
            None => write!(f, "y = {:.4}", self.b),
            Some(a) => write!(f, "y = {:.4} * x + {:.4}", a, self.b)
        }
    }
}


type AntennasGroups = HashMap<Cell, HashSet<Point>>;
fn get_antennas_groups(grid: &Grid) -> AntennasGroups {
    let mut antennas: HashMap<Cell, HashSet<Point>> = HashMap::new();
    for i in 0..grid.rows() {
        for j in 0..grid.columns() {
            match grid.try_get(i, j).unwrap() {
                Cell::FreeCell => {},
                Cell::Antinode => {
                    println!("Antinode at {}, {}", i, j);
                },
                cell => {
                    println!("Antenna: at {}, {}", i, j);
                    let points = antennas.entry(cell).or_insert(HashSet::new());
                    points.insert(Point::from(i, j));
                },
            }
        }
    }

    antennas
}

fn get_possible_nodes(point: &Point, line: &Line, distance: f64, grid: &Grid) -> Vec<Point> {
    let mut result = vec![];
    result.push(*point);
    let mut i = 1;
    let mut next_point = line.line_from(point, (i as f64) * distance);
    while grid.try_get(next_point.0, next_point.1).is_ok() {
        result.push(next_point);
        i += 1;
        next_point = line.line_from(point, (i as f64) * distance);
    }

    let mut i = 1;
    let mut next_point = line.line_from(point, -1f64 * (i as f64) * distance);
    while grid.try_get(next_point.0, next_point.1).is_ok() {
        result.push(next_point);
        i += 1;
        next_point = line.line_from(point, -1f64 * (i as f64) * distance);
    }

    result
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        // TODO: Solve Part 1 of the puzzle
        let mut grid = Grid::from(reader);
        println!("Grid rows = {}, columns = {}", grid.rows(), grid.columns());
        let antennas_groups = get_antennas_groups(&grid);
        let mut antinodes = HashSet::new();
        for (cell, points_set) in antennas_groups {
            println!("=== Antenna: {} ===", cell);
            let points = points_set.iter().collect::<Vec<_>>();
            for i in 0..points.len() {
                for j in (i+1)..points.len() {
                    let distance = points[i].distance(points[j]);
                    let line = Line::from(points[i], points[j]);
                    println!("===");
                    println!("p1 = {}, p2 = {}", points[i], points[j]);
                    println!("distance = {:.4}", distance);
                    println!("line = {}", line);
                    let possible_nodes = [
                        line.line_from(points[i], distance),
                        line.line_from(points[i], -distance),
                        line.line_from(points[j], distance),
                        line.line_from(points[j], -distance),
                    ];
                    println!("From {}: {}, {}", points[i], possible_nodes[0], possible_nodes[1]);
                    println!("From {}: {}, {}", points[j], possible_nodes[2], possible_nodes[3]);
                    for point in possible_nodes {
                        if !points_set.contains(&point) && grid.try_get(
                            point.x::<i32>(),
                            point.y::<i32>()
                        ).is_ok() {
                            println!("Point {} is antinode", point);
                            antinodes.insert(point);
                        }
                    }
                    println!("---");
                }
            }
        }
        println!("Antinodes:");
        for node in antinodes.iter() {
            grid.set_type(node.0 as usize, node.1 as usize, Cell::Antinode);
            println!("{}", node);
        }
        println!("{}", grid);
        Ok(antinodes.iter().count())
    }

    // TODO: Set the expected answer for the test input
    // assert_eq!(14, part1(BufReader::new(TEST.as_bytes()))?);

    // let input_file = BufReader::new(File::open(INPUT_FILE)?);
    // let result = time_snippet!(part1(input_file)?);
    // println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut grid = Grid::from(reader);
        println!("Grid rows = {}, columns = {}", grid.rows(), grid.columns());
        let antennas_groups = get_antennas_groups(&grid);
        let mut antinodes = HashSet::new();
        for (cell, points_set) in antennas_groups {
            println!("=== Antenna: {} ===", cell);
            let points = points_set.iter().collect::<Vec<_>>();
            for i in 0..points.len() {
                for j in (i+1)..points.len() {
                    let distance = points[i].distance(points[j]);
                    let line = Line::from(points[i], points[j]);
                    println!("===");
                    println!("p1 = {}, p2 = {}", points[i], points[j]);
                    println!("distance = {:.4}", distance);
                    println!("line = {}", line);
                    let possible_nodes = get_possible_nodes(points[i], &line, distance, &grid);

                    for point in possible_nodes {
                        antinodes.insert(point);
                    }
                    println!("---");
                }
            }
        }
        println!("Antinodes:");
        for node in antinodes.iter() {
            if grid.try_get(node.0, node.1)? == Cell::FreeCell {
                grid.set_type(node.0 as usize, node.1 as usize, Cell::Antinode);
            }
            println!("{}", node);
        }
        println!("{}", grid);
        Ok(antinodes.iter().count())
    }

    assert_eq!(34, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
