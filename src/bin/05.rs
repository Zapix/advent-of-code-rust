use std::cmp::Ordering;
use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;

use std::collections::{HashMap, HashSet};
use std::ptr::read;
use std::str::FromStr;
use itertools::Itertools;

const DAY: &str = "05"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
"; // TODO: Add the test input

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn get_dependency(s: String) -> Result<(usize, usize)> {
        let dependency = s.split('|').map(
            |x| usize::from_str(x).unwrap_or(0)).collect::<Vec<_>>();
        Ok((dependency[0], dependency[1]))
    }

    fn get_sequence(s: String) -> Result<Vec<usize>> {
        Ok(s.split(',').map(|x| usize::from_str(x).unwrap_or(0))
            .collect::<Vec<_>>())
    }

    fn is_valid_sequence(sequence: &Vec<usize>, dependency_map: &HashMap<usize, HashSet<usize>>) -> bool {
        let mut visited = HashSet::new();
        for num in sequence {
            match dependency_map.get(num) {
                None => {},
                Some(dependencies) => {
                    if visited.intersection(dependencies).count() > 0 {
                        return false;
                    }
                }
            }

            visited.insert(*num);
        }
        true
    }

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        // TODO: Solve Part 1 of the puzzle
        let mut answer = 0;
        let mut dependency_map: HashMap<usize, HashSet<usize>> = HashMap::new();
        let mut lines = reader.lines().flatten();
        let mut read_sequences = false;
        for line in lines {
            if line.is_empty() {
                read_sequences = true;
                continue;
            }
            if !read_sequences {
                let (before_num, after_num) = get_dependency(line)?;
                let dependencies = dependency_map.entry(before_num).or_insert(HashSet::new());
                dependencies.insert(after_num);
            } else {
                let sequence = get_sequence(line)?;
                if is_valid_sequence(&sequence, &dependency_map) {
                    answer += sequence[sequence.len() / 2];
                }
            }
        }

        Ok(answer)
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(143, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut answer = 0;
        let mut dependency_map: HashMap<usize, HashSet<usize>> = HashMap::new();
        let mut lines = reader.lines().flatten();
        let mut read_sequences = false;
        for line in lines {
            if line.is_empty() {
                read_sequences = true;
                continue;
            }
            if !read_sequences {
                let (before_num, after_num) = get_dependency(line)?;
                let dependencies = dependency_map.entry(before_num).or_insert(HashSet::new());
                dependencies.insert(after_num);
            } else {
                let mut sequence = get_sequence(line)?;
                if !is_valid_sequence(&sequence, &dependency_map) {
                    sequence.sort_by(|a, b| {
                        match dependency_map.get(a) {
                            None => {},
                            Some(a_deps) => {
                                if a_deps.contains(b) {
                                    return Ordering::Less;
                                }
                            }
                        }
                        match dependency_map.get(b) {
                            None => {},
                            Some(b_deps ) => {
                                if b_deps.iter().contains(a) {
                                    return Ordering::Greater;
                                }
                            }
                        };
                        Ordering::Equal
                    });
                    answer += sequence[sequence.len() / 2];
                }
            }
        }
        Ok(answer)
    }

    assert_eq!(123, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
