use std::collections::{HashMap, HashSet, VecDeque};
use std::env::var;
use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ptr::read;
use std::cmp::Reverse;
use std::fmt::{Display, Formatter};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;

const DAY: &str = "24"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj
"; // TODO: Add the test input

const TEST_2: &'static str = "\
x00: 0
x01: 1
x02: 0
x03: 1
x04: 0
x05: 1
y00: 0
y01: 0
y02: 1
y03: 1
y04: 0
y05: 1

x00 AND y00 -> z05
x01 AND y01 -> z02
x02 AND y02 -> z01
x03 AND y03 -> z03
x04 AND y04 -> z04
x05 AND y05 -> z00
";

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
enum Operations {
    AND,
    XOR,
    OR
}

impl Operations {
    fn try_from(op: &str) -> Result<Self> {
        match op {
            "XOR" => Ok(Operations::XOR),
            "OR" => Ok(Operations::OR),
            "AND" => Ok(Operations::AND),
            op => Err(anyhow!(format!("unexpected operation \"{}\"", op)))
        }
    }

    fn apply(&self, a: usize, b: usize) -> usize {
        match self {
            Operations::XOR => a ^ b,
            Operations::OR => a | b,
            Operations::AND => a & b,
        }
    }

}

impl Display for Operations {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Operations::XOR => write!(f, " ^ "),
            Operations::AND => write!(f, " & "),
            Operations::OR => write!(f, " | "),
        }
    }
}

#[derive(Clone, Eq, PartialEq, Hash)]
struct Command(String, String, Operations, String);

impl Command {
    fn try_parse(raw: &str) -> Result<Self> {
        let data = raw.split(" ").collect::<Vec<_>>();
        let var_a = data[0].to_string();
        let var_b = data[2].to_string();
        let operation = Operations::try_from(data[1])?;
        let result = data[4].to_string();
        if var_a.starts_with("x") {
            Ok(Self(var_a, var_b, operation, result))

        } else {
            Ok(Self(var_b, var_a, operation, result))
        }

    }

    fn has_variable(&self, variable: &String) -> bool {
        (self.0 == variable.clone()) || (self.1 == variable.clone()) || (self.3 == variable.clone())
    }
}

impl Display for Command {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {} -> {}", self.0, self.2, self.1, self.3)
    }
}

fn parse_values(raw: &str) -> Result<(String, usize)> {
    let name = raw[..3].to_string();
    let value = match &raw[5..] {
        "1" => 1usize,
        "0" => 0,
        value => {
            return Err(anyhow!(format!("Unexpected value {}", value)));
        }
    };
    Ok((name, value))
}

fn parse_commands (raw: &str) -> Result<Command>{
    Command::try_parse(raw)
}

fn get_input_data<R: BufRead>(reader: R) -> Result<(HashMap<String, usize>, Vec<Command>)> {
    let mut reader = reader.lines().flatten();
    let mut variables = HashMap::new();
    while let Some(raw_data) = reader.next() {
        if raw_data.is_empty() {
            break;
        }
        let (name, value) = parse_values(raw_data.as_str())?;
        variables.insert(name, value);
    }
    let mut que = Vec::new();

    while let Some(raw_data) = reader.next() {
        let command = parse_commands(raw_data.as_str())?;
        que.push(command);
    }

    Ok((variables, que))
}

fn get_bits(variables: &HashMap<String, usize>, prefix: &str) -> Vec<usize> {
    let mut values = variables
        .iter()
        .filter(|x| x.0.starts_with(prefix))
        .collect::<Vec<_>>();

    values.sort_by_key(|x| x.0);
    values
        .iter()
        .map(|x| *x.1)
        .collect::<Vec<_>>()
}


fn select_commands_with(
    x: String,
    y: String,
    z: String,
    extra: Option<String>,
    commands: &Vec<Command>
) -> Result<Vec<Command>> {
    match extra.clone() {
        Some(extra) => {
            let mut executable_commands = vec![];
            let x_xor_y = commands
                .iter()
                .find(|cmd| {
                    cmd.0 == x && cmd.1 == y && cmd.2 == Operations::XOR
                }).unwrap();
            executable_commands.push(x_xor_y.clone());

            let x_and_y = commands
                .iter()
                .find(|cmd| {
                    cmd.0 == x && cmd.1 == y && cmd.2 == Operations::AND
                }).unwrap();
            executable_commands.push(x_and_y.clone());

            let compute_z = commands
                .iter()
                .find(|cmd| {
                    cmd.3 == z
                }).unwrap();
            executable_commands.push(compute_z.clone());

            let compute_prefix = commands
                .iter()
                .find(|cmd| {
                    (cmd.0 == extra || cmd.1 == extra) && cmd.2 == Operations::AND
                }).context(format!("Can not find command with extra \"{}\" and operation AND", extra))?;
            executable_commands.push(compute_prefix.clone());

            let compute_next_extra = commands
                .iter()
                .find(|cmd| {
                    (cmd.0 == compute_prefix.3 || cmd.1 == compute_prefix.3) && cmd.2 == Operations::OR
                }).context(format!("Can not find command with param '{}' and operation OR", compute_prefix.3))?;
            executable_commands.push(compute_next_extra.clone());
            Ok(executable_commands)
        }
        None => {
            Ok(commands
                .iter()
                .filter(|cmd| {
                    cmd.has_variable(&x) || cmd.has_variable(&y) || cmd.has_variable(&z)
                }).map(|x| x.clone()).collect::<Vec<_>>()
            )
        }
    }
}

fn binary_add(x: &Vec<usize>, y: &Vec<usize>) -> Vec<usize> {
    let mut result = vec![];
    let mut extra = 0;
    for (x, y) in x.iter().zip(y.iter()) {
        let v = x + y + extra;
        result.push(v % 2);
        extra = v / 2;
    }
    if (extra > 0)  {
        result.push(extra);
    }
    result
}
fn check_compute(x: &Vec<usize>, y: &Vec<usize>, z: &Vec<usize>) -> Result<()> {
    let mut t = term::stdout().context("Can't received terminal")?;
    let expected = binary_add(x, y);
    writeln!(t, "Check values")?;
    writeln!(t, "{:10}: {:?}", "x", x)?;
    writeln!(t, "{:10}: {:?}", "y", y)?;
    writeln!(t, "{:10}: {:?}", "real z", expected)?;
    writeln!(t, "{:10}: {:?}", "current z", z)?;
    Ok(())
}

fn compute(variables: &mut HashMap<String, usize>, que: &Vec<Command>) {
    let mut que = VecDeque::from_iter(que.iter().map(|x| x.clone()));
    while let Some(command) = que.pop_front() {
        let Command(a, b, operation, result) = command;
        if variables.contains_key(&a) && variables.contains_key(&b) {
            let a = variables.get(&a).unwrap();
            let b = variables.get(&b).unwrap();
            let value = operation.apply(*a, *b);
            variables.insert(result, value);
        } else {
            que.push_back(Command(a, b, operation, result))
        }
    }
}

/// validates commands by input output params. if there aren't any errors then return next extra
/// Else return error
fn validate_commands(x: String, y: String, z: String, extra: Option<String>, commands: &Vec<Command>) -> Result<String> {
    match extra {
        None => {
            let next_extra = commands.iter().find(|cmd| cmd.2 == Operations::AND).unwrap().3.clone();
            println!("Validate commands with input params {}, {} and output {} {}", x, y, z, next_extra);
            for command in commands {
                println!("{command}");
            }
            for (value_x, value_y, expected_z, expected_extra) in [(0usize, 0usize, 0usize, 0usize), (1, 0, 1, 0), (0, 1, 1, 0), (1, 1, 0, 1)] {
                let mut variables = HashMap::new();
                variables.insert(x.clone(), value_x);
                variables.insert(y.clone(), value_y);
                compute(&mut variables, commands);
                if *variables.get(&z).unwrap_or(&2) != expected_z && *variables.get(&next_extra).unwrap_or(&2) != expected_extra {
                    println!("!!!Commands are not valid valid!!!");
                    return Err(anyhow!("Can not compute"));
                }
            }
            println!("Commands are valid");
            Ok(next_extra)
        },
        Some(extra) => {
            let next_extra = commands.iter().find(|cmd| cmd.2 == Operations::OR).unwrap().3.clone();
            println!("Validate commands with input params {}, {}, {} and output {} {}", x, y, extra, z, next_extra);
            for command in commands {
                println!("{command}");
            }
            for (value_x, value_y, value_extra, expected_z, expected_extra) in [
                (0, 0, 0, 0, 0),
                (1, 0, 0, 1, 0),
                (0, 1, 0, 1, 0),
                (1, 1, 0, 0, 1),
                (0, 0, 1, 1, 0),
                (1, 0, 1, 0, 1),
                (0, 1, 1, 0, 1),
                (1, 1, 1, 1, 1),
            ] {
                let mut variables = HashMap::new();
                variables.insert(x.clone(), value_x);
                variables.insert(y.clone(), value_y);
                variables.insert(extra.clone(), value_extra);
                compute(&mut variables, commands);
                if *variables.get(&z).unwrap_or(&2) != expected_z && *variables.get(&next_extra).unwrap_or(&2) != expected_extra {
                    println!("!!!Commands are not valid valid!!!");
                    return Err(anyhow!("Can not compute"));
                }
            }
            println!("Commands are valid");
            Ok(next_extra)
        }
    }

}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        // TODO: Solve Part 1 of the puzzle
        let (variables, que) = get_input_data(reader)?;
        let mut variables = variables;

        compute(&mut variables, &que);

        let values= get_bits(&variables, "z");
        println!("{:?}", values);
        let answer = values
            .into_iter()
            .enumerate()
            .map(|(step, value)| {
                (value) * 2usize.pow(step as u32)
            })
            .sum();
        Ok(answer)
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(2024, part1(BufReader::new(TEST.as_bytes()))?);

    //let input_file = BufReader::new(File::open(INPUT_FILE)?);
    //let result = time_snippet!(part1(input_file)?);
    //println!("Result = {}", result);
    //endregion

    //region Part 2
    // println!("\n=== Part 2 ===");
    //
    fn part2<R: BufRead>(reader: R) -> Result<String> {
        let (variables, que) = get_input_data(reader)?;
        let mut variables = variables;

        let x = get_bits(&variables, "x");
        let mut extra = None;
        for i in 0..x.len() {
            let x = format!("x{:02}", i);
            let y = format!("y{:02}", i);
            let z = format!("z{:02}", i);
            let test_commands = select_commands_with(x.clone(), y.clone(), z.clone(), extra.clone(), &que)?;
            let next_extra = validate_commands(x.clone(), y.clone(), z.clone(), extra.clone(), &test_commands)?;
            extra = Some(next_extra);
        }
        compute(&mut variables, &que);
        let y = get_bits(&variables, "y");
        let z = get_bits(&variables, "y");
        check_compute(&x, &y, &z);

        Ok("".to_string())
    }

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
