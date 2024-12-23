use std::collections::{HashMap, VecDeque};
use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::thread::sleep;
use std::str::FromStr;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::{Product, izip};
use adv_code_2024::*;

const DAY: &str = "22"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
1
10
100
2024
"; // TODO: Add the test input

const TEST_2: &str = "\
1
2
3
2024
"; // TODO: Add the test input

#[derive(Copy, Clone)]
struct NumberGenerator {
    value: usize
}

impl NumberGenerator {
    fn new(value: usize) -> Self {
        Self {
            value
        }
    }

    fn current(&self) -> usize {
        self.value
    }
}
const PRUNE_VALUE: usize = 16777216;

impl Iterator for NumberGenerator {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let return_value = self.value;
        let value = self.value << 6;
        self.value = (value ^ self.value) % PRUNE_VALUE;
        let value = self.value >> 5;
        self.value = (value ^ self.value) % PRUNE_VALUE;
        let value = (self.value << 11) % PRUNE_VALUE;
        self.value = (value ^ self.value) % PRUNE_VALUE;
        Some(return_value)
    }
}


fn get_nth_value(initial: usize, step: usize) -> usize {
    let mut gen = NumberGenerator::new(initial);
    let mut gen = gen.skip(step);
    gen.next().unwrap()
}

fn get_diff_generator(value: usize) -> Box<dyn Iterator<Item=isize>> {
    let mut gen = NumberGenerator::new(value);
    let delta = gen
        .into_iter()
        .skip(1)
        .zip(gen.into_iter())
        .map(|(x, y)| (x % 10) as isize - (y % 10) as isize);
    Box::new(delta)
}

fn max_achieve(value: usize) -> HashMap<(isize, isize, isize, isize), usize> {
    let mut gen = NumberGenerator::new(value).skip(1);
    let mut diff = get_diff_generator(value);
    let mut window = VecDeque::new();
    let mut result = HashMap::new();
    for (value, diff) in gen.zip(diff).take(1999) {
        window.push_back(diff);
        while window.len() > 4 {
            window.pop_front();
        }
        if window.len() == 4 {
            let data = window.iter().collect::<Vec<_>>();
            let data = (*data[0], *data[1], *data[2], *data[3]);
            let entry = result.entry(data).or_insert(0);
            if *entry == 0 {
                *entry = value % 10;
            }
        }
    }
    result
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        // TODO: Solve Part 1 of the puzzle
        let answer = reader
            .lines()
            .flatten()
            .map(|x| {
                let value = usize::from_str(x.as_str())
                    .context(format!("Can not convert \"{x}\" to usize"))?;
                Ok(get_nth_value(value, 2000))
            })
            .collect::<Result<Vec<_>>>()?
            .iter()
            .sum();
        Ok(answer)
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(37327623, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        reader
            .lines()
            .flatten()
            .map(|x| {
                let value = usize::from_str(x.as_str())
                    .context(format!("Can not convert \"{x}\" to usize"))?;
                Ok(max_achieve(value))
            })
            .collect::<Result<Vec<_>>>()?
            .iter()
            .fold(HashMap::new(), |mut acc, x| {
                for (key, value) in x {
                    let entry = acc.entry(*key).or_insert(0);
                    *entry += value;
                }
                acc
            })
            .values()
            .map(|x| *x)
            .max().context("Can not find max value")
    }

    assert_eq!(23, part2(BufReader::new(TEST_2.as_bytes()))?);

    println!("Valid answer continue with real example");

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generator_number() {
        let mut gen = NumberGenerator::new(123);
        for value in [
            123,
            15887950,
            16495136,
            527345,
            704524,
            1553684,
            12683156,
            11100544,
            12249484,
            7753432,
            5908254,
        ] {
            assert_eq!(Some(value), gen.next());
        }
    }

    #[test]
    fn test_get_2000() {
        for (value, expected) in [
            (1, 8685429),
            (10, 4700978),
            (100, 15273692),
            (2024, 8667524),
        ] {
            assert_eq!(get_nth_value(value, 2000), expected);
        }
    }

    #[test]
    fn test_diff() {
        let mut gen = get_diff_generator(123);
        for value in [-3, 6, -1, -1, 0, 2, -2, 0, -2] {
            assert_eq!(gen.next(), Some(value));
        }
    }
}