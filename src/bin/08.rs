use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::collections::{BTreeSet, HashMap};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

const DAY: &str = "08"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689
"; // TODO: Add the test input

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct Point3D {
    x: usize,
    y: usize,
    z: usize,
}

impl FromStr for Point3D {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let points = s
            .split(',')
            .map(|x| x.parse::<usize>().map_err(|x| Error::msg(x.to_string())))
            .collect::<Result<Vec<usize>>>()?;

        Ok(Point3D {
            x: *points.get(0).unwrap(),
            y: *points.get(1).unwrap(),
            z: *points.get(2).unwrap(),
        })
    }
}

fn get_square_distance(a: Point3D, b: Point3D) -> usize {
    (a.x.abs_diff(b.x)).pow(2) + (a.y.abs_diff(b.y)).pow(2) + (a.z.abs_diff(b.z)).pow(2)
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R, cabels: usize) -> Result<usize> {
        // TODO: Solve Part 1 of the puzzle
        let points = reader
            .lines()
            .flatten()
            .map(|x| x.parse::<Point3D>())
            .collect::<Result<Vec<_>>>()?;

        let mut distances = BTreeSet::new();
        for i in 0..points.len() {
            for j in (i + 1)..points.len() {
                distances.insert((get_square_distance(points[i], points[j]), i, j));
            }
        }
        let mut distances = distances.into_iter();
        let mut point_to_joint = (0..points.len()).fold(HashMap::new(), |mut acc, item| {
            acc.insert(item, item);
            acc
        });
        let mut joint_to_points = (0..points.len()).fold(HashMap::new(), |mut acc, item| {
            acc.insert(item, vec![item]);
            acc
        });
        let mut used_cabels = 0;
        while used_cabels < cabels {
            used_cabels += 1;
            let (_, i, j) = distances.next().unwrap();
            if point_to_joint.get(&i) == point_to_joint.get(&j) {
                continue;
            }
            let prev_joint = *point_to_joint.get(&j).unwrap();
            let mut prev_joint_points = joint_to_points.remove(&prev_joint).unwrap();
            let main_joint = *point_to_joint.get(&i).unwrap();
            let main_joint_points = joint_to_points.get_mut(&main_joint).unwrap();
            while let Some(point_idx) = prev_joint_points.pop() {
                point_to_joint.insert(point_idx, main_joint);
                main_joint_points.push(point_idx);
            }
        }
        let mut joints = joint_to_points
            .values()
            .map(|x| x.len())
            .collect::<Vec<_>>();
        joints.sort();
        joints.reverse();
        let answer = joints.iter().take(3).fold(1, |acc, item| acc * item);

        Ok(answer)
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(40, part1(BufReader::new(TEST.as_bytes()), 10)?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file, 1000)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let points = reader
            .lines()
            .flatten()
            .map(|x| x.parse::<Point3D>())
            .collect::<Result<Vec<_>>>()?;

        let mut distances = BTreeSet::new();
        for i in 0..points.len() {
            for j in (i + 1)..points.len() {
                distances.insert((get_square_distance(points[i], points[j]), i, j));
            }
        }
        let mut distances = distances.into_iter();
        let mut point_to_joint = (0..points.len()).fold(HashMap::new(), |mut acc, item| {
            acc.insert(item, item);
            acc
        });
        let mut joint_to_points = (0..points.len()).fold(HashMap::new(), |mut acc, item| {
            acc.insert(item, vec![item]);
            acc
        });
        let mut answer = 0;
        while joint_to_points.len() != 1 {
            let (_, i, j) = distances.next().unwrap();
            if point_to_joint.get(&i) == point_to_joint.get(&j) {
                continue;
            }
            let prev_joint = *point_to_joint.get(&j).unwrap();
            let mut prev_joint_points = joint_to_points.remove(&prev_joint).unwrap();
            let main_joint = *point_to_joint.get(&i).unwrap();
            let main_joint_points = joint_to_points.get_mut(&main_joint).unwrap();
            while let Some(point_idx) = prev_joint_points.pop() {
                point_to_joint.insert(point_idx, main_joint);
                main_joint_points.push(point_idx);
            }
            answer = points[i].x * points[j].x;
        }

        Ok(answer)
    }

    assert_eq!(25272, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
