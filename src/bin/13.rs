use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ptr::read;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::{Itertools};
use adv_code_2024::*;
use std::str::FromStr;

const DAY: &str = "13"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279
"; // TODO: Add the test input

type Vector = (usize, usize);
type Point = (usize, usize);

type InputValue = (Vector, Vector, Point);

fn parse_usize_input(s: &String, prefix: &str) -> Result<(usize, usize)> {
    let values = s.chars()
        .skip(prefix.len())
        .join("")
        .split(", ")
        .map(|x| usize::from_str(x.split_at(2).1).context("Can not parse").unwrap())
        .collect::<Vec<_>>();

    Ok((
        *values.get(0).context("no x value")?,
        *values.get(1).context("no y values")?
    ))
}

fn parse_button_input(s: &String) -> Result<Vector> {
    parse_usize_input(s, "Button X: ")
}

fn parse_prize_position(s: &String) -> Result<Point> {
    parse_usize_input(s, "Prize: ")
}

fn parse_input_value(x: &Vec<String>) -> Result<InputValue> {
    Ok((
       parse_button_input(x.get(0).context("No Button A")?)?,
       parse_button_input(x.get(1).context("No Button A")?)?,
       parse_prize_position(x.get(2).context("No prize info")?)?
    ))
}

fn read_file<R: BufRead>(reader:R) -> Result<Vec<Result<InputValue>>> {
    Ok(
        reader
            .lines()
            .flatten()
            .chunks(4)
            .into_iter()
            .map(
                |x| parse_input_value(&x.collect::<Vec<_>>())
            )
            .collect::<Vec<_>>()
    )
}

fn compute_det(a: Point, b: Point) -> i128 {
    (a.0 * b.1) as i128  - (a.1 * b.0) as i128
}

fn find_min_tokens(a: Vector, b: Vector, goal: Point) -> usize {
    let det = compute_det(a, b);
    if det == 0 {
        println!("Handle multiple solutions");
        return 0
    }
    let det_a = compute_det(goal, b);
    let det_b = compute_det(a, goal);
    if det_a % det == 0 && det_b % det == 0 {
        (det_a / det * 3 + det_b / det) as usize
    } else {
        0
    }
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        // TODO: Solve Part 1 of the puzzle
        let mut answer = 0;
        for input in read_file(reader)? {
            let input = input?;
            let min_tokens = find_min_tokens(input.0, input.1, input.2);
            answer += min_tokens;
        }
        Ok(answer)
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(480, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut answer = 0;
        for input in read_file(reader)? {
            let input = input?;
            let goal = input.2;
            let goal = (goal.0 + 10000000000000, goal.1 + 10000000000000);
            let min_tokens = find_min_tokens(input.0, input.1, goal);
            answer += min_tokens;
        }
        Ok(answer)
    }

    // assert_eq!(0, part2(BufReader::new(TEST.as_bytes()))?);
    //
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
