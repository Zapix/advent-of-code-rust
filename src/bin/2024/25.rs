use std::collections::VecDeque;
use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;

const DAY: &str = "25"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####
"; // TODO: Add the test input

type Key = Vec<Vec<bool>>;
type Lock = Vec<Vec<bool>>;

fn has_overlap(lock: &Lock, key: &Key) -> bool{
    lock
        .iter()
        .zip(key.iter())
        .any(|(x, y)| {
            x
                .iter()
                .zip(y.iter())
                .any(|(x, y)| (x & y))

        })
}

fn read_window(raw: &Vec<String>) -> Result<Vec<Vec<bool>>>{
    raw
        .iter()
        .map(|x| {
            x.chars().map(|x| match x {
                '#' => Ok(true),
                '.' => Ok(false),
                ch => Err(anyhow!(format!("Unexpected symbol '{}'", ch)))
            }).collect::<Result<Vec<_>>>()
        })
        .collect::<Result<Vec<_>>>()
}

fn is_lock(data: &Vec<Vec<bool>>) -> bool {
    data[0].iter().all(|x| *x)
}

fn read_data<R: BufRead>(reader: R)  -> Result<(Vec<Lock>, Vec<Key>)> {
    let mut window = vec![];
    let mut keys = vec![];
    let mut locks = vec![];
    for line in reader.lines().flatten() {
        if !line.is_empty() {
            window.push(line);
            continue;
        }
        let data = read_window(&window)?;
        if is_lock(&data) {
            locks.push(data);
        } else {
            keys.push(data);
        }
        window.clear();
    }
    let data = read_window(&window)?;
    if is_lock(&data) {
        locks.push(data);
    } else {
        keys.push(data);
    }

    Ok((keys, locks))
}

fn draw_key_lock(lock: &Lock, key: &Key) -> Result<()> {
    let mut t = term::stdout().context("No terminal")?;
    for i in 0..lock.len() {
        for j in 0..lock[0].len() {
            match (lock[i][j], key[i][j]) {
                (true, false) => {
                    t.fg(term::color::YELLOW)?;
                    write!(t, "#")?;
                    t.fg(term::color::WHITE)?;
                },
                (false, true) => {
                    t.fg(term::color::CYAN)?;
                    write!(t, "#")?;
                    t.fg(term::color::WHITE)?;
                },
                (true, true) => {
                    t.fg(term::color::RED)?;
                    write!(t, "#")?;
                    t.fg(term::color::WHITE)?;
                }
                (false, false) => {
                    write!(t, ".")?;
                }
            }
        }
        writeln!(t,"")?
    }
    Ok(())
}
fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        // TODO: Solve Part 1 of the puzzle
        let (locks, keys) = read_data(reader)?;

        let answer = locks.iter()
            .flat_map(|x| keys.iter().map(move |y| (x, y)))
            .map(|(x, y)| {
                draw_key_lock(x, y)?;
                Ok(!has_overlap(x, y))
            })
            .collect::<Result<Vec<_>>>()?
            .iter()
            .filter(|&x| *x)
            .count();
        Ok(answer)
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(3, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    // println!("\n=== Part 2 ===");
    //
    // fn part2<R: BufRead>(reader: R) -> Result<usize> {
    //     Ok(0)
    // }
    //
    // assert_eq!(0, part2(BufReader::new(TEST.as_bytes()))?);
    //
    // let input_file = BufReader::new(File::open(INPUT_FILE)?);
    // let result = time_snippet!(part2(input_file)?);
    // println!("Result = {}", result);
    //endregion

    Ok(())
}
