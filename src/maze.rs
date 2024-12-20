use std::collections::HashSet;
use std::io::BufRead;

pub type Point = (usize, usize);

pub type Grid = Vec<Vec<char>>;

pub fn read_map<R: BufRead>(reader: R) -> Grid {
    reader
        .lines()
        .flatten()
        .map(|x| x.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>()
}
pub fn find_start_end_point(grid: &Grid) -> (Point, Point) {
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

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Direction {
    East,
    South,
    West,
    North,
}

impl Direction {
    pub fn rotate_clockwise(&self) -> Direction {
        match self {
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West=> Direction::North,
            Direction::North => Direction::East,
        }
    }

    pub fn rotate_counterclockwise(&self) -> Direction {
        match self {
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West=> Direction::South,
            Direction::North => Direction::West,
        }
    }

    pub fn delta(&self) -> (i32, i32) {
        match self {
            Direction::East => (0, 1),
            Direction::South => (1, 0),
            Direction::West=> (0, -1),
            Direction::North => (-1, 0),
        }
    }
}
pub fn next_position(point: &Point, direction: &Direction) -> anyhow::Result<Point> {
    let delta = direction.delta();
    anyhow::Ok((
        usize::try_from(point.0 as i32 + delta.0)?,
        usize::try_from(point.1 as i32 + delta.1)?,
    ))
}

pub fn prev_position(point: &Point, direction: &Direction) -> anyhow::Result<Point> {
    let delta = direction.delta();
    anyhow::Ok((
        usize::try_from(point.0 as i32 - delta.0)?,
        usize::try_from(point.1 as i32 - delta.1)?,
    ))
}

pub fn print_grid(grid: &Grid) {
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            print!("{}", grid[i][j]);
        }
        print!("\n");
    }
}
