use anyhow::*;
use std::collections::HashMap;
use std::fs::File;
use std::hash::Hash;
use std::io::{BufRead, BufReader};
use std::str::FromStr;
use std::time::Duration;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;

const DAY: &str = "11"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
125 17
"; // TODO: Add the test input

fn read_vector<R: BufRead>(mut reader: R) -> Result<Vec<usize>> {
    let mut str = String::new();
    reader.read_line(&mut str)?;
    let mut result = vec![];
    for s in str.split_whitespace() {
        result.push(usize::from_str(s)?);
    }
    Ok(result)
}

fn compute_next_stones(stone: usize) -> Vec<usize> {
    if stone == 0 {
        return vec![1usize];
    }
    let log_value = stone.ilog10() + 1;
    if log_value % 2 == 0 {
        let dividor = 10usize.pow(log_value / 2) ;
        return vec![stone / dividor, stone % dividor];
    }
    return vec![stone * 2024]
}

fn get_next_stones(stone: usize, cache: &mut HashMap<usize, Vec<usize>>) -> Vec<usize> {
    if !cache.contains_key(&stone) {
        cache.insert(stone, compute_next_stones(stone));
    }
    cache.get(&stone).unwrap().clone()
}

fn compute_from_stone(stone: usize, step: usize, limit: usize, cache: &mut HashMap<(usize, usize), usize>) -> usize {
    if limit == step {
        return 1;
    }
    let diff = limit - step;
    if cache.contains_key(&(stone, diff)) {
        return cache.get(&(stone, diff)).unwrap().clone();
    }
    let mut result = 0usize;
    for next_stone in compute_next_stones(stone) {
        result += compute_from_stone(next_stone, step + 1, limit, cache);
    }
    cache.insert((stone, diff), result);
    result
}


fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn compute_part<R: BufRead>(mut reader: R, count: usize) -> Result<usize> {
        let odd_vector: Vec<usize> = read_vector(reader)?;
        let mut cache: HashMap<(usize, usize), usize> = HashMap::new();

        let mut result = 0usize;
        for stone in odd_vector {
            result += compute_from_stone(stone, 0, count, &mut cache);
        }

        Ok(result)
    }


    fn part1<R: BufRead>(mut reader: R) -> Result<usize> {
        compute_part(reader, 25)
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(55312, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");
    
    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        compute_part(reader, 75)
    }
    
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    // assert_eq!(55312, part2(BufReader::new(TEST.as_bytes()))?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
