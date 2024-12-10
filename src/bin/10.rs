use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::thread;
use std::sync::Arc;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;
use std::collections::{HashSet, VecDeque};
const DAY: &str = "10"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
"; // TODO: Add the test input

type Grid = Vec<Vec<usize>>;

fn read_grid<R: BufRead>(reader: R) -> Grid {
    reader
        .lines()
        .flatten()
        .map(|x| x
            .chars()
            .map(|x| if x =='.' { 11 } else { x as usize - '0' as usize }).collect::<Vec<_>>()
        )
        .collect::<Vec<_>>()
}

fn get_safe(grid: &Grid, i: usize, j: usize) -> Result<usize> {
    let value = *grid.get(i).context("now row")?.get(j).context("now col")?;
    Ok(value)
}

fn validate_move(grid: &Grid, i: usize, j: usize, di: i32, dj: i32) -> Result<(usize, usize)> {
    let ni = usize::try_from(i as i32 + di)?;
    let nj = usize::try_from(j as i32 + dj)?;

    if grid[i][j] + 1 != get_safe(grid, ni, nj)? {
        return Err(anyhow!("wrong step"))
    }
    Ok((ni, nj))
}

fn compute_slope_1(grid: &Grid, i: usize, j: usize) -> usize {
    if grid[i][j] != 0 {
        return 0
    };
    let mut que = VecDeque::new();
    que.push_back((i, j));
    let mut reached = HashSet::new();
    while let Some((i, j)) = que.pop_front() {
        if grid[i][j] == 9 {
            reached.insert((i, j));
            continue;
        }
        for (di, dj) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            match validate_move(&grid, i, j, di, dj) {
                Result::Ok(value) => {
                    que.push_back(value);
                },
                _ => {},
            }
        }
    }
    reached.iter().count()
}

fn compute_slope_2(grid: &Grid, i: usize, j: usize) -> usize {
    if grid[i][j] != 0 {
        return 0
    };
    let mut que = VecDeque::new();
    que.push_back((i, j));
    let mut answer = 0;
    while let Some((i, j)) = que.pop_front() {
        if grid[i][j] == 9 {
            answer += 1;
            continue;
        }
        for (di, dj) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            match validate_move(&grid, i, j, di, dj) {
                Result::Ok(value) => {
                    que.push_back(value);
                },
                _ => {},
            }
        }
    }
    answer
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        // TODO: Solve Part 1 of the puzzle
        let grid = read_grid(reader);
        let n = grid.len();
        let m = grid[0].len();
        let mut result = 0usize;
        for i in 0..n {
            for j in 0..m  {
                result += compute_slope_1(&grid, i, j);
            }
        }
        Ok(result)
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(36, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let grid = read_grid(reader);
        let n = grid.len();
        let m = grid[0].len();
        let mut result = 0usize;
        for i in 0..n {
            for j in 0..m  {
                result += compute_slope_2(&grid, i, j);
            }
        }
        Ok(result)
    }

    assert_eq!(81, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
