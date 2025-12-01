use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use image::{RgbImage, Rgb};
use adv_code_2024::*;

const DAY: &str = "14"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3
"; // TODO: Add the test input

type Point = (i32, i32);
type Vector = (i32, i32);
type RobotInfo = (Point, Vector);

fn read_point(s: &str) -> Result<Point> {
    let s  = s.split_at(2).1;
    let vec = s.split(",").map(|x| x.parse::<i32>().context("can not parse i32")).collect::<Result<Vec<_>>>()?;
    Ok((
        *vec.get(0).context("no x value")?,
        *vec.get(1).context("no y value")?
    ))
}

fn read_robot_info(s: &str) -> Result<RobotInfo>{
    let vec= s.split(" ").map(|x| read_point(x)).collect::<Result<Vec<_>>>()?;
    Ok((
        *vec.get(0).context("no robot position")?,
        *vec.get(1).context("no robot velocity")?
    ))
}

fn read_data<R: BufRead>(reader: R) -> impl Iterator<Item=Result<RobotInfo>> {
    reader.lines().flatten().map(|x| read_robot_info(x.as_str())).into_iter()
}

fn draw_white_background(img_buf: &mut RgbImage) -> Result<()> {
    for x in 0..img_buf.width() {
        for y in 0..img_buf.height() {
            img_buf.put_pixel(x, y, Rgb([255, 255, 255]));
        }
    }
    Ok(())
}

fn draw_points(img_buf: &mut RgbImage, points: &Vec<Point>) -> Result<()> {
    for point in points {
        img_buf.put_pixel(point.0 as u32, point.1 as u32, Rgb([6, 64, 43]));
    }
    Ok(())
}

fn draw_image(points: &Vec<Point>, step: usize, width: usize, height: usize) -> Result<()> {
    let mut img_buf = RgbImage::new(width as u32, height as u32);
    draw_white_background(&mut img_buf)?;
    draw_points(&mut img_buf, points)?;
    img_buf.save(format!("input/{DAY}/{step}.png")).context("Can not safe image")
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R, width: i32, height: i32) -> Result<usize> {
        // TODO: Solve Part 1 of the puzzle
        let mut q1 = 0;
        let mut q2 = 0;
        let mut q3 = 0;
        let mut q4 = 0;

        for robot_info in read_data(reader) {
            let (point, velocity) = robot_info?;
            let result = (
                (((point.0 + velocity.0 * 100) % width) + width) % width - width / 2,
                (((point.1 + velocity.1 * 100) % height) + height) % height - height / 2,
            );
            match result {
                (x, y) if x < 0 && y < 0 => {
                    q1 += 1;
                },
                (x, y) if x > 0 && y < 0 => {
                    q2 += 1;
                },
                (x, y) if x < 0 && y > 0 => {
                    q3 += 1;
                },
                (x, y) if x > 0 && y > 0 => {
                    q4 += 1;
                },
                _ => {},
            }
        }
        println!("{q1}, {q2}, {q3}, {q4}");
        Ok(q1 * q2 * q3 * q4)
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(12, part1(BufReader::new(TEST.as_bytes()), 11, 7)?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file, 101, 103)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R, width: i32, height: i32) -> Result<usize> {
        let robots_info = read_data(reader).collect::<Result<Vec<_>>>()?;
        for i in 1000..10000 {
            let points =  robots_info.clone().into_iter().map(|x| {
                let (point, velocity) = x;
                (
                    ((((point.0 + velocity.0 * i) % width) + width) % width),
                    ((((point.1 + velocity.1 * i) % height) + height) % height),
                )
            }).collect::<Vec<_>>();
            draw_image(
                &points,
                i as usize,
                width as usize,
                height as usize,
            )?;
        }
        Ok(0)
    }

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file, 101, 103)?);
    println!("XO-XO-XO: {result}");
    //endregion

    Ok(())
}
