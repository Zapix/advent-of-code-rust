use core::result::Result::Ok;
use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::{BinaryHeap, HashSet, HashMap};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;
use adv_code_2024::maze::{find_start_end_point, print_grid, Point};
use crate::maze::{read_map, Direction, Grid, next_position};

const DAY: &str = "20"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############
"; // TODO: Add the test input

pub fn find_optimal(start_point: Point, end_point: Point, grid: &Grid) -> Result<Vec<Point>> {
    let mut que = BinaryHeap::new();
    que.push(vec![start_point]);
    let mut visited =  HashSet::new();
    while let Some(path) = que.pop() {
        let point = *path.last().unwrap();
        if point == end_point {
            return Ok(path);
        }
        if visited.contains(&point) {
            continue;
        }
        if grid[point.0][point.1] == '#' {
            continue;
        }
        for direction in [Direction::East, Direction::South, Direction::North, Direction::West] {
            let next_point = next_position(&point, &direction)?;
            let mut path = path.clone();
            path.push(next_point);
            que.push(path)
        }
        visited.insert(point);
    }

    Err(anyhow!("Optimal way not found"))
}

#[derive(Hash, Eq, PartialEq, Copy, Clone, Ord, PartialOrd)]
enum Cheat {
    UNUSED,
    USED,
}

fn factorial(value: usize) -> usize {
    match value {
        0 => 1,
        value => (1..=value).product()
    }
}

pub fn count_optimal_with_cheat(optimal_path: &Vec<Point>, threshold: usize, grid: &Grid) -> Result<usize> {
    let mut count = 0;
    let point_cost = optimal_path.iter().enumerate().fold(HashMap::new(), |mut acc, x| {
        acc.insert(*x.1, x.0);
        acc
    });
    for point in optimal_path {
        let x = point_cost[point];
        for direction in [Direction::East, Direction::South, Direction::North, Direction::West] {
            let wall = match next_position(point, &direction) {
                Ok(position ) => position,
                _ => continue
            };
            let next_point = match next_position(&wall, &direction) {
                Ok(position ) => position,
                _ => continue
            };
            let y = match point_cost.get(&next_point) {
                Some(value) if *value > x => *value,
                _ => continue
            };
            if threshold <= y - (x + 2) {
                count += 1
            }
        }
    }
    Ok(count)
}

fn compute_point(point: &Point, dx: i32, dy: i32) -> Result<Point> {
    Ok((
        usize::try_from(point.0 as i32 + dx).context("Can not convert")?,
        usize::try_from(point.1 as i32 + dy).context("Can not convert")?,
    ))
}

pub fn count_optimal_with_cheat_v2(optimal_path: &Vec<Point>, threshold: usize) -> usize {
    let mut count = 0;
    let point_cost = optimal_path.iter().enumerate().fold(HashMap::new(), |mut acc, x| {
        acc.insert(*x.1, x.0);
        acc
    });
    for point in optimal_path {
        let x = point_cost[point];
        for i in (-20i32..=20) {
            for j in (-20i32..=20) {
                if i == 0 && j == 0 {
                    continue
                }
                let extra_steps = i.abs() as usize + j.abs() as usize;
                if extra_steps > 20  {
                    continue;
                }
                let next_point = match compute_point(point, i, j) {
                    Ok(next_point) => next_point,
                    _ => continue,
                };
                let y = match point_cost.get(&next_point) {
                    Some(value) if *value > x + extra_steps => *value,
                    _ => continue
                };
                if y - (x + extra_steps) >= threshold {
                    count += 1
                }
            }
        }
    }
    count
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R, threshold: usize) -> Result<usize> {
        // TODO: Solve Part 1 of the puzzle
        let grid = read_map(reader);
        let (start_point, end_point) = find_start_end_point(&grid);
        print_grid(&grid);
        let optimal_path = find_optimal(start_point, end_point, &grid)?;
        println!("Optimal cost is {:?}", optimal_path);
        count_optimal_with_cheat(&optimal_path, threshold, &grid)
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(5, part1(BufReader::new(TEST.as_bytes()), 20)?);

    // let input_file = BufReader::new(File::open(INPUT_FILE)?);
    // let result = time_snippet!(part1(input_file, 100)?);
    // println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R, threshold: usize) -> Result<usize> {
        let grid = read_map(reader);
        let (start_point, end_point) = find_start_end_point(&grid);
        print_grid(&grid);
        let optimal_path = find_optimal(start_point, end_point, &grid)?;
        println!("Optimal cost is {:?}", optimal_path);
        Ok(count_optimal_with_cheat_v2(&optimal_path, threshold))
    }

    assert_eq!(29, part2(BufReader::new(TEST.as_bytes()), 72)?);
    assert_eq!(41, part2(BufReader::new(TEST.as_bytes()), 70)?);
    assert_eq!(55, part2(BufReader::new(TEST.as_bytes()), 68)?);
    assert_eq!(67, part2(BufReader::new(TEST.as_bytes()), 66)?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file, 100)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
