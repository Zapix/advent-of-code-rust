use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::collections::HashMap;
use std::fmt::Write;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter};
use std::str::FromStr;

const DAY: &str = "06"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  
"; // TODO: Add the test input

enum Cell {
    Number(usize),
    Add,
    Multiply,
}

impl FromStr for Cell {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let parsed = s.parse::<usize>();
        if parsed.is_ok() {
            let value = parsed.unwrap();
            Ok(Cell::Number(value))
        } else {
            match s {
                "*" => Ok(Cell::Multiply),
                "+" => Ok(Cell::Add),
                _ => Err(Error::msg(format!("Unrecognized cell \"{}\"", s))),
            }
        }
    }
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        // TODO: Solve Part 1 of the puzzle

        let mut expressions: HashMap<usize, Vec<usize>> = HashMap::new();
        let mut answer = 0;
        for line in reader.lines().flatten() {
            let mut idx = 0;
            for item in line.split(' ') {
                if item.trim() == "" {
                    continue;
                }
                match item.parse::<Cell>()? {
                    Cell::Number(value) => {
                        let entry = expressions.entry(idx).or_insert(vec![]);
                        (*entry).push(value);
                    }
                    Cell::Add => {
                        answer += expressions
                            .get(&idx)
                            .unwrap_or(&vec![])
                            .iter()
                            .fold(0, |acc, &value| acc + value)
                    }
                    Cell::Multiply => {
                        answer += expressions
                            .get(&idx)
                            .unwrap_or(&vec![])
                            .iter()
                            .fold(1, |acc, &value| acc * value)
                    }
                }
                idx += 1;
            }
        }

        Ok(answer)
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(4277556, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");
    //
    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let matrix = reader
            .lines()
            .flatten()
            .map(|x| x.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let n = matrix.iter().map(|x| x.len()).max().unwrap_or(0);
        let m = matrix.len();

        let mut input = String::new();
        for i in 0..n {
            for j in 0..m {
                write!(&mut input, "{}", matrix[j][i])?;
            }
            write!(&mut input, "\r\n")?;
        }
        write!(&mut input, "\r\n")?;
        let mut answer = 0;
        let mut expression = None;
        let mut values = vec![];
        let reader = BufReader::new(input.as_bytes());
        let lines = reader.lines().flatten();
        for line in lines {
            if line.trim() == "" {
                match expression {
                    Some(Cell::Add) => {
                        answer += values.iter().fold(0, |acc, item| acc + item);
                    }
                    Some(Cell::Multiply) => {
                        answer += values.iter().fold(1, |acc, item| acc * item);
                    }
                    _ => return Err(Error::msg("Unexpected expression")),
                }
                values.clear();
                expression = None;
                continue;
            }
            if line.contains("+") {
                expression = Some(Cell::Add);
            }
            if line.contains("*") {
                expression = Some(Cell::Multiply);
            }
            values.push(
                line.trim_matches(|x| x == '+' || x == '*')
                    .trim()
                    .parse::<usize>()?,
            );
        }

        Ok(answer)
    }

    assert_eq!(3263827, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
