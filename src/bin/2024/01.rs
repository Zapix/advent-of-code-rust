use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;

use std::collections::{HashMap};

const DAY: &str = "01"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
3   4
4   3
2   5
1   3
3   9
3   3
"; // TODO: Add the test input

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut first_list = Vec::new();
        let mut second_list = Vec::new();
        for value in reader.lines().flatten() {
            let values = value.split("   ").collect::<Vec<_>>();
            first_list.push(values[0].parse::<i32>().unwrap_or(0));
            second_list.push(values[1].parse::<i32>().unwrap_or(0));
        }
        first_list.sort();
        second_list.sort();
        let answer = first_list.iter()
            .zip(second_list.iter())
            .map(|x| (*x.0 - *x.1).abs() as usize)
            .sum();
        Ok(answer)
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(11, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut first_list = Vec::new();
        let mut second_list = Vec::new();
        for value in reader.lines().flatten() {
            let values = value.split("   ").collect::<Vec<_>>();
            first_list.push(values[0].parse::<i32>().unwrap_or(0));
            second_list.push(values[1].parse::<i32>().unwrap_or(0));
        }

        let values_counter= second_list
            .iter()
            .fold(HashMap::new(), |mut acc, item| {
                *acc.entry(*item).or_insert(0) += 1;
                acc
            });

        let answer = first_list
            .iter()
            .map(|x| (*x * *values_counter.get(x).unwrap_or(&0)) as usize)
            .sum();

        Ok(answer)
    }

    assert_eq!(31, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
