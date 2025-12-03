use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "03"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
987654321111111
811111111111119
234234234234278
818181911112111
"; // TODO: Add the test input

fn get_joltage(s: String, size: usize) -> usize {
    let digits = s
        .chars()
        .map(|x| (x as u8 - '0' as u8) as usize)
        .collect::<Vec<_>>();

    let mut result = vec![];

    for i in 0..digits.len() {
        while let Some(&val) = result.last() {
            if val < digits[i] && digits.len() - i >= size - (result.len() - 1) {
                result.pop();
            } else {
                break;
            }
        }
        if result.len() < size {
            result.push(digits[i]);
        }
    }
    result.into_iter().fold(0, |acc, item| acc * 10 + item)
}

fn get_joltage_1(s: String) -> usize {
    get_joltage(s, 2)
}

fn get_joltage_2(s: String) -> usize {
    get_joltage(s, 12)
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        // TODO: Solve Part 1 of the puzzle
        let answer = reader.lines().flatten().map(get_joltage_1).sum();
        Ok(answer)
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(357, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let answer = reader.lines().flatten().map(get_joltage_2).sum();
        Ok(answer)
    }

    assert_eq!(3121910778619, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    // endregion

    Ok(())
}
