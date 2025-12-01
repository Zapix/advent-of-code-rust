use anyhow::*;
use std::collections::{binary_heap, BinaryHeap};
use std::cmp::Reverse;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;

const DAY: &str = "18"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0
"; // TODO: Add the test input

type Point = (usize, usize);

fn read_point(s: &str) -> Result<Point> {
    let mut parts = s.split(',');
    let x = parts.next().ok_or(anyhow!("Invalid point"))?.parse()?;
    let y = parts.next().ok_or(anyhow!("Invalid point"))?.parse()?;
    Ok((x, y))
}

fn read_points<R: BufRead>(reader: R) -> Result<Vec<Point>> {
    reader.lines().map(|l| read_point(&l?)).collect()
}

fn print_grid(grid: &Vec<Vec<usize>>) {
    for row in grid {
        for cell in row {
            match cell {
                0 => print!("."),
                1 => print!("#"),
                _ => print!("?"),
            }
        }
        println!();
    }
}

fn manhattan_distance(p1: Point, p2: Point) -> usize {
    let (x1, y1) = p1;
    let (x2, y2) = p2;
    ((x1 as isize - x2 as isize).abs() + (y1 as isize - y2 as isize).abs()) as usize
}

fn a_star(start_point: Point, end_point: Point, grid: &Vec<Vec<usize>>) -> Result<Vec<Point>> {
    let mut que = BinaryHeap::new();
    que.push((Reverse(manhattan_distance(start_point, end_point)), vec![start_point]));
    let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
    while let Some((Reverse(cost), path)) = que.pop() {
        let current = path.last().unwrap();
        if current == &end_point {
            return Ok(path);
        }
        let (x, y) = current;
        if grid[*x][*y] == 1 {
            continue;
        }
        if visited[*x][*y] {
            continue;
        }
        for (dx, dy) in &[(0, 1), (1, 0), (0, -1), (-1, 0)] {
            let new_x = (*x as isize + dx) as usize;
            let new_y = (*y as isize + dy) as usize;
            if new_x < grid.len() && new_y < grid[0].len() {
                let new_point = (new_x, new_y);
                if !path.contains(&new_point) {
                    let mut new_path = path.clone();
                    new_path.push(new_point);
                    que.push((Reverse(cost + manhattan_distance(new_point, end_point)), new_path));
                }
            }
        }
        visited[*x][*y] = true;
    }
    Err(anyhow!("No path found"))
}


fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R, n: usize, m: usize, steps: usize) -> Result<usize> {
        // TODO: Solve Part 1 of the puzzle
        let points = read_points(reader)?;
        let mut grid = vec![vec![0usize; m]; n];
        for (i, j) in points.iter().take(steps) {
            grid[*i][*j] = 1;
        }
        let path = a_star((0, 0), (n - 1, m - 1), &grid)?;
        print_grid(&grid);
        Ok(path.len() - 1)
        
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(22, part1(BufReader::new(TEST.as_bytes()), 7, 7, 12)?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file, 71, 71, 1024)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");
    
    fn part2<R: BufRead>(reader: R, n: usize, m: usize, steps: usize) -> Result<(usize, usize)> {
        let points = read_points(reader)?;
        let mut grid = vec![vec![0usize; m]; n];
        for (i, j) in points.iter().take(steps) {
            grid[*i][*j] = 1;
        }
        for  (i, j) in points.iter().skip(steps) {
            grid[*i][*j] = 1;
            match a_star((0, 0), (n - 1, m - 1), &grid) {
                Err(_) => return Ok((*i, *j)),
                _ => {}
            }
        }
        Err(anyhow!("Always reachable"))
    }
    
    assert_eq!((6, 1), part2(BufReader::new(TEST.as_bytes()), 7, 7, 12)?);
    
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file, 71, 71, 1024)?);
    println!("Result = {:?}", result);
    //endregion

    Ok(())
}
