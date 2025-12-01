use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;
use crate::Block::FreeBlock;

const DAY: &str = "09"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
2333133121414131402\
"; // TODO: Add the test input

type FileId = usize;
type Size = usize;
type Offset = usize;

#[derive(Debug, Eq, PartialEq)]
enum Block {
    FreeBlock,
    FileBlock(FileId),
}

impl Display for Block {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Block::FreeBlock => write!(f, "[.]"),
            Block::FileBlock(file_id) => write!(f, "[{}]", file_id)
        }
    }
}

fn read_data<R: BufRead>(mut reader: R) -> Result<Vec<Block>> {
    let mut raw_data = String::new();
    reader.read_line(&mut raw_data);
    raw_data.trim();
    let mut data = vec![];
    let mut file_id: FileId = 0;
    for (i, ch) in raw_data.chars().enumerate() {
        let size = ch as u8 - '0' as u8;
        if i % 2 == 0 {
            for _ in 0..size {
                data.push(Block::FileBlock(file_id));
            }
            file_id += 1;
        } else {
            for _ in 0..size {
                data.push(Block::FreeBlock);
            }
        }
    }

    Ok(data)
}

fn compress_data(data: &mut Vec<Block>) -> Result<()> {
    let mut i = 0;
    let mut j = data.len() - 1;
    while data[j] == Block::FreeBlock {
        j -= 1;
    }
    while i < j {
        data[i] = match data[i] {
            Block::FileBlock(file_id) => Block::FileBlock(file_id),
            Block::FreeBlock => {
                let result= match data[j] {
                    Block::FileBlock(file_id) => Block::FileBlock(file_id),
                    Block::FreeBlock => {
                        return Err(anyhow!("Should not bee a free block"));
                    }
                };
                data[j] = Block::FreeBlock;
                result
            }
        };

        while data[j] == Block::FreeBlock {
            j -= 1;
        }

        i += 1;
    }

    Ok(())
}

fn compute_free_space(data: &Vec<Block>, offset: Offset) -> Size {
    let mut i = 0usize;
    while data[offset + i] == Block::FreeBlock {
        i += 1;
    }
    i
}

fn compute_last_file_size(data: &Vec<Block>, file_id: FileId, offset: usize) -> usize {
    let mut j = offset.clone();
    while j > 0 && data[j] ==Block::FileBlock(file_id) {
        j -= 1;
    }
    offset - j
}

fn compute_file_size(data: &Vec<Block>, file_id: FileId, offset: Offset) -> Size {
    let mut j = 0;
    while offset + j < data.len() && data[offset + j] == Block::FileBlock(file_id) {
        j += 1;
    }
    j
}

fn read_file_id(data: &Vec<Block>, offset: Offset) -> Result<FileId> {
    match data[offset] {
        Block::FreeBlock => Err(anyhow!("Unexpected free block")),
        Block::FileBlock(file_id) => Ok(file_id)
    }
}

fn write_file(data: &mut Vec<Block>, offset: Offset, file_id: FileId, size: Size) -> Result<Offset> {
    for i in 0..size {
        data[offset + i] = match data[offset + i] {
            Block::FreeBlock => Block::FileBlock(file_id),
            Block::FileBlock(_) => return Err(anyhow!("File corruption could not write"))
        }
    }
    Ok(offset + size - 1)
}

fn erase(data: &mut Vec<Block>, offset: Offset, size: Size) {
    for i in 0..size {
        data[offset + i] = Block::FreeBlock;
    }
}

fn read_files(data: &Vec<Block>) -> Vec<(FileId, Offset, Size)> {
    let mut result = vec![];
    let mut i: Offset = 0;
    while i < data.len() {
        i = match data[i] {
            Block::FreeBlock => i + 1,
            Block::FileBlock(file_id) => {
                let size = compute_file_size(data, file_id, i);
                result.push((file_id, i, size));
                i + size
            }
        }
    }
    result
}

fn find_last_file_block(data: &Vec<Block>, offset: usize) -> usize {
    let mut j = 0;
    while offset - j > 0 && data[offset - j] == Block::FreeBlock {
        j += 1;
    }
    offset - j
}


fn compress_v2(data: &mut Vec<Block>) -> Result<()> {
    let mut files_stack = read_files(&data);

    while !files_stack.is_empty() {
        let (file_id, offset, size) = files_stack.pop().unwrap();
        let mut i: Offset = 0;
        while i < offset {
            i = match data[i] {
                Block::FreeBlock => {
                    let free_space = compute_free_space(&data, i);
                    if free_space >= size {
                        write_file(data, i, file_id, size)?;
                        erase(data, offset, size);
                        break;
                    }
                    i + free_space
                },
                _ => i + 1
            };
        }
    }

    Ok(())
}

fn check_sum(data: &Vec<Block>) -> usize {
    data
        .iter()
        .enumerate()
        .map(|(i, x)| match x {
            Block::FreeBlock => 0,
            Block::FileBlock(file_id) => file_id * i,
        })
        .sum()
}

fn print_blocks(data: &Vec<Block>) {
    println!("{}", data.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(""));
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(mut reader: R) -> Result<usize> {
        // TODO: Solve Part 1 of the puzzle
        let mut blocks = read_data(reader)?;
        compress_data(&mut blocks)?;
        let answer = check_sum(&mut blocks);
        Ok(answer)
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(1928, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut blocks = read_data(reader)?;
        compress_v2(&mut blocks)?;
        let answer = check_sum(&mut blocks);
        Ok(answer)
    }

    assert_eq!(2858, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
