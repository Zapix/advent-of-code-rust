use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "11"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out
"; // TODO: Add the test input

const TEST_2: &str = "\
svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out   
";

const YOU_NODE: &str = "you";
const OUT_NODE: &str = "out";
const SVR_NODE: &str = "svr";
const FFT_NODE: &str = "fft";
const DAC_NODE: &str = "dac";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn build_graph<R: BufRead>(reader: R) -> HashMap<String, Vec<String>> {
        let mut graph = HashMap::new();

        for line in reader.lines().flatten() {
            let splits = line
                .split(':')
                .map(|x| x.trim().to_string())
                .collect::<Vec<_>>();
            let node = splits[0].clone();
            let out_nodes = splits[1]
                .split(' ')
                .map(|x| x.to_string())
                .collect::<Vec<_>>();
            graph.insert(node, out_nodes);
        }
        graph
    }

    fn variants(
        graph: &HashMap<String, Vec<String>>,
        in_node: String,
        out_node: String,
        exclude: Option<HashSet<String>>,
    ) -> usize {
        let answer = 0;
        let mut cache: HashMap<String, usize> = HashMap::new();
        let exclude = exclude.unwrap_or(HashSet::new());
        fn dp(
            in1: String,
            out: String,
            graph: &HashMap<String, Vec<String>>,
            exclude: &HashSet<String>,
            cache: &mut HashMap<String, usize>,
        ) -> usize {
            if in1 == out {
                return 1;
            }
            if exclude.contains(&in1) {
                return 0;
            }
            if let Some(val) = (*cache).get(&in1) {
                *val
            } else {
                let val = graph
                    .get(&in1)
                    .unwrap()
                    .iter()
                    .map(|x| dp(x.clone(), out.clone(), graph, exclude, cache))
                    .sum();
                cache.insert(in1, val);
                val
            }
        }
        dp(in_node, out_node, graph, &exclude, &mut cache)
    }

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        // TODO: Solve Part 1 of the puzzle
        let graph = build_graph(reader);
        let answer = variants(&graph, YOU_NODE.to_string(), OUT_NODE.to_string(), None);
        Ok(answer)
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(5, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let graph = build_graph(reader);

        let svr_fft = variants(
            &graph,
            SVR_NODE.to_string(),
            FFT_NODE.to_string(),
            Some(HashSet::from([DAC_NODE.to_string(), OUT_NODE.to_string()])),
        );
        println!("svr->fft = {}", svr_fft);
        let fft_dac = variants(
            &graph,
            FFT_NODE.to_string(),
            DAC_NODE.to_string(),
            Some(HashSet::from([SVR_NODE.to_string(), OUT_NODE.to_string()])),
        );
        println!("fft->dac = {}", fft_dac);
        let dac_out = variants(
            &graph,
            DAC_NODE.to_string(),
            OUT_NODE.to_string(),
            Some(HashSet::from([SVR_NODE.to_string(), FFT_NODE.to_string()])),
        );
        let p2 = svr_fft * fft_dac * dac_out;
        println!(
            "svr->fft({}) * fft->dac({}) * dac->out({}) = {}",
            svr_fft, fft_dac, dac_out, p2
        );
        let answer = p2;
        Ok(answer)
    }

    assert_eq!(2, part2(BufReader::new(TEST_2.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
