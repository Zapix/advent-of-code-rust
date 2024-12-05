use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;

const DAY: &str = "04"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
"; // TODO: Add the test input

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn is_xmas_word(matrix: &Vec<Vec<char>>, x: usize, y: usize, dx: i32, dy: i32) -> Result<()> {
        let pattern = ['X', 'M', 'A', 'S'];
        let mut x = x;
        let mut y = y;
        for i in 0..pattern.len() {
            if pattern[i] as u8 != (*matrix.get(x).context("can not get vec")?.get(y).context("can not get char")?) as u8 {
                return Err(anyhow!("error"));
            }
            if i != pattern.len() - 1 {
                x = usize::try_from(x as i32 + dx)?;
                y = usize::try_from(y as i32 + dy)?;
            }
        }
        Ok(())
    }
    fn count_xmas_word(matrix: &Vec<Vec<char>>, x: usize, y: usize) -> usize {
        [(0i32, 1i32), (0, -1), (1, 0), (-1, 0), (1, 1), (-1, -1), (1, -1), (-1, 1)]
            .into_iter()
            .filter(|item| is_xmas_word(matrix, x, y, item.0, item.1).is_ok()).count()
    }

    fn is_mas_diagonal(matrix: &Vec<Vec<char>>, x: usize, y: usize, dx: i32, dy: i32) -> Result<()> {
        let pattern = ['M', 'A', 'S'];
        let inc = [(-1, -1), (0, 0), (1, 1)];
        for i in 0..3 {
            let nx = usize::try_from(x as i32 + dx * inc[i].0)?;
            let ny = usize::try_from(y as i32 + dy * inc[i].1)?;
            if pattern[i] as u8 != (*matrix.get(nx).context("can not get vec")?.get(ny).context("can not get char")?) as u8 {
                return Err(anyhow!("error"));
            }
        }
        Ok(())
    }

    fn is_cross_word(matrix: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
        (
            is_mas_diagonal(matrix, x, y, 1, 1).is_ok() || is_mas_diagonal(matrix, x, y, -1, -1).is_ok()
        ) && (
            is_mas_diagonal(matrix, x, y, -1, 1).is_ok() || is_mas_diagonal(matrix, x, y, 1, -1).is_ok()
        )
    }

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        // TODO: Solve Part 1 of the puzzle
        let matrix = reader.lines().flatten().map(|x| x.chars().collect::<Vec<_>>()).collect::<Vec<_>>();
        let mut answer = 0;
        for i in 0..matrix.len() {
            for j in 0..matrix[i].len() {
                answer += count_xmas_word(&matrix, i, j);
            }
        }
        Ok(answer)
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(18, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let matrix = reader.lines().flatten().map(|x| x.chars().collect::<Vec<_>>()).collect::<Vec<_>>();
        let mut answer = 0;
        for i in 0..matrix.len() {
            for j in 0..matrix[i].len() {
                answer += if is_cross_word(&matrix, i, j) { 1 } else { 0 };
            }
        }

        Ok(answer)
    }

    assert_eq!(9, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
