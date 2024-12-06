use core::result::Result::Ok;
use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;
use std::collections::{HashSet, HashMap, VecDeque};
use itertools::Itertools;

const DAY: &str = "06"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
"; // TODO: Add the test input

type Grid = Vec<Vec<char>>;

type Position = (usize, usize);


#[derive(Copy, Clone, Eq, PartialEq, Hash)]
enum Direction {
    UP,
    RIGHT,
    DOWN,
    LEFT,
}

fn next_direction(direction: &Direction) -> Direction {
    match direction {
        Direction::UP => Direction::RIGHT,
        Direction::RIGHT => Direction::DOWN,
        Direction::DOWN => Direction::LEFT,
        Direction::LEFT => Direction::UP,
    }
}

fn next_step(direction: &Direction) -> (i32, i32) {
    match direction {
        Direction::UP => (-1, 0),
        Direction::RIGHT => (0, 1),
        Direction::DOWN => (1, 0),
        Direction::LEFT => (0, -1),
    }
}

type DirectionLinesInfo = HashMap<Direction, HashSet<usize>>;

fn build_directions_line_info() -> DirectionLinesInfo {
    HashMap::from([
        (Direction::UP, HashSet::<usize>::new()),
        (Direction::RIGHT, HashSet::<usize>::new()),
        (Direction::LEFT, HashSet::<usize>::new()),
        (Direction::DOWN, HashSet::<usize>::new()),
    ])
}
fn find_guard(grid: &Grid) -> Result<Position> {
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            match grid[i][j] {
                '^' => {
                    return Ok((i, j));
                },
                _ => {},
            }
        }
    }
    Err(anyhow!("Start not found"))
}

fn move_forward(guard: &Position, direction: &Direction, grid: &Grid, obstacle: Option<Position>) -> Result<(Position, Direction)> {
    let mut n_direction = *direction;
    for _ in 0..4 {
        let delta = next_step(&n_direction);
        let nx =usize::try_from(guard.0 as i32 + delta.0)?;
        let ny =usize::try_from(guard.1 as i32 + delta.1)?;

        match obstacle {
            Some(position) => {
                if position == (nx, ny) {
                    n_direction = next_direction(direction);
                    return Ok((*guard, n_direction))
                }
            }
            None => {},
        }

        match grid.get(nx).context("now row")?.get(ny).context("no column")? {
            '#' => {
                n_direction = next_direction(direction);
                return Ok((*guard, n_direction))
            }
            _ => {
                return Ok(((nx, ny), n_direction));
            }
        }
    }
    Err(anyhow!("Can't move"))
}

fn read_grid<R: BufRead>(reader: R) -> Grid {
    reader.lines().flatten().map(|x| x.chars().collect::<Vec<_>>()).collect::<Vec<_>>()
}

fn insert_direction(guard: &Position, direction: &Direction, lines: &mut DirectionLinesInfo) {
    let value = match direction {
        Direction::UP | Direction::DOWN => guard.1,
        Direction::LEFT | Direction::RIGHT => guard.0
    };
    lines.get_mut(direction).unwrap().insert(value);
}

fn try_set_obstacle_on_next_cell_2(guard: &Position, direction: &Direction, grid: &Grid) -> Result<()> {
    let delta = next_step(direction);
    let mut visited = HashSet::new();
    let nx =usize::try_from(guard.0 as i32 + delta.0)?;
    let ny =usize::try_from(guard.1 as i32 + delta.1)?;
    if grid.get(nx).context("now row")?.get(ny).context("no column")? == &'#' {
        return Err(anyhow!("can't set obstacle"))
    }

    let mut guard = guard.clone();
    let mut direction = next_direction(direction);
    visited.insert((guard.clone(), direction));
    while let Ok((next_guard, next_direction)) = move_forward(&guard, &direction, grid, Some((nx, ny))) {
        if visited.contains(&(next_guard, next_direction)) {
            return Ok(())
        }
        guard = next_guard;
        direction = next_direction;
        visited.insert((guard, direction));
    }
    Err(anyhow!("Can not find loop"))
}

fn can_set_obstacle_on_next_cell_2(guard: &Position, direction: &Direction, grid: &Grid) -> bool {
    try_set_obstacle_on_next_cell_2(guard, direction, grid).is_ok()
}

fn try_found_loop(start_point: &Position, obstacle: &Position, grid: &Grid) -> Result<()> {
    if start_point == obstacle {
        return Err(anyhow!("Can't block start point"));
    }
    let mut visited = HashSet::new();
    let mut guard = *start_point;
    let mut direction = Direction::UP;

    while let Ok((next_guard, next_direction)) = move_forward(&guard, &direction, grid, Some(*obstacle)) {
        if visited.contains(&(next_guard, next_direction)) {
            return Ok(())
        }
        visited.insert((next_guard, next_direction));
        guard = next_guard;
        direction = next_direction;
    }

    Err(anyhow!("Can't find loop"))
}

fn get_visited_points(start: &Position, grid: &Grid) -> HashSet<Position> {
    let mut guard = *start;
    let mut direction = Direction::UP;
    let mut visited = HashSet::new();
    visited.insert(guard);

    while let Ok((next_guard,next_direction)) = move_forward(&guard, &direction, &grid, None) {
        guard = next_guard;
        direction = next_direction;
        visited.insert(next_guard);
    }

    visited
}
fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        // TODO: Solve Part 1 of the puzzle
        let grid = read_grid(reader);
        let guard = find_guard(&grid)?;
        let visited = get_visited_points(&guard, &grid);
        Ok(visited.iter().count())
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(41, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let grid = read_grid(reader);
        let guard = find_guard(&grid)?;
        let visited = get_visited_points(&guard, &grid);
        let mut obstacle_count = 0;

        for point in visited {
            if try_found_loop(&guard, &point, &grid).is_ok() {
                obstacle_count += 1
            }
        }

        Ok(obstacle_count)
    }

    assert_eq!(6, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
