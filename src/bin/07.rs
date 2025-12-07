use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "07"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
"; // TODO: Add the test input

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        // TODO: Solve Part 1 of the puzzle
        let mut answer = 0;
        let mut prev_line = vec![];
        for line in reader.lines().flatten() {
            if prev_line.is_empty() {
                println!("{}", line);
                prev_line = line.chars().collect::<Vec<_>>();
                continue;
            }
            let mut line = line.chars().collect::<Vec<_>>();
            for i in 0..line.len() {
                if prev_line[i] == 'S' {
                    line[i] = '|';
                    continue;
                }
                if line[i] == '^' && prev_line[i] == '|' {
                    answer += 1;
                    line[i - 1] = '|';
                    line[i + 1] = '|';
                }
                if line[i] == '.' && prev_line[i] == '|' {
                    line[i] = '|';
                }
            }
            println!("{}", String::from_iter(line.clone()));
            prev_line = line;
        }
        Ok(answer)
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(21, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut prev_line = vec![];
        for line in reader.lines().flatten() {
            if prev_line.is_empty() {
                prev_line = line
                    .chars()
                    .map(|x| if x == 'S' { 1usize } else { 0usize })
                    .collect::<Vec<_>>();
                continue;
            }
            let mut current_line = vec![0; line.len()];
            for (i, ch) in line.chars().enumerate() {
                if ch == '.' {
                    current_line[i] += prev_line[i];
                } else if ch == '^' {
                    current_line[i - 1] += prev_line[i];
                    current_line[i] = 0;
                    current_line[i + 1] += prev_line[i];
                }
            }
            prev_line = current_line;
        }
        Ok(prev_line.into_iter().sum())
    }
    assert_eq!(40, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
