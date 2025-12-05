use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::collections::BTreeSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

const DAY: &str = "05"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
3-5
10-14
16-20
12-18

1
5
8
11
17
32
"; // TODO: Add the test input

fn get_range(s: &str) -> Result<(usize, usize)> {
    let range = s.split("-").collect::<Vec<_>>();
    Ok((
        range.get(0).unwrap().parse::<usize>()?,
        range.get(1).unwrap().parse::<usize>()?,
    ))
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct Interval {
    begin: usize,
    end: usize,
}

impl Interval {
    fn is_in(self, val: usize) -> bool {
        self.begin <= val && val <= self.end
    }

    fn has_overlap(self, val: Interval) -> bool {
        self.is_in(val.begin) || val.is_in(self.begin)
    }

    fn join(self, val: Interval) -> Result<Interval> {
        if self.has_overlap(val) {
            Ok(Interval {
                begin: self.begin.min(val.begin),
                end: self.end.max(val.end),
            })
        } else {
            Err(Error::msg("Doe not overlaps"))
        }
    }
}

impl FromStr for Interval {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let (begin, end) = get_range(s).map_err(|_| Error::msg("Can not parse"))?;

        std::result::Result::Ok(Interval { begin, end })
    }
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn solution<R: BufRead>(reader: R) -> Result<(usize, usize)> {
        // TODO: Solve Part 1 of the puzzle
        let mut answer = 0;
        let lines = reader.lines().flatten().collect::<Vec<_>>();
        let mut intervals = vec![];
        let mut cnt = 0;
        for line in lines.iter() {
            cnt += 1;
            if line.as_str() == "" {
                break;
            }

            let mut interval = line.parse::<Interval>()?;
            let mut new_intervals = vec![];
            for old in intervals {
                if interval.has_overlap(old) {
                    interval = interval.join(old)?;
                } else {
                    new_intervals.push(old);
                }
            }
            new_intervals.push(interval);
            intervals = new_intervals;
        }

        for line in lines.iter().skip(cnt) {
            let value = line.parse::<usize>()?;
            if intervals.iter().any(|x| x.is_in(value)) {
                answer += 1
            }
        }

        let count = intervals.iter().map(|x| x.end - x.begin + 1).sum();
        Ok((answer, count))
    }

    // TODO: Set the expected answer for the test input
    let (ans1, ans2) = solution(BufReader::new(TEST.as_bytes()))?;
    assert_eq!(3, ans1);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(solution(input_file)?);
    println!("Result = {}", result.0);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    assert_eq!(14, ans2);
    println!("Result = {}", result.1);
    //endregion

    Ok(())
}
