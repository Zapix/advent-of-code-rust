use std::collections::HashMap;
use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;

const DAY: &str = "17"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");


fn get_operand_value(value: usize, registers: &HashMap<char, usize>) -> Result<usize> {
    match value {
        value if value < 4 => Ok(value),
        value if value == 4 => registers.get(&'A').map(|x| *x).context("wrong register"),
        value if value == 5 => registers.get(&'B').map(|x| *x).context("wrong register"),
        value if value == 6 => registers.get(&'C').map(|x| *x).context("wrong register"),
        _ => Err(anyhow!("error wrong combo operand"))
    }
}
fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn compute(a: usize, b: usize, c: usize, programs: &Vec<usize>) -> Result<Vec<usize>> {
        let mut registers = HashMap::from([
            ('A', a),
            ('B', b),
            ('C', c)
        ]);
        let mut operation_pointer = 0;
        let mut output = vec![];
        while operation_pointer < programs.len() {
            let instruction = programs[operation_pointer];
            let operand = programs[operation_pointer + 1];
            match instruction % 8 {
                0 => {
                    let combo_operand = get_operand_value(operand, &registers)?;
                    let value = registers.get_mut(&'A').context("not found")?;
                    *value = *value / 2_u32.pow(combo_operand as u32) as usize;
                },
                1 => {
                    let value = registers.get_mut(&'B').context("not found")?;
                    *value ^= operand;
                },
                2 => {
                    let combo_operand = get_operand_value(operand, &registers)?;
                    let value = registers.get_mut(&'B').context("not found")?;
                    *value = combo_operand % 8
                },
                3 => {
                    if *registers.get(&'A').unwrap() != 0usize {
                        operation_pointer = operand;
                        continue;
                    }
                },
                4 => {
                    let c = *registers.get(&'C').context("not found")?;
                    let b = registers.get_mut(&'B').context("not found")?;
                    *b ^= c;
                },
                5 => {
                    let combo_operand = get_operand_value(operand, &registers)?;
                    output.push(combo_operand % 8);
                }
                6 => {
                    let combo_operand = get_operand_value(operand, &registers)?;
                    let a = *registers.get(&'A').context("not found")?;
                    let value = registers.get_mut(&'B').context("not found")?;
                    *value = a / 2_usize.pow(combo_operand as u32);
                },
                7 => {
                    let combo_operand = get_operand_value(operand, &registers)?;
                    let a = *registers.get(&'A').context("not found")?;
                    let value = registers.get_mut(&'C').context("not found")?;
                    *value = a / 2_usize.pow(combo_operand as u32);
                },
                _ => panic!("unexpected behavior")
            }
            operation_pointer += 2;
        }
        Ok(output)
    }

    // TODO: Set the expected answer for the test input
    let test_program = vec![0,1,5,4,3,0];
    let expected: Vec<usize> = vec![4,6,3,5,6,3,5,2,1,0];
    assert_eq!(expected, compute(729, 0, 0, &test_program)?);

    let registers= HashMap::from([
        ('A', 59397658),
        ('B', 0),
        ('C', 0),
    ]);
    let program = vec![2,4,1,1,7,5,4,6,1,4,0,3,5,5,3,0];
    let result = time_snippet!(compute(59397658, 0, 0, &program)?);
    let value = result.into_iter().map(|x|x.to_string()).collect::<Vec<_>>().join(",");
    println!("Result = {}", value);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn rev_compute(program: &Vec<usize>) -> Result<usize> {
        let n = program.len();
        let mut a = 0usize;
        for i in (0..n).rev() {
            a <<= 3;
            let expected = program[i..].into_iter().map(|x| *x).collect::<Vec<usize>>();
            while compute(a, 0, 0, program)? != expected {
                a += 1;
            }
        }
        Ok(a)
    }

    let test_program = vec![0,3,5,4,3,0];
    let result = rev_compute(&test_program)?;
    assert_eq!(117440, result);
    let result = time_snippet!(rev_compute(&program)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
