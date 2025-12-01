use std::collections::{HashMap, HashSet};
use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;

const DAY: &str = "19"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb
"; // TODO: Add the test input

#[derive(Default, Debug)]
struct TrieNode{
    is_end: bool,
    children: HashMap<char, TrieNode>,
}

struct Trie {
    root: TrieNode
}


impl Trie {
    fn new() -> Self {
        let root= TrieNode::default();
        Self {
            root
        }
    }

    fn from(strings: Vec<&str>) -> Self {
        let mut trie = Self::new();
        for string in strings {
            trie.insert(string);
        }
        trie
    }

    fn insert(&mut self, value: &str) {
        let mut node = &mut self.root;
        for char in value.chars() {
            node = node.children.entry(char).or_insert(TrieNode::default());
        }
        node.is_end = true;
    }

    fn search_sequences(&self, string: &Vec<char>, offset: usize) -> Vec<usize> {
        let mut i = 0;
        let mut result = vec![];
        let mut current = Some(&self.root);
        while let Some(node) = current {
            if node.is_end {
                result.push(offset + i);
            }
            if offset + i >= string.len() {
                break;
            }
            current = node.children.get(&string[offset + i]);
            i += 1;
        }
        result
    }
}

fn can_build_with_trie(x: &Vec<char>, trie: &Trie) -> bool {
    let mut stack = vec![];
    stack.push(0usize);
    let mut visited = HashSet::new();
    while let Some(offset) = stack.pop() {
        if visited.contains(&offset) {
            continue;
        }
        if offset == x.len() {
            return true;
        }
        stack.append(&mut trie.search_sequences(x, offset));
        visited.insert(offset);
    }
    false
}

fn count_variants(x: &String, towels: &Vec<&str>) -> usize {
    let mut dp = vec![0usize; x.len()];
    for i in 0..x.len() {
        for towel in towels {
            let x_bytes = x[i..(i + towel.len()).min(x.len())].as_bytes();
            let towel_bytes = towel.as_bytes();
            if x_bytes == towel_bytes {
                dp[i + towel.len() - 1] += match i {
                    0 => 1,
                    _ => dp[i - 1],
                }
            }
        }
    }

    dp[x.len() -1]
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        // TODO: Solve Part 1 of the puzzle
        let lines = reader.lines().flatten().collect::<Vec<_>>();
        let towels = lines[0].split(", ").collect::<Vec<_>>();
        let trie = Trie::from(towels);
        let answer = lines
            .into_iter()
            .skip(2)
            .filter(|x| {
                let x = x.chars().collect::<Vec<_>>();
                can_build_with_trie(&x, &trie)
            })
            .count();

        Ok(answer)
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(6, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let lines = reader.lines().flatten().collect::<Vec<_>>();
        let binding = lines.clone().into_iter().take(1).last().unwrap();
        let towels = binding.split(", ").collect::<Vec<_>>();
        let answer = lines
            .into_iter()
            .skip(2)
            .map(|x| {
                count_variants(&x, &towels)
            })
            .sum();

        Ok(answer)
    }

    assert_eq!(16, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
