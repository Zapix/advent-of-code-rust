use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "04"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
"; // TODO: Add the test input

fn get_safe(rolls: &Vec<Vec<char>>, i: i32, j: i32) -> char {
    if i < 0
        || j < 0
        || i as usize >= rolls.len()
        || j as usize >= rolls.get(0).unwrap_or(&vec![]).len()
    {
        '.'
    } else {
        rolls[i as usize][j as usize]
    }
}

fn is_valid(rolls: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
    [
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ]
    .into_iter()
    .filter(|(di, dj)| get_safe(rolls, (i as i32 + di), (j as i32 + dj)) == '@')
    .count()
        < 4
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        // TODO: Solve Part 1 of the puzzle
        let rolls = reader
            .lines()
            .flatten()
            .map(|x| x.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let mut answer = 0;
        for i in 0..rolls.len() {
            for j in 0..rolls[0].len() {
                if rolls[i][j] == '@' && is_valid(&rolls, i, j) {
                    answer += 1;
                }
            }
        }
        Ok(answer)
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(13, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut rolls = reader
            .lines()
            .flatten()
            .map(|x| x.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let mut prev_answer = 1;
        let mut answer = 0;
        while prev_answer != answer {
            let mut current = vec![];
            for i in 0..rolls.len() {
                for j in 0..rolls[0].len() {
                    if rolls[i][j] == '@' && is_valid(&rolls, i, j) {
                        current.push((i, j))
                    }
                }
            }
            prev_answer = answer;
            answer += current.len();
            for (i, j) in current {
                rolls[i][j] = '.';
            }
        }
        Ok(answer)
    }

    assert_eq!(43, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
