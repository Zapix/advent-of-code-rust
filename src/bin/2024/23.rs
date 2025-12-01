use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::{HashMap, HashSet};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;
use itertools::{Itertools, Permutations};

const DAY: &str = "23"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn
"; // TODO: Add the test input


fn is_all_nodes_connected(computers: &Vec<&String>, graph: &HashMap<String, HashSet<String>>) -> bool {
    let n = computers.len();
    for i in 0..n {
        let computer_a = computers[i];
        for j in i+1..n {
            let computer_b = computers[j];
            if !graph.get(computer_a).unwrap().contains(computer_b) {
                return false
            }
        }
    }
   true
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        // TODO: Solve Part 1 of the puzzle
        let pairs = reader
            .lines()
            .flatten()
            .map(|x| {
                let value = x.split("-").collect::<Vec<_>>();
                (value[0].to_string(), value[1].to_string())
            })
            .collect::<Vec<_>>();
        let mut computers = HashSet::new();
        let mut graph = HashMap::new();
        for (x, y)in pairs {
            computers.insert(x.clone());
            computers.insert(y.clone());
            let entry = graph.entry(x.clone()).or_insert(HashSet::new());
            (*entry).insert(y.clone());
            let entry = graph.entry(y.clone()).or_insert(HashSet::new());
            (*entry).insert(x.clone());
        }

        let mut three_nodes = HashSet::new();
        for comp in computers {
            match graph.get(&comp) {
                Some(connected_computers) => {
                    for connected_pairs in  connected_computers.iter().permutations(2) {
                        if !graph.get(&connected_pairs[0].clone()).unwrap().contains(&connected_pairs[1].clone()) {
                            continue;
                        }
                        let mut node = [
                            comp.clone(),
                            connected_pairs[0].clone(),
                            connected_pairs[1].clone()
                        ];
                        node.sort();
                        three_nodes.insert((node[0].clone(), node[1].clone(), node[2].clone()));
                    }
                },
                _ => {}
            }
        }
        println!("Amount of nodes: {}", three_nodes.len());
        let answer = three_nodes
            .iter()
            .filter(|x| x.0.starts_with("t") || x.1.starts_with("t") || x.2.starts_with("t"))
            .count();
        Ok(answer)
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(7, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<String> {
        let mut t = term::stdout().context("Could not receive terminal")?;
        let pairs = reader
            .lines()
            .flatten()
            .map(|x| {
                let value = x.split("-").collect::<Vec<_>>();
                (value[0].to_string(), value[1].to_string())
            })
            .collect::<Vec<_>>();
        let mut computers = HashSet::new();
        let mut graph = HashMap::new();
        for (x, y)in pairs {
            computers.insert(x.clone());
            computers.insert(y.clone());
            let entry = graph.entry(x.clone()).or_insert(HashSet::new());
            (*entry).insert(y.clone());
            let entry = graph.entry(y.clone()).or_insert(HashSet::new());
            (*entry).insert(x.clone());
        }
        let max_connections = graph.values().map(|x| x.len()).max().context("No connections")?;
        t.fg(term::color::BRIGHT_GREEN)?;
        writeln!(t, "Amount of computers: {}", computers.len())?;
        t.fg(term::color::BRIGHT_WHITE)?;
        writeln!(t, "Max connections: {max_connections}")?;
        for count in (3..max_connections + 1).rev() {
            writeln!(t, "Check interconnection with computers: {count}")?;
            for (comp, computers) in graph.iter() {
                for combination in computers.iter().combinations(count) {
                    let mut result = combination.into_iter().collect::<Vec<_>>();
                    result.push(comp);
                    result.sort();
                    writeln!(t, "Check for combination: {:?}", result)?;
                    if is_all_nodes_connected(&result, &graph) {
                        t.fg(term::color::BRIGHT_GREEN)?;
                        writeln!(t, "Valid combination!")?;
                        t.fg(term::color::BRIGHT_WHITE)?;
                        return Ok(result.iter().join(","))
                    }
                    t.cursor_up()?;
                    t.carriage_return()?;
                    t.delete_line()?;
                }
            }
        }
        Err(anyhow!("could not find network"))
    }

    assert_eq!("co,de,ka,ta", part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
