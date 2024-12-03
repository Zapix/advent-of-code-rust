use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;

const DAY: &str = "02"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
"; // TODO: Add the test input

enum Prev {
    Single(usize),
    OneOf(usize, usize)
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn to_vec(s: String) -> Vec<usize> {
        s.split(" ").map(|x| x.parse::<usize>().unwrap_or(0)).collect::<Vec<_>>()
    }

    fn is_increasing<'a>(data: impl Iterator<Item = &'a usize>) -> bool {
        let mut prev: Option<usize> = None;
        for value in data {
            match prev  {
                Some(prev) if prev >= *value || (*value as i32 - prev as i32) > 3 => {
                    return false;
                },
                _ => {}
            }
            prev = Some(*value);
        }
        true
    }

    fn is_safe(data: &Vec<usize>) -> bool {
        is_increasing(data.iter()) || is_increasing(data.iter().rev())
    }

    fn is_partially_increasing<'a>(data: impl Iterator<Item = &'a usize>) -> bool {
        let mut first_vec = Vec::new();
        let mut second_vec = Vec::new();
        let mut prev: Option<usize> = None;
        let mut has_one_error = false;

        for value in data {
            if has_one_error {
                first_vec.push(*value);
                second_vec.push(*value);
                continue;
            }

            match prev  {
                Some(prev) if prev >= *value || (*value as i32 - prev as i32) > 3 => {
                    has_one_error = true;
                    second_vec.pop();
                    second_vec.push(*value);
                },
                _ => {
                    first_vec.push(*value);
                    second_vec.push(*value);
                }
            }
            prev = Some(*value);
        }

        is_safe(&first_vec) || is_safe(&second_vec)
    }

    fn is_partially_safe(data: &Vec<usize>) -> bool {
        is_partially_increasing(data.iter()) || is_partially_increasing(data.iter().rev())
    }

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        // TODO: Solve Part 1 of the puzzle
        let answer = reader
            .lines()
            .flatten()
            .map(|x| to_vec(x))
            .filter(|x| is_safe(x))
            .count();
        Ok(answer)
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(2, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let answer = reader
            .lines()
            .flatten()
            .map(|x| to_vec(x))
            .filter(|x| is_partially_safe(x))
            .count();
        Ok(answer)
    }

    assert_eq!(4, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
