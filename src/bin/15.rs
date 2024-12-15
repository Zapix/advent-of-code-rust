use core::result::Result::Ok;
use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;
use crate::Cell2::Wall;

const DAY: &str = "15"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST_1: &str = "\
########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<
"; // TODO: Add the test input

const TEST_2: &str ="\
##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
";

const TEST_3: &str = "\
#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^
";

trait FromChar {
    fn from_char(ch: char) -> Vec<Self> where Self: Sized;
}

trait IsRobot {
    fn is_robot(&self) -> bool;
}

trait IsComputable {
    fn is_computable(&self) -> bool;
}

trait GetFree {
    fn get_free() -> Self where Self: Sized;
}

trait CommonCell:  FromChar + IsRobot + Display + Clone + Copy + IsComputable + GetFree {}

#[derive(Clone, Copy, Eq, PartialEq)]
enum Cell {
    Free,
    Wall,
    Box,
    Robot
}

impl CommonCell for Cell {}

impl FromChar for Cell {
    fn from_char(ch: char) -> Vec<Self> {
        match ch {
            '#' => vec![Self::Wall],
            '@' => vec![Self::Robot],
            'O' => vec![Self::Box],
            _ => vec![Self::Free],
        }
    }
}

impl IsRobot for Cell {
    fn is_robot(&self) -> bool {
        *self == Cell::Robot
    }
}

impl IsComputable for Cell {
    fn is_computable(&self) -> bool {
        *self == Cell::Box
    }
}

impl GetFree for Cell {
    fn get_free() -> Self
    where
        Self: Sized
    {
        Self::Free
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Cell::Free => write!(f, "."),
            Cell::Wall => write!(f, "#"),
            Cell::Box => write!(f, "O"),
            Cell::Robot => write!(f, "@")
        }
    }
}


#[derive(Clone, Copy, Eq, PartialEq)]
enum Cell2 {
    Free,
    Wall,
    LeftBox,
    RightBox,
    Robot,
}

impl CommonCell for Cell2 {}

impl FromChar for Cell2 {
    fn from_char(ch: char) -> Vec<Self> {
        match ch {
            '#' => vec![Self::Wall, Self::Wall],
            '@' => vec![Self::Robot, Self::Free],
            'O' => vec![Self::LeftBox, Self::RightBox],
            _ => vec![Self::Free, Self::Free],
        }
    }
}

impl IsRobot for Cell2 {
    fn is_robot(&self) -> bool {
        *self == Cell2::Robot
    }
}

impl IsComputable for Cell2 {
    fn is_computable(&self) -> bool {
        *self == Cell2::LeftBox
    }
}

impl GetFree for Cell2 {
    fn get_free() -> Self
    where
        Self: Sized
    {
       Cell2::Free
    }
}

impl Display for Cell2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Cell2::LeftBox => write!(f, "["),
            Cell2::RightBox => write!(f, "]"),
            Cell2::Free => write!(f, "."),
            Cell2::Wall => write!(f, "#"),
            Cell2::Robot => write!(f, "@"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Move {
    Left,
    Right,
    Up,
    Down
}

impl Move {
    fn try_from(ch: char) -> Result<Self> {
        match ch {
            '<' => Ok(Move::Left),
            '>' => Ok(Move::Right),
            '^' => Ok(Move::Up),
            'v' => Ok(Move::Down),
            _ => Err(anyhow!("Unexpected symbol")),
        }
    }

    fn delta(&self) -> (i32, i32) {
        match self {
            Move::Left => (0, -1),
            Move::Right => (0, 1),
            Move::Up => (-1, 0),
            Move::Down => (1, 0)
        }
    }
}

impl Display for Move {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Move::Left => write!(f, "Left"),
            Move::Right => write!(f, "Right"),
            Move::Up => write!(f, "Up"),
            Move::Down => write!(f, "Down")

        }
    }
}

struct Grid<T: CommonCell> {
    data: Vec<Vec<T>>,
}

impl<T: CommonCell> Grid<T> {
    fn from(data: &Vec<String>) -> Self {
        let data = data
            .iter()
            .map(|x| x.chars().map(|x| T::from_char(x)).flatten().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        Self {
            data,
        }
    }

    fn n(&self) -> usize {
        self.data.len()
    }

    fn m(&self) -> usize {
        self.data[0].len()
    }

    fn robot(&self) -> Option<(usize, usize)> {
        let mut robot = None;
        for i in 0.. self.data.len() {
            for j in 0..self.data[0].len() {
                if self.data[i][j].is_robot() {
                    robot = Some((i, j));
                    break;
                }
            }
            if robot.is_some() {
                break;
            }
        }
        robot
    }

    fn get(&self, x: usize, y: usize) -> Result<T> {
        let row = self.data.get(x).context("No such row")?;
        let value = row.get(y).context("No such column")?;
        Ok(*value)
    }

    fn move_cell(&mut self, coord: (usize, usize), movement: &Move) -> Result<(usize, usize)> {
        let new_pos = (
            usize::try_from(coord.0 as i32 + movement.delta().0).context("Can't convert")?,
            usize::try_from( coord.1 as i32 + movement.delta().1).context("Can't convert")?
        );
        self.data[new_pos.0][new_pos.1] = self.data[coord.0][coord.1];
        self.data[coord.0][coord.1] = T::get_free();
        Ok(new_pos)
    }
}

impl<T: CommonCell> Display for Grid<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.n() {
            for j in 0..self.m() {
                write!(f, "{}", self.data[i][j])?
            }
            write!(f, "\n")?
        }
        std::fmt::Result::Ok(())
    }
}

fn get_movements(raw_data: &Vec<String>) -> Result<Vec<Move>>  {
    raw_data
        .iter()
        .map(
            |x| x.chars().map(Move::try_from).collect::<Vec<_>>()
        )
        .flatten()
        .collect::<Result<Vec<_>>>()
}

fn move_robot(robot: (usize, usize), movement: Move, grid: &mut Grid<Cell>) -> Result<(usize, usize)> {
    let mut stack = vec![];
    let mut pos = robot;
    while grid.get(pos.0, pos.1)? == Cell::Robot || grid.get(pos.0, pos.1)? == Cell::Box {
        stack.push(pos);
        pos = (
            usize::try_from(pos.0 as i32 + movement.delta().0).context("Can't convert")?,
            usize::try_from(pos.1 as i32 + movement.delta().1).context("Can't convert")?
        );
    }

    match grid.get(pos.0, pos.1)? {
        Cell::Wall => Ok(robot),
        Cell::Free => {
            let mut pos = robot;
            while !stack.is_empty() {
                pos = grid.move_cell(stack.pop().unwrap(), &movement)?;
            }
            Ok(pos)
        },
        _ => Err(anyhow!("Unexpected behavior"))
    }
}

fn can_move(pos: (usize, usize), movement: &Move, grid: &Grid<Cell2>) -> Result<bool> {
    let cell = grid.get(pos.0, pos.1)?;
    if cell == Cell2::Wall {
        return Ok(false)
    }
    if cell == Cell2::Free {
        return Ok(false)
    }
    let next_pos = (
        usize::try_from(pos.0 as i32 + movement.delta().0).context("Can't convert")?,
        usize::try_from(pos.1 as i32 + movement.delta().1).context("Can't convert")?
    );
    match movement {
        Move::Left | Move::Right => {
            let cell = grid.get(next_pos.0, next_pos.1)?;
            Ok(cell != Cell2::Wall)
        },
        Move::Up | Move::Down => {
            match grid.get(pos.0, pos.1)? {
                Cell2::Robot => {
                    let cell = grid.get(next_pos.0, next_pos.1)?;
                    Ok(cell != Cell2::Wall)
                },
                Cell2::LeftBox => {
                    let cell = grid.get(next_pos.0, next_pos.1)?;
                    let right_cell = grid.get(next_pos.0, next_pos.1 + 1)?;
                    Ok(cell != Cell2::Wall && right_cell == Cell2::Free)
                },
                Cell2::RightBox => {
                    let cell = grid.get(next_pos.0, next_pos.1)?;
                    let left_cell = grid.get(next_pos.0, next_pos.1 - 1)?;
                    Ok(cell != Cell2::Wall && left_cell == Cell2::Free)
                },
                _ => Err(anyhow!("unexpected behavior"))
            }
        },
    }
}

fn compute_next_position(pos: &(usize, usize), movement: &Move) -> Result<(usize, usize)> {
    let delta = movement.delta();
    Ok((
        usize::try_from(pos.0 as i32 + delta.0).context("can not parse")?,
        usize::try_from(pos.1 as i32 + delta.1).context("can not parse")?
    ))
}

fn move_element(pos: (usize, usize), movement: &Move, grid: &mut Grid<Cell2>) -> Result<(usize, usize)> {
    match movement {
        Move::Left | Move::Right => {
            grid.move_cell(pos, movement)
        },
        _ => match grid.get(pos.0, pos.1)? {
            Cell2::Robot => {  grid.move_cell(pos, movement) },
            Cell2::LeftBox => {
                let right_cell = (pos.0, pos.1 + 1);
                grid.move_cell(right_cell, movement)?;
                grid.move_cell(pos, movement)
            }
            Cell2::RightBox => {
                let left_cell = (pos.0, pos.1 - 1);
                grid.move_cell(left_cell, movement)?;
                grid.move_cell(pos, movement)
            }
            _ => Err(anyhow!("unexpected behavior"))
        }
    }
}

fn is_free(positions: &Vec<(usize, usize)>, grid: &Grid<Cell2>) -> Result<bool> {
    if positions.is_empty() {
        return Ok(true);
    }
    let cells =
        positions
            .iter()
            .map(
                |x| grid.get(x.0, x.1)
            )
            .collect::<Result<Vec<_>>>()?;
    Ok(cells.iter().all(|x| *x == Cell2::Free))
}

fn has_wall(positions: &Vec<(usize, usize)>, grid: &Grid<Cell2>) -> Result<bool> {
    if positions.is_empty() {
        return Ok(false);
    }
    let cells =
        positions
            .iter()
            .map(
                |x| grid.get(x.0, x.1)
            )
            .collect::<Result<Vec<_>>>()?;
    Ok(cells.iter().any(|x| *x == Cell2::Wall))
}

fn get_new_positions(positions: &Vec<(usize, usize)>, movement: &Move, grid: &Grid<Cell2>) -> Result<Vec<(usize, usize)>>{
    match movement {
        Move::Left | Move::Right => positions.iter().map(|x| {
            compute_next_position(x, movement)
        })
            .collect::<Result<Vec<_>>>(),
        _ => {
            let mut next_positions = HashSet::new();
            for pos in positions {
                let next_pos = compute_next_position(pos, movement)?;
                match grid.get(next_pos.0, next_pos.1)? {
                    Cell2::Wall => {
                        next_positions.insert(next_pos);
                    },
                    Cell2::LeftBox => {
                        next_positions.insert((next_pos.0, next_pos.1 + 1));
                        next_positions.insert(next_pos);
                    },
                    Cell2::RightBox => {
                        next_positions.insert((next_pos.0, next_pos.1 - 1));
                        next_positions.insert(next_pos);
                    },
                    _ => {},
                }
            }
            Ok(next_positions.into_iter().collect::<Vec<(usize, usize)>>())
        }
    }
}

fn move_robot_2(robot: (usize, usize), movement: Move, grid: &mut Grid<Cell2>) -> Result<(usize, usize)> {
    let mut stack = vec![];
    let mut positions = vec![robot];
    while !is_free(&positions, &grid)? && !has_wall(&positions, &grid)? {
        stack.push(positions.clone());
        positions = get_new_positions(&positions, &movement, &grid)?;
    }
    let mut next_pos = robot;
    if is_free(&positions, &grid)? {
        while !stack.is_empty() {
            let positions = stack.pop().unwrap();
            for pos in positions {
                next_pos = grid.move_cell(pos, &movement)?;
            }
        }
    }
    Ok(next_pos)
}

fn compute_value<T: CommonCell>(grid: &Grid<T>) -> Result<usize> {
    let mut result = 0;
    for i in 0..grid.n() {
        for j in 0..grid.m() {
            if grid.get(i, j)?.is_computable() {
                result += i * 100 + j;
            }
        }
    }
    Ok(result)
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        // TODO: Solve Part 1 of the puzzle
        let mut reader= reader.lines().flatten().collect::<Vec<_>>();
        let raw_grid = reader.clone().into_iter().take_while(|x| !x.is_empty()).collect::<Vec<_>>();
        let mut grid: Grid<Cell> = Grid::from(&raw_grid);
        let mut robot = grid.robot().context("Robot not found")?;

        let raw_movements = reader.into_iter().skip_while(|x| !x.is_empty()).skip(1).collect::<Vec<_>>();
        for movement in get_movements(&raw_movements)? {
            robot = match move_robot(robot, movement, &mut grid) {
                Ok(pos) => pos,
                _ => robot
            };
        }

        compute_value(&grid)
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(2028, part1(BufReader::new(TEST_1.as_bytes()))?);
    assert_eq!(10092, part1(BufReader::new(TEST_2.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut reader= reader.lines().flatten().collect::<Vec<_>>();
        let raw_grid = reader.clone().into_iter().take_while(|x| !x.is_empty()).collect::<Vec<_>>();
        let mut grid: Grid<Cell2> = Grid::from(&raw_grid);
        let mut robot = grid.robot().context("Robot not found")?;
        println!("{}", grid);

        let raw_movements = reader.into_iter().skip_while(|x| !x.is_empty()).skip(1).collect::<Vec<_>>();
        for movement in get_movements(&raw_movements)? {
            robot = match move_robot_2(robot, movement, &mut grid) {
                Ok(pos) => pos,
                _ => robot
            };
        }
        println!("{}", grid);
        compute_value(&grid)
    }

    assert_eq!(9021, part2(BufReader::new(TEST_2.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
