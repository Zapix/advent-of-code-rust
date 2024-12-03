use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;

const DAY: &str = "03"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
"; // TODO: Add the test input
const TEST_2: &str = "\
xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))
"; // TODO: Add the test input

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn try_mul(s: &str, offset: usize) -> Result<(usize, usize)> {
        let mut first_val = 0usize;
        let mut second_val = 0usize;
        if !s[offset..].starts_with("mul(") {
            return Err(anyhow!("doesn't start"));
        }
        let comma = s[(offset + 4)..] .find(",").ok_or(anyhow!("not found"))?;
        first_val = s[(offset + 4)..(offset + 4 + comma)].parse::<usize>().map_err(|_| anyhow!("can not parse"))?;

        let end = s[(offset + 4 + comma)..] .find(")").ok_or(anyhow!("can find close bracket"))?;
        second_val = s[(offset + 4 + comma + 1)..(offset + 4 + comma + end)].parse::<usize>().map_err(|_| anyhow!("Can not parse"))?;
        Ok((first_val * second_val, 1))
    }

    fn is_do(s: &str, offset: usize) -> bool {
        s[offset..].starts_with("do()")
    }

    fn is_do_not(s: &str, offset: usize) -> bool {
        s[offset..].starts_with("don't()")
    }

    fn handle_string(s: String) -> usize {
        let str = s.as_str();
        let mut i = 0usize;
        let mut result = 0usize;
        while i < s.len() {
            let (value, offset) = try_mul(&str, i).unwrap_or((0, 1));
            result += value;
            i += offset;
        }
        result
    }

    fn handle_string_2(s: String, status: bool) -> (usize, bool) {
        let str = s.as_str();
        let mut i = 0usize;
        let mut result = 0usize;
        let mut multipliable = status;
        while i < s.len() {
            if is_do(&str, i) {
                multipliable = true;
            }
            if is_do_not(&str, i) {
                multipliable = false;
            }
            if multipliable {
                let (value, _) = try_mul(&str, i).unwrap_or((0, 1));
                result += value;
            }
            i += 1;
        }
        (result, multipliable)
    }

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        // TODO: Solve Part 1 of the puzzle
        let answer = reader.lines().flatten().map(|x|handle_string(x)).sum();
        Ok(answer)
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(161, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut answer = 0usize;
        let mut multipliable = true;
        for line in  reader.lines().flatten() {
            let (val, status) = handle_string_2(line, multipliable);
            answer += val;
            multipliable = status;;
        }
        Ok(answer)
    }

    assert_eq!(48, part2(BufReader::new(TEST_2.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
