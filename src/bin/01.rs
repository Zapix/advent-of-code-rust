use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "01"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

#[derive(Debug, Eq, PartialEq)]
enum Rotation {
    Left(i32),
    Right(i32),
}

impl Rotation {
    fn try_from(value: &str) -> Result<Rotation> {
        let chars = value.chars().collect::<Vec<_>>();
        let direction = chars[0];
        let value = chars
            .iter()
            .skip(1)
            .fold(0, |acc, &ch| acc * 10 + (ch as u8 - '0' as u8) as i32);

        match direction {
            'L' => Ok(Rotation::Left(value)),
            'R' => Ok(Rotation::Right(value)),
            _ => panic!("Unexpected condition"),
        }
    }
}

const TEST: &str = "\
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
"; // TODO: Add the test input

const MOD: i32 = 100;

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let rotations = reader
            .lines()
            .map(|x| Rotation::try_from(x?.as_str()))
            .collect::<Result<Vec<_>>>()?;

        let mut curr = 50i32;
        let mut count = 0usize;

        for rotation in rotations {
            curr = match rotation {
                Rotation::Left(x) => (curr - x + MOD) % MOD,
                Rotation::Right(x) => (curr + x + MOD) % MOD,
            };

            if curr == 0 {
                count += 1;
            }
        }

        Ok(count)
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(3, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let rotations = reader
            .lines()
            .map(|x| Rotation::try_from(x?.as_str()))
            .collect::<Result<Vec<_>>>()?;

        let mut curr = 50i32;
        let mut count = 0usize;
        let mut prev_zero = 0;

        for rotation in rotations {
            curr = match rotation {
                Rotation::Left(x) => (curr - x),
                Rotation::Right(x) => (curr + x),
            };

            if curr < 0 {
                let inc_value = curr / (-1 * MOD) + 1 - prev_zero;
                count += inc_value as usize;
                curr = curr.rem_euclid(MOD);
            } else if curr == 0 {
                count += 1;
            } else if curr >= 100 {
                let inc_value = (curr / MOD).max(0);
                count += inc_value as usize;
                curr = curr.rem_euclid(MOD);
            }

            if curr == 0 {
                prev_zero = 1;
            } else {
                prev_zero = 0;
            }
        }

        Ok(count)
    }

    assert_eq!(6, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
