use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::sync::{Arc, Mutex};
use std::thread;

const DAY: &str = "12"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2
"; // TODO: Add the test input

const PREV: &str = "\
";

#[derive(Eq, PartialEq, Clone, Copy, Hash)]
struct BitMap {
    size: usize,
    bitmap: [[bool; 3]; 3],
}

impl BitMap {
    fn new(values: &Vec<String>) -> Self {
        let mut bitmap = [[false; 3]; 3];
        let mut size = 0;
        for (i, line) in values.iter().enumerate() {
            for (j, ch) in line.chars().enumerate() {
                if ch == '#' {
                    bitmap[i][j] = true;
                    size += 1;
                }
            }
        }
        Self { bitmap, size }
    }

    fn rotate(&self) -> Self {
        let mut new_bitmap = [[false; 3]; 3];
        for i in 0..3 {
            for j in 0..3 {
                new_bitmap[j][3 - i - 1] = self.bitmap[i][j];
            }
        }

        Self {
            size: self.size,
            bitmap: new_bitmap,
        }
    }

    fn flip_x(&self) -> Self {
        let mut new_bitmap = [[false; 3]; 3];
        for i in 0..3 {
            for j in 0..3 {
                new_bitmap[i][3 - j - 1] = self.bitmap[i][j];
            }
        }
        Self {
            size: self.size,
            bitmap: new_bitmap,
        }
    }

    fn flip_y(&self) -> Self {
        let mut new_bitmap = [[false; 3]; 3];
        for i in 0..3 {
            for j in 0..3 {
                new_bitmap[3 - i - 1][j] = self.bitmap[i][j];
            }
        }
        Self {
            size: self.size,
            bitmap: new_bitmap,
        }
    }
}

impl Display for BitMap {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for line in self.bitmap.iter() {
            for cell in line {
                let ch = match cell {
                    true => '#',
                    false => '.',
                };
                write!(f, "{}", ch)?;
            }
            writeln!(f, "")?;
        }
        std::fmt::Result::Ok(())
    }
}

#[derive(Eq, PartialEq, Clone, Hash)]
struct Grid {
    grid: Vec<Vec<bool>>,
}

impl Grid {
    fn new(rows: usize, cols: usize) -> Self {
        let grid = vec![vec![false; cols]; rows];
        Self { grid }
    }

    fn height(&self) -> usize {
        self.grid.len()
    }

    fn width(&self) -> usize {
        self.grid[0].len()
    }

    fn insert_bitmap(&mut self, x: usize, y: usize, bitmap: &BitMap) -> Result<()> {
        if x + 3 > self.grid.len() {
            return Err(Error::msg("index outside bitmap"));
        }
        if y + 3 > self.grid[0].len() {
            return Err(Error::msg("index outside bitmap"));
        }
        for i in 0..3 {
            for j in 0..3 {
                let nx = x + i;
                let ny = y + j;
                if !bitmap.bitmap[i][j] {
                    continue;
                }
                if let Some(row) = self.grid.get(nx) {
                    if let Some(val) = row.get(ny) {
                        if *val {
                            return Err(Error::msg(format!("Cell {} {} already used", nx, ny)));
                        }
                    } else {
                        return Err(Error::msg(format!("No cell {} {}", nx, ny)));
                    }
                } else {
                    return Err(Error::msg(format!("No row {}", nx)));
                }
            }
        }
        for i in 0..3 {
            for j in 0..3 {
                let nx = x + i;
                let ny = y + j;
                if !bitmap.bitmap[i][j] {
                    continue;
                }
                self.grid[nx][ny] = true;
            }
        }
        Ok(())
    }

    fn remove_bitmap(&mut self, x: usize, y: usize, bitmap: &BitMap) -> Result<()> {
        if x + 3 > self.grid.len() {
            return Err(Error::msg("index outside bitmap"));
        }
        if y + 3 > self.grid[0].len() {
            return Err(Error::msg("index outside bitmap"));
        }
        for i in 0..3 {
            for j in 0..3 {
                let nx = x + i;
                let ny = y + j;
                if !bitmap.bitmap[i][j] {
                    continue;
                }
                self.grid[nx][ny] = false;
            }
        }
        Ok(())
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for line in self.grid.iter() {
            for cell in line.iter() {
                write!(f, "{}", if *cell { '#' } else { '.' })?;
            }
            writeln!(f, "")?;
        }
        std::fmt::Result::Ok(())
    }
}

fn fit_region(
    cnt: usize,
    gifts_count: Vec<usize>,
    grid: &mut Grid,
    gifts: &Vec<HashSet<BitMap>>,
) -> bool {
    if let Some((idx, _)) = gifts_count.iter().enumerate().find(|(_, &cnt)| cnt > 0) {
        let mut gifts_count = gifts_count.clone();
        gifts_count[idx] -= 1;
        for bitmap in gifts[idx].iter() {
            for i in 0..grid.height() {
                for j in 0..grid.width() {
                    if grid.insert_bitmap(i, j, bitmap).is_ok() {
                        if fit_region(cnt + 1, gifts_count.clone(), grid, gifts) {
                            return true;
                        }
                        grid.remove_bitmap(i, j, bitmap);
                    }
                }
            }
        }
        false
    } else {
        true
    }
}

fn can_fit_region(
    width: usize,
    height: usize,
    gifts_count: Vec<usize>,
    gifts: &Vec<HashSet<BitMap>>,
) -> bool {
    gifts
        .iter()
        .map(|x| x.iter().next().unwrap())
        .map(|x| x.size)
        .zip(gifts_count.iter())
        .map(|(size, &count)| size * count)
        .fold(0usize, |acc, item| acc + item)
        <= width * height
}

fn can_fit_region_1(
    width: usize,
    height: usize,
    gifts_count: Vec<usize>,
    gifts: &Vec<HashSet<BitMap>>,
) -> bool {
    let mut grid = Grid::new(height, width);
    if fit_region(0, gifts_count.clone(), &mut grid, gifts) {
        println!("Fits region {}x{} {:?}", width, height, gifts_count);
        println!("{}", grid);
        true
    } else {
        println!("Does not fit region {}x{} {:?}", width, height, gifts_count);
        false
    }
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        // TODO: Solve Part 1 of the puzzle
        let mut gifts = vec![];
        let mut buf = vec![];
        let mut regions = vec![];
        for line in reader.lines().flatten() {
            if line.contains(':') && line.contains('x') {
                let parts = line.split(':').collect::<Vec<_>>();
                println!("parts: {:?}", parts);
                let size = parts[0]
                    .split('x')
                    .map(|x| x.parse::<usize>().map_err(|x| Error::msg(x.to_string())))
                    .collect::<Result<Vec<usize>>>()?;
                let gifts_count = parts[1]
                    .trim()
                    .split(' ')
                    .map(|x| x.parse::<usize>().map_err(|x| Error::msg(x.to_string())))
                    .collect::<Result<Vec<usize>>>()?;

                println!("Grid size {:?} gifts_count {:?}", size, gifts_count);
                regions.push((size, gifts_count));
                continue;
            }
            if line.contains(':') && !line.contains('x') {
                buf.clear();
                continue;
            }
            if line.trim().is_empty() {
                let mut bitmaps = HashSet::new();
                let mut bitmap = BitMap::new(&buf);
                for _ in 0..4 {
                    bitmap = bitmap.rotate();
                    bitmaps.insert(bitmap.clone());
                }

                bitmap = bitmap.flip_x();
                for _ in 0..4 {
                    bitmap = bitmap.rotate();
                    bitmaps.insert(bitmap.clone());
                }

                bitmap = bitmap.flip_x().flip_y();
                for _ in 0..4 {
                    bitmap = bitmap.rotate();
                    bitmaps.insert(bitmap.clone());
                }

                println!("Bitmap variants {}", bitmaps.len());
                for b in bitmaps.iter() {
                    println!("{}", b);
                }

                gifts.push(bitmaps);

                continue;
            }
            buf.push(line);
        }
        let answer = Arc::new(Mutex::new(0usize));
        let gifts = Arc::new(gifts);
        let mut handles = vec![];
        for (size, gifts_count) in regions {
            let gifts = Arc::clone(&gifts);
            let answer = Arc::clone(&answer);
            let handle = thread::spawn(move || {
                if can_fit_region(size[0], size[1], gifts_count, gifts.as_ref()) {
                    let mut num = answer.lock().unwrap();
                    *num += 1;
                }
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        let answer = answer.lock().unwrap();
        Ok(*answer)
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(3, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    // println!("\n=== Part 2 ===");
    //
    // fn part2<R: BufRead>(reader: R) -> Result<usize> {
    //     Ok(0)
    // }
    //
    // assert_eq!(0, part2(BufReader::new(TEST.as_bytes()))?);
    //
    // let input_file = BufReader::new(File::open(INPUT_FILE)?);
    // let result = time_snippet!(part2(input_file)?);
    // println!("Result = {}", result);
    //endregion

    Ok(())
}
