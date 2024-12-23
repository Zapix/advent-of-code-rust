use core::result::Result::Ok;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::env::var;
use std::fmt::{Display, Formatter};
use anyhow::*;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufRead, BufReader, Stdout};
use std::iter;
use std::rc::Rc;
use std::str::FromStr;
use std::time::Duration;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::{enumerate, Itertools, Position};
use adv_code_2024::*;
use itertools::structs::Permutations;
use term::StdoutTerminal;

const DAY: &str = "21"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const DIGITAL_PAD_SYMBOLS: [char; 11] = ['A', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
const DIRECTION_PAD_SYMBOLS: [char; 5] = ['A', '<', '>', '^', 'v'];

const TEST: &str = "\
029A
980A
179A
456A
379A
"; // TODO: Add the test input

fn digital_pad(symbol: char) -> Result<(i32, i32)> {
    match symbol {
        '7' => Ok((0, 0)),
        '8' => Ok((0, 1)),
        '9' => Ok((0, 2)),
        '4' => Ok((1, 0)),
        '5' => Ok((1, 1)),
        '6' => Ok((1, 2)),
        '1' => Ok((2, 0)),
        '2' => Ok((2, 1)),
        '3' => Ok((2, 2)),
        '0' => Ok((3, 1)),
        'A' => Ok((3, 2)),
        _ => Err(anyhow!("Unexpected symbol '{symbol}' on digital pad")),
    }
}

fn position_to_digital_pad(position: (i32, i32)) -> Result<char> {
    match position {
        (0, 0) => Ok('7'),
        (0, 1) => Ok('8'),
        (0, 2) => Ok('9'),
        (1, 0) => Ok('4'),
        (1, 1) => Ok('5'),
        (1, 2) => Ok('6'),
        (2, 0) => Ok('1'),
        (2, 1) => Ok('2'),
        (2, 2) => Ok('3'),
        (3, 1) => Ok('0'),
        (3, 2) => Ok('A'),
        _ => Err(anyhow!("Unexpected position ({}, {}) on digital pad", position.0, position.1)),
    }
}

fn validate_digital_position(position: (i32, i32)) -> bool {
    matches!(
        position,
        (0, 0) | (0, 1) | (0, 2) |
        (1, 0) | (1, 1) | (1, 2) |
        (2, 0) | (2, 1) | (2, 2) |
        (3, 1) | (3, 2)
    )
}

fn directional_pad(symbol: char) -> Result<(i32, i32)> {
    match symbol {
        '^' => Ok((0, 1)),
        'A' => Ok((0, 2)),
        '<' => Ok((1, 0)),
        'v' => Ok((1, 1)),
        '>' => Ok((1, 2)),
        _ => Err(anyhow!("Unexpected symbol '{symbol}' on directional pad"))
    }
}

fn directional_position_to_symbol(position: (i32, i32)) -> Result<char> {
    match position {
        (0, 1) => Ok('^'),
        (0, 2) => Ok('A'),
        (1, 0) => Ok('<'),
        (1, 1) => Ok('v'),
        (1, 2) => Ok('>'),
        _ => Err(anyhow!(format!("Unexpected position ({}, {}) for directional pad", position.0, position.1)))
    }
}

fn validate_directional_position(position: (i32, i32)) -> bool {
    matches!(
        position,
        (0, 1) | (0, 2) |
        (1, 0) | (1, 1) | (1, 2)
    )
}

type GetChar = Box<dyn Fn((i32, i32)) -> Result<char>>;
type OnPress = Box<dyn Fn(char) -> Result<()>>;

trait Robot {
    fn move_up(&mut self) -> Result<()>;
    fn move_down(&mut self) -> Result<()>;
    fn move_left(&mut self) -> Result<()>;
    fn move_right(&mut self) -> Result<()>;
    fn press(&mut self) -> Result<()>;
}

struct Pad {
    name: String,
    position: (i32, i32),
    get_char: GetChar,
    on_press: Option<OnPress>,
}

impl Pad {
    fn new(
        name: String,
        position: (i32, i32),
        get_char: GetChar,
        on_press: Option<OnPress>
    ) -> Self {
        Self { name, position, get_char, on_press }
    }
}

impl Display for Pad {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.name, (self.get_char)(self.position).unwrap())
    }
}

impl Robot for Pad {
    fn move_up(&mut self) -> Result<()> {
        self.position = (
            self.position.0 - 1,
            self.position.1
        );
        (self.get_char)(self.position)?;
        Ok(())
    }

    fn move_down(&mut self) -> Result<()> {
        self.position = (
            self.position.0 + 1,
            self.position.1
        );
        (self.get_char)(self.position)?;
        Ok(())
    }

    fn move_left(&mut self) -> Result<()> {
        self.position = (
            self.position.0,
            self.position.1 - 1
        );
        (self.get_char)(self.position)?;
        Ok(())
    }

    fn move_right(&mut self) -> Result<()> {
        self.position = (
            self.position.0,
            self.position.1 + 1
        );
        (self.get_char)(self.position)?;
        Ok(())
    }

    fn press(&mut self) -> Result<()> {
        match &self.on_press {
            Some(on_press) => on_press((self.get_char)(self.position)?),
            None => {
                Ok(())
            }
        }
    }
}

fn get_moves(dx: i32, dec_char: char, inc_char: char) -> Result<Vec<char>> {
    let ch = match dx {
        0 => {'.'},
        x if x > 0 => {
            inc_char
        },
        x if x < 0 => {
            dec_char
        },
        _ => {
            return Err(anyhow!("unexpected number"));
        }
    };
    Ok(iter::repeat(ch).take(dx.abs() as usize).collect::<Vec<_>>())
}


fn gen_all_variants<ValidatePosition>(
    start_point: (i32, i32),
    end_point: (i32, i32),
    validate_position: ValidatePosition,
) -> Result<Vec<String>>
where
    ValidatePosition: Fn((i32, i32)) -> bool,
{
    let dx = end_point.0 - start_point.0;
    let dy = end_point.1 - start_point.1;
    let moves = get_moves(dx, '^', 'v')?
        .iter().
        chain(
            get_moves(dy, '<', '>')?.iter()
        )
        .permutations((dx.abs() + dy.abs()) as usize)
        .filter(|x| {
            let mut position = start_point;
            for mv in x {
                position = match mv {
                    '^' => (position.0  - 1, position.1),
                    'v' => (position.0  + 1, position.1),
                    '<' => (position.0, position.1 - 1),
                    '>' => (position.0, position.1 + 1),
                    _ => (-1, -1)
                };
                if !validate_position(position) {
                    return false;
                }
            }
            true
        })
        .map(|x| format!("{}A", String::from_iter(x)))
        .collect::<HashSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    Ok(moves)
}

fn get_minimum<T, ToCoord, GetCost, ValidatePosition>(ch1: char, ch2: char, to_coord: ToCoord, get_cost: GetCost, validate_position: ValidatePosition) -> Result<T>
    where
        T: Ord + Clone,
        ToCoord: Fn(char) -> Result<(i32, i32)>,
        GetCost: Fn(&String) -> T,
        ValidatePosition: Fn((i32, i32)) -> bool
{
    gen_all_variants(
        to_coord(ch1)?,
        to_coord(ch2)?,
        validate_position,
    )?
        .iter()
        .map(|x| {
            let cost = get_cost(x);
            cost
        })
        .collect::<Vec<_>>()
        .iter().min().context("No value").map(|x| x.clone())
}

type Graph<Item: Ord + Clone + Display> = HashMap<(char, char), Item>;

fn build_graph<T: Clone, GetCost>(
    symbols: &Vec<char>,
    get_cost: &GetCost,
    default: T,
) -> Result<Graph<T>>
    where
        T: std::cmp::Ord,
        GetCost: Fn(char, char) -> Result<T>,
{
    let mut graph: Graph<T> = HashMap::new();
    for ch1 in symbols {
        for ch2 in symbols {
            if ch1 == ch2 {
                graph.insert((*ch1, *ch2), default.clone());
            } else {
                graph.insert((*ch1, *ch2), get_cost(*ch1, *ch2)?);
            }
        }
    }
    Ok(graph)
}

fn build_get_cost_fn<'a, T: Ord + Clone, Add>(
    graph: &'a Graph<T>,
    default: T,
    add: &'a Add,
) -> Box<(dyn Fn(&String) -> T + 'a)>
    where
        T: Ord + Clone,
        Add: Fn(T, T) -> T + 'a
{
    Box::new(move |x: &String| {
        let mut value = default.clone();
        let mut prev = 'A';
        for (i, ch) in x.chars().enumerate() {
            value = add(value, graph.get(&(prev, ch)).unwrap().clone());
            prev = ch;
        }
        value
    })
}

fn print_graph<T: Display>(symbols: &Vec<char>, graph: &Graph<T>) {
    for ch1 in symbols {
        for ch2 in symbols {
            println!("'{}'->'{}': {}", *ch1, *ch2, graph.get(&(*ch1, *ch2)).unwrap());
        }
    }
}

fn get_40_variants(code: &String) -> Result<Vec<String>> {
    let mut variants = vec![];
    let mut next_variants = vec![];

    let mut current = &mut variants;
    current.push("".to_string());
    let mut next = &mut next_variants;
    let mut prev = 'A';
    for ch in code.chars() {
        next.clear();
        for variant in gen_all_variants(directional_pad(prev)?, directional_pad(ch)?, Box::new(validate_digital_position))? {
            for cur in current.iter() {
                next.push(format!("{}{}", cur, variant));
            }
        }
        (current, next) = (next, current);
        prev = ch;
    }
    Ok(current.into_iter().map(|x| format!("{x}")).collect::<Vec<_>>())
}

fn debug_graph(ch1: char, ch2: char, graph: &Graph<String>) {
    println!("'{}'->'{}': {}", ch1, ch2, graph.get(&(ch1, ch2)).unwrap());
}

fn display_min(t: &mut Box<dyn term::Terminal<Output=Stdout> + Send>, value: usize) -> Result<()> {
    t.fg(term::color::GREEN)?;
    writeln!(t, "Min: {value}")?;
    t.fg(term::color::WHITE).context("Can not convert")
}

fn get_cost_on_40(ch1: char, ch2: char) -> Result<usize> {
   let start_point = directional_pad(ch1)?;
   let end_point = directional_pad(ch2)?;
    gen_all_variants(start_point, end_point, Box::new(validate_directional_position))?
        .into_iter()
        .map(|x| x.len())
        .min()
        .context(
            format!("No moves on cost_40 from '{}' to '{}'", ch1, ch2)
        )
}

fn compute_cost<T, Add>(code: &String, costs: &Graph<T>, initial: T, add: &Add) -> T
    where
        T: Clone,
        Add: Fn(T, T) -> T
{   let mut value: T =  initial;
    let mut prev = '-';
    for ch in code.chars() {
        if prev != '-' {
            value = add(value, costs.get(&(prev, ch)).unwrap().clone());
        }
        prev = ch;
    }
    value
}

fn get_cost_on_pad<GetPoint, ValidatePosition>(ch1: char, ch2: char, costs: &Graph<usize>, get_point: &GetPoint, validate_position: &ValidatePosition) -> Result<usize>
    where
        GetPoint: Fn(char) -> Result<(i32, i32)>,
        ValidatePosition: Fn((i32, i32)) -> bool
{
    let start_point = get_point(ch1)?;
    let end_point = get_point(ch2)?;
    let variants = gen_all_variants(start_point, end_point, Box::new(validate_position))?;
    variants
        .iter()
        .map(|x| {
            let cost = compute_cost(&format!("A{x}"), costs, 0, &|a, b| a + b);
            cost
        })
        .min()
        .context(
            format!("No moves on direction pad '{}' to '{}'", ch1, ch2)
        )
}

fn highlight_symbol(str: &String, idx: usize, t: &mut Box<StdoutTerminal>) -> Result<()> {
    for (i, ch) in str.chars().enumerate() {
        if i == idx {
            t.fg(term::color::BRIGHT_RED)?;
            write!(t, "{ch}")?;
            t.fg(term::color::BRIGHT_WHITE)?;
        } else {
            write!(t, "{ch}")?
        }
    }
    writeln!(t).context("Can not print")
}

fn debug_code(code: String) -> Result<()> {
    let mut current_output = vec![];
    let current_output = Rc::new(RefCell::new(current_output));
    let push_char = {
        let current_output = current_output.clone();
        move |ch: char| {
            current_output.clone().borrow_mut().push(ch);
            Ok(())
        }
    };
    let mut pad_depressure = Pad::new(
        "Pad depressure".to_string(),
        (3, 2),
        Box::new(position_to_digital_pad),
        Some(Box::new(push_char))
    );
    let mut pad_depressure = Rc::new(RefCell::new(pad_depressure));
    let push_depressure= {
        let pad = pad_depressure.clone();
        move |ch: char| {
            let mut pad = (*pad).borrow_mut();
            match ch {
                'A' => pad.press(),
                '^' => pad.move_up(),
                'v' => pad.move_down(),
                '<' => pad.move_left(),
                '>' => pad.move_right(),
                _ => {
                    Err(anyhow!("Un expected char '{ch}' for pad radi"))
                }
            }
        }
    };
    let mut pad_radi = Pad::new(
        "Pad radi".to_string(),
        (0, 2),
        Box::new(directional_position_to_symbol),
        Some(Box::new(push_depressure))
    );
    let mut pad_radi = Rc::new(RefCell::new(pad_radi));
    let push_radi = {
        let pad_radi = pad_radi.clone();
        move |ch: char| {
            let mut pad_radi = (*pad_radi).borrow_mut();
            match ch {
                'A' => pad_radi.press(),
                '^' => pad_radi.move_up(),
                'v' => pad_radi.move_down(),
                '<' => pad_radi.move_left(),
                '>' => pad_radi.move_right(),
                _ => {
                    Err(anyhow!("Un expected char '{ch}' for pad radi"))
                }
            }
        }
    };
    let mut pad_40 = Pad::new(
        "Pad 40".to_string(),
        (0, 2),
        Box::new(directional_position_to_symbol),
        Some(Box::new(push_radi))
    );
    let mut t = term::stdout().context("Can not get stdout")?;
    t.fg(term::color::BRIGHT_GREEN)?;
    writeln!(t, "Debug code")?;
    t.fg(term::color::BRIGHT_WHITE)?;
    for (i, ch) in code.clone().chars().enumerate() {
        if i != 0 {
            t.delete_line()?;
            t.cursor_up()?;
            t.delete_line()?;
            t.cursor_up()?;
            t.delete_line()?;
            t.cursor_up()?;
            t.delete_line()?;
            t.cursor_up()?;
            t.delete_line()?;
            t.cursor_up()?;
            t.delete_line()?;
        }
        match ch {
           'A' => pad_40.press()?,
           '^' => pad_40.move_up()?,
           'v' => pad_40.move_down()?,
           '<' => pad_40.move_left()?,
           '>' => pad_40.move_right()?,
            ch => {
                return Err(anyhow!(format!("Error on uexpected char '{ch}'")));
            }
        }
        highlight_symbol(&code, i,&mut t)?;
        writeln!(t, "{}", pad_40)?;
        writeln!(t, "{}", pad_radi.borrow())?;
        writeln!(t, "{}", pad_depressure.borrow())?;
        writeln!(t, "Current output: {}", String::from_iter(current_output.clone().borrow().iter()))?;
        std::thread::sleep(Duration::from_millis(100));
    }
    Ok(())
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        // TODO: Solve Part 1 of the puzzle
        /// How much cost on normal pad to move and press symbol on 40 pad
        let graph_40 = build_graph(
            &Vec::from(DIRECTION_PAD_SYMBOLS),
            &get_cost_on_40,
            1,
        )?;

        /// How much cost on normal pad to move and press symbol on radi pad
        let graph_radi = build_graph(
            &Vec::from(DIRECTION_PAD_SYMBOLS),
            &|x, y| get_cost_on_pad(x, y, &graph_40, &directional_pad, &validate_directional_position),
            1
        )?;

        let graph_pressure = build_graph(
            &Vec::from(DIGITAL_PAD_SYMBOLS),
            &|x, y| get_cost_on_pad(x, y, &graph_radi, &digital_pad, &validate_digital_position),
            1
        )?;

        Ok(reader
            .lines()
            .flatten()
            .map(|code| {
                let value = usize::from_str(code[..3].trim_ascii_start()).context("can not parse")?;
                let num_code = compute_cost(
                    &format!("A{code}"),
                    &graph_pressure,
                    0,
                    &|x, y| x + y
                );
                println!("{code}: {num_code}");
                Ok(num_code * value)
            })
            .collect::<Result<Vec<_>>>()?
            .into_iter()
            .sum())
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(126384, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut graph = build_graph(
            &Vec::from(DIRECTION_PAD_SYMBOLS),
            &get_cost_on_40,
            1,
        )?;

        for _ in 0..24 {
            graph = build_graph(
                &Vec::from(DIRECTION_PAD_SYMBOLS),
                &|x, y| get_cost_on_pad(x, y, &graph, &directional_pad, &validate_directional_position),
                1
            )?;
        }

        let graph_pressure = build_graph(
            &Vec::from(DIGITAL_PAD_SYMBOLS),
            &|x, y| get_cost_on_pad(x, y, &graph, &digital_pad, &validate_digital_position),
            1
        )?;

        Ok(reader
            .lines()
            .flatten()
            .map(|code| {
                let value = usize::from_str(code[..3].trim_ascii_start()).context("can not parse")?;
                let num_code = compute_cost(
                    &format!("A{code}"),
                    &graph_pressure,
                    0,
                    &|x, y| x + y
                );
                println!("{code}: - {num_code}");
                Ok(num_code * value)
            })
            .collect::<Result<Vec<_>>>()?
            .into_iter()
            .sum())
    }

    //
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
