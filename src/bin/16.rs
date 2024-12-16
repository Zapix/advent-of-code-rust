use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::{BinaryHeap, HashSet, HashMap};
use std::cmp::Reverse;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;

const DAY: &str = "16"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST_1: &str = "\
###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############
"; // TODO: Add the test input

const TEST_2: &str = "\
#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################
";


type Point = (usize, usize);

fn find_start_end_point(grid: &Grid) -> (Point, Point) {
    let mut start_point = None;
    let mut end_point = None;
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            match grid[i][j] {
                'E' => end_point = Some((i, j)),
                'S' => start_point = Some((i, j)),
                _ => {},
            }
        }
    }
    (start_point.unwrap(), end_point.unwrap())
}
type Grid = Vec<Vec<char>>;

fn read_map<R: BufRead>(reader: R) -> Grid {
    reader
        .lines()
        .flatten()
        .map(|x| x.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>()
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
enum Direction {
    East,
    South,
    West,
    North,
}

impl Direction {
    fn rotate_clockwise(&self) -> Direction {
        match self {
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West=> Direction::North,
            Direction::North => Direction::East,
        }
    }

    fn rotate_counterclockwise(&self) -> Direction {
        match self {
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West=> Direction::South,
            Direction::North => Direction::West,
        }
    }

    fn delta(&self) -> (i32, i32) {
        match self {
            Direction::East => (0, 1),
            Direction::South => (1, 0),
            Direction::West=> (0, -1),
            Direction::North => (-1, 0),
        }
    }
}

type Position = (Point, Direction);

fn next_position(point: &Point, direction: &Direction) -> Result<Point> {
    let delta = direction.delta();
    Ok((
        usize::try_from(point.0 as i32 + delta.0)?,
        usize::try_from(point.1 as i32 + delta.1)?,
    ))
}

fn prev_position(point: &Point, direction: &Direction) -> Result<Point> {
    let delta = direction.delta();
    Ok((
        usize::try_from(point.0 as i32 - delta.0)?,
        usize::try_from(point.1 as i32 - delta.1)?,
    ))
}

fn print_grid_with_seats(grid: &Grid, seats: &HashSet<Point>) {
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if seats.contains(&(i, j)) {
                print!("O");
            } else {
                print!("{}", grid[i][j]);
            }
        }
        print!("\n");
    }
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        // TODO: Solve Part 1 of the puzzle
        let grid = read_map(reader);
        let (start_point, end_point) = find_start_end_point(&grid);
        let mut que = BinaryHeap::new();
        let mut visited = HashSet::new();
        que.push((Reverse(0usize), (start_point, Direction::East)));
        while let Some((Reverse(cost), (position, direction))) = que.pop() {
            if visited.contains(&(position, direction)) {
                continue;
            }
            match grid[position.0][position.1] {
                'E' => {
                    return Ok(cost)
                },
                '#' => {},
                _ => {
                    que.push((Reverse(cost + 1), (next_position(&position, &direction)?, direction)));
                    que.push((Reverse(cost + 1000), (position, direction.rotate_clockwise())));
                    que.push((Reverse(cost + 1000), (position, direction.rotate_counterclockwise())));
                }
            }
            visited.insert((position, direction));
        }
        Err(anyhow!("End is unreachable"))
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(7036, part1(BufReader::new(TEST_1.as_bytes()))?);
    assert_eq!(11048, part1(BufReader::new(TEST_2.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");
    //
    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let grid = read_map(reader);
        let (start_point, end_point) = find_start_end_point(&grid);
        let mut que = BinaryHeap::new();
        let mut visited = HashMap::new();
        let mut cheapest_cost = None;
        que.push((Reverse(0usize), (start_point, Direction::East)));
        while let Some((Reverse(cost), (position, direction))) = que.pop() {
            if let Some(cheapest_cost) = cheapest_cost{
                if cheapest_cost < cost {
                    continue;
                }
            }

            if visited.get(&(position, direction)).is_some() {
                continue;
            }
            match grid[position.0][position.1] {
                'E' => {
                    cheapest_cost = Some(cost);
                },
                '#' => {
                    continue
                },
                _ => {
                    que.push((Reverse(cost + 1), (next_position(&position, &direction)?, direction)));
                    que.push((Reverse(cost + 1000), (position, direction.rotate_clockwise())));
                    que.push((Reverse(cost + 1000), (position, direction.rotate_counterclockwise())));
                }
            }
            visited.insert((position, direction), cost);
        }
        let mut good_places = HashSet::new();
        let mut backtracking_points= visited
            .clone()
            .into_iter()
            .filter(|(key, value)| key.0 == end_point)
            .collect::<Vec<_>>();
        while !backtracking_points.is_empty() {
            let mut next_points = vec![];
            for ((point, direction), cost) in backtracking_points {
                good_places.insert(point);
                if point == start_point {
                    continue;
                }
                if visited.get(&(prev_position(&point, &direction)?, direction)) == Some(&(cost - 1)) {
                    next_points.push(((prev_position(&point, &direction)?, direction), cost - 1));
                }
                if visited.get(&(point, direction.rotate_clockwise())) == Some(&(cost - 1000)) {
                    next_points.push(((point, direction.rotate_clockwise()), cost - 1000))
                }
                if visited.get(&(point, direction.rotate_counterclockwise())) == Some(&(cost - 1000)) {
                    next_points.push(((point, direction.rotate_counterclockwise()), cost - 1000))
                }
            }
            backtracking_points = next_points;
        }
        print_grid_with_seats(&grid, &good_places);
        Ok(good_places.len())
    }
    //
    // assert_eq!(45, part2(BufReader::new(TEST_1.as_bytes()))?);
    assert_eq!(64, part2(BufReader::new(TEST_2.as_bytes()))?);
    //
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
