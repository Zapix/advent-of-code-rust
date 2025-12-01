use core::result::Result::Ok;
use std::collections::HashSet;
use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;
use std::str::FromStr;
use std::fmt;
use std::fmt::Formatter;

const DAY: &str = "07"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
"; // TODO: Add the test input

#[derive(Debug, Clone, Copy)]
enum Token {
    Number(usize),
    Mul,
    Add,
    Concat,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Token::Number(x) => write!(f, "{}", x),
            Token::Mul => write!(f, "*"),
            Token::Add => write!(f, "+"),
            Token::Concat => write!(f, "||"),
        }
    }
}


fn get_expression_str(expression: &Vec<Token>) -> String {
    expression
        .clone()
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join("")
}

fn compute_expression(expression: Vec<Token>) -> Result<usize> {
    let mut sum_expression = vec![];

    for token in expression {
        match token {
            Token::Number(number) => {
                match sum_expression.last() {
                    None | Some(Token::Add) => {
                        sum_expression.push(Token::Number(number))
                    },
                    Some(Token::Number(_)) => {
                        return Err(anyhow!("Not allowed"))
                    },
                    Some(Token::Concat) => {
                        return Err(anyhow!("Unexpected operator"))
                    },
                    Some(Token::Mul) => {
                        sum_expression.pop();
                        match sum_expression.pop() {
                            Some(Token::Number(value)) => {
                                sum_expression.push(Token::Number(number * value));
                            },
                            _ => {
                                return Err(anyhow!("Can not preform multiply"))
                            }
                        }
                    }
                }
            },
            token => {
                sum_expression.push(token)
            }
        }
    }
    let result = sum_expression
        .iter()
        .map(|x| match *x {
            Token::Number(x) => x,
            _ => 0,
        })
        .sum();
    Ok(result)
}

fn compute_as_dump(expression: Vec<Token>) -> Result<usize> {
    let mut result = match expression[0] {
        Token::Number(val) => val,
        _ => return Err(anyhow!("unexpected operator")),
    };

    for i in (2..expression.len()).step_by(2) {
        match expression[i - 1] {
            Token::Number(_)  => return Err(anyhow!("unexpected number")),
            Token::Add => result += match expression[i] {
                Token::Number(val) => val,
                _ => return Err(anyhow!("unexpected operator")),
            },
            Token::Mul => result *= match expression[i] {
                Token::Number(val) => val,
                _ => return Err(anyhow!("unexpected operator")),
            },
            Token::Concat => {
                let second = match expression[i] {
                    Token::Number(val) => val,
                    _ => return Err(anyhow!("unexpected operator")),
                };
                result = usize::from_str(format!("{}{}", result, second).as_str())?
            },
        }
    }

    Ok(result)
}

fn try_parse_string(s: String) -> Result<(usize, Vec<usize>)> {
    let paths = s.split(": ").collect::<Vec<_>>();
    let value = usize::from_str(paths[0])?;
    let numbers = paths[1].split(' ').map(|x| usize::from_str(x).unwrap()).collect::<Vec<_>>();
    Ok((value, numbers))
}


fn try_build_equation(s: String) -> Result<usize> {
    let (value, numbers) = try_parse_string(s)?;
    let n = numbers.len() - 1;
    for i in 0..2usize.pow(n as u32) {
        let mut expression = vec![];
        for j in 0..n {
           expression.push(Token::Number(numbers[j]));
           if i & (1 << j) > 0 {
               expression.push(Token::Mul);
           } else {
               expression.push(Token::Add);
           }
        }
        expression.push(Token::Number(numbers[n]));
        match compute_as_dump(expression.clone()) {
            Ok(result) if result == value => {
                return Ok(value)
            },
            _ => {},
        }
    }
    Err(anyhow!("can not build equation"))
}

fn try_build_equation_with_concat(s: String) -> Result<usize> {
    let (value, numbers) = try_parse_string(s)?;
    let n = numbers.len() - 1;
    for i in 0..3usize.pow(n as u32) {
        let mut expression = vec![];
        for j in 0..n {
            expression.push(Token::Number(numbers[j]));
            match (i / 3usize.pow(j as u32)) % 3 {
                0 => expression.push(Token::Mul),
                1 => expression.push(Token::Add),
                2 => expression.push(Token::Concat),
                _ => {
                    return Err(anyhow!("unexpected value"));
                },
            }
        }
        expression.push(Token::Number(numbers[n]));
        match compute_as_dump(expression.clone()) {
            Ok(result) if result == value => {
                return Ok(value)
            },
            _ => {},
        }
    }
    Err(anyhow!("can not build equation"))
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
            .map(|x| try_build_equation(x).unwrap_or(0))
            .sum();
        Ok(answer)
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(3749, part1(BufReader::new(TEST.as_bytes()))?);

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
            .map(|x| try_build_equation_with_concat(x).unwrap_or(0))
            .sum();
        Ok(answer)
    }

    assert_eq!(11387, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
