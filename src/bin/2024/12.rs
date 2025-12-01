use core::result::Result::Ok;
use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;
use std::collections::{HashMap, HashSet, VecDeque};
use crate::Side::VerticalRight;

const DAY: &str = "12"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE
"; // TODO: Add the test input

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
enum Side {
    VerticalLeft,
    VerticalRight,
    HorizontalTop,
    HorizontalBottom,
}

const MOVES: [(Side, (i32, i32)); 4] = [
    (Side::VerticalRight, (0, 1)),
    (Side::HorizontalBottom, (1, 0)),
    (Side::HorizontalTop, (-1, 0)),
    (Side::VerticalLeft, (0, -1))
];

type Grid = Vec<Vec<char>>;

fn read_grid<R: BufRead>(reader: R) -> Grid {
    reader
        .lines()
        .flatten()
        .map(|x| x.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>()
}

fn safe_get(x: i32, y: i32, grid: &Grid) -> Result<char> {
    Ok(*grid.get(x as usize).context("Row not found")?.get(y as usize).context("Col not found")?)
}

fn compute_perimeter(x: usize, y: usize, grid: &Grid) -> usize {
    MOVES
        .iter()
        .map(|(_, item)| {
            match safe_get(x as i32 + item.0, y as i32 + item.1, &grid) {
                Ok(ch) if ch == grid[x][y] => 0,
                _ => 1
            }
        })
        .sum()
}

fn get_sides(x: usize, y: usize, grid: &Grid) -> Vec<Side> {
    MOVES
        .iter()
        .filter_map(|(side, item)| {
            match safe_get(x as i32 + item.0, y as i32 + item.1, &grid) {
                Ok(ch) if ch == grid[x][y] => None,
                _ => Some(*side)
            }
        })
        .collect::<Vec<_>>()
}

fn count_sequences(values: &Vec<usize>) -> usize {
    let mut values = values.clone();
    values.sort();
    let mut counter = 0;
    let mut prev = None;
    for value in values {
        prev = match prev {
            Some(x) if x + 1 == value  => Some(value),
            _ => {
                counter += 1;
                Some(value)
            }
        };
    }
    counter
}

fn count_horizontal(coords: &Vec<(usize, usize)>) -> usize {
    coords
        .iter()
        .fold(
            HashMap::new(),
            |mut acc, x| {
                let entry = acc.entry(x.0).or_insert(vec![]);
                entry.push(x.1);
                acc
            }
        )
        .values()
        .map(|x| count_sequences(x))
        .sum()
}

fn count_vertical(coords: &Vec<(usize, usize)>) -> usize {
    coords
        .iter()
        .fold(
            HashMap::new(),
            |mut acc, x| {
                let entry = acc.entry(x.1).or_insert(vec![]);
                entry.push(x.0);
                acc
            }
        )
        .values()
        .map(|x| count_sequences(x))
        .sum()
}


fn count_side(side: &Side, coords: &Vec<(usize, usize)>) -> usize {
    match side {
        Side::HorizontalTop | Side::HorizontalBottom => count_horizontal(&coords),
        Side::VerticalLeft | VerticalRight => count_vertical(&coords)
    }
}
fn count_all_sides(sides: &HashMap<Side, Vec<(usize, usize)>>) -> usize {
    sides.iter().map(|(side, coords)| count_side(side, coords)).sum()
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        // TODO: Solve Part 1 of the puzzle
        let grid = read_grid(reader);
        let n = grid.len();
        let m  = grid[0].len();
        let mut visited = HashSet::new();
        let mut result = 0;
        for i in 0..n {
            for j in 0..m {
                if visited.contains(&(i,j)) {
                    continue;
                }
                let mut que = VecDeque::new();
                que.push_back((i, j));
                let mut area = 0usize;
                let mut perimeter = 0;
                while let Some((x, y)) = que.pop_front() {
                    if visited.contains(&(x, y)) {
                        continue;
                    }
                    area += 1;
                    perimeter += compute_perimeter(x, y, &grid);
                    for (_, (dx, dy)) in MOVES {
                        let nx = x as i32 + dx;
                        let ny = y as i32 + dy;
                        match safe_get(nx, ny, &grid) {
                            Ok(ch) if ch == grid[i][j] => {
                                que.push_back((nx as usize, ny as usize));
                            },
                            _ => {},
                        }
                    }
                    visited.insert((x, y));
                }
                result += area * perimeter;
            }
        }
        Ok(result)
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(1930, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let grid = read_grid(reader);
        let n = grid.len();
        let m  = grid[0].len();
        let mut visited = HashSet::new();
        let mut result = 0;
        for i in 0..n {
            for j in 0..m {
                if visited.contains(&(i,j)) {
                    continue;
                }
                let mut que = VecDeque::new();
                que.push_back((i, j));
                let mut area = 0usize;
                let mut sides = HashMap::new();
                while let Some((x, y)) = que.pop_front() {
                    if visited.contains(&(x, y)) {
                        continue;
                    }
                    area += 1;
                    for side in get_sides(x, y, &grid) {
                        let entry = sides.entry(side).or_insert(vec![]);
                        entry.push((x, y));
                    }
                    for (_, (dx, dy)) in MOVES {
                        let nx = x as i32 + dx;
                        let ny = y as i32 + dy;
                        match safe_get(nx, ny, &grid) {
                            Ok(ch) if ch == grid[i][j] => {
                                que.push_back((nx as usize, ny as usize));
                            },
                            _ => {},
                        }
                    }
                    visited.insert((x, y));
                }
                result += area * count_all_sides(&sides);
            }
        }
        Ok(result)
    }

    assert_eq!(1206, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
