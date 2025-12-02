use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "02"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124
"; // TODO: Add the test input

fn found_silly_in_range(a: i64, b: i64) -> Vec<i64> {
    let min_base = a.ilog10();
    let half_base = min_base / 2;
    let mut half_value = 10i64.pow(half_base);
    let mut result = vec![];
    loop {
        let value = half_value * (10i64.pow(half_value.ilog10() + 1)) + half_value;
        if a <= value && value <= b {
            result.push(value);
        }
        if value > b {
            break;
        }
        half_value += 1
    }

    result
}

fn found_silly_in_range_2(a: i64, b: i64) -> Vec<i64> {
    let max_base = b.ilog10() + 1;
    let half_base = max_base / 2;
    let mut result = HashSet::new();
    for i in 1..10i64.pow(half_base) {
        let base = i.ilog10() + 1;
        let mut value = i * 10i64.pow(base) + i; // repeat at least 2 times
        while value <= b {
            if a <= value && value <= b {
                result.insert(value);
            }
            value = value * 10i64.pow(base) + i;
        }
    }
    result.into_iter().collect::<Vec<_>>()
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<i64> {
        // TODO: Solve Part 1 of the puzzle
        let answer = reader
            .lines()
            .flatten()
            .map(|x| x.split(',').map(|x| x.to_string()).collect::<Vec<String>>())
            .flatten()
            .map(|x| {
                let data = x
                    .split('-')
                    .map(|x| x.parse::<i64>().unwrap_or(0))
                    .collect::<Vec<_>>();

                (data[0], data[1])
            })
            .map(|x| found_silly_in_range(x.0, x.1))
            .flatten()
            .sum();
        Ok(answer)
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(1227775554, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<i64> {
        let answer = reader
            .lines()
            .flatten()
            .map(|x| x.split(',').map(|x| x.to_string()).collect::<Vec<String>>())
            .flatten()
            .map(|x| {
                let data = x
                    .split('-')
                    .map(|x| x.parse::<i64>().unwrap_or(0))
                    .collect::<Vec<_>>();

                (data[0], data[1])
            })
            .map(|x| found_silly_in_range_2(x.0, x.1))
            .flatten()
            .sum();
        Ok(answer)
    }

    assert_eq!(4174379265, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
