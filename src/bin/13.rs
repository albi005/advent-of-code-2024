use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "13";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        Ok(reader
            .lines()
            .flatten()
            .chunks(4)
            .into_iter()
            .filter_map(|lines| {
                let lines: Vec<_> = lines.collect();
                let ax: usize = lines[0][12..=13].parse().unwrap();
                let ay: usize = lines[0][18..=19].parse().unwrap();
                let bx: usize = lines[1][12..=13].parse().unwrap();
                let by: usize = lines[1][18..=19].parse().unwrap();
                let comma = lines[2].find(',').unwrap();
                let x: usize = lines[2][9..comma].parse().unwrap();
                let y: usize = lines[2][comma + 4..].parse().unwrap();

                (0..=100)
                    .filter_map(|a: usize| {
                        let b = x.checked_sub(a * ax)?.checked_div(bx)?; // welp, checked_div doesn't check divisibility
                        if (a * ay + b * by) != y {
                            return None;
                        }
                        Some(3 * a + b)
                    })
                    .min()
            })
            .sum())
    }

    assert_eq!(480, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        Ok(reader
            .lines()
            .flatten()
            .chunks(4)
            .into_iter()
            .filter_map(|lines| {
                let lines: Vec<_> = lines.collect();
                let ax: usize = lines[0][12..=13].parse().unwrap();
                let ay: usize = lines[0][18..=19].parse().unwrap();
                let bx: usize = lines[1][12..=13].parse().unwrap();
                let by: usize = lines[1][18..=19].parse().unwrap();
                let comma = lines[2].find(',').unwrap();
                let x: usize = lines[2][9..comma].parse::<usize>().unwrap() + 10000000000000;
                let y: usize = lines[2][comma + 4..].parse::<usize>().unwrap() + 10000000000000;

                let det = (ax * by) as isize - (ay * bx) as isize;
                if det == 0 {
                    panic!();
                }

                // ax * a + bx * b = x
                // ay * a + by * b = y
                //
                // a = (x - bx * b) / ax
                // ay * ((x - bx * b) / ax) + by * b = y
                // ay * ((x - bx * b) / ax) - y = -by * b
                // (ay * x - ay * bx * b) / ax - y = -by * b
                // ay * x / ax - ay * bx * b / ax - y = -by * b
                // ay * x / ax - y = -by * b + ay * bx * b / ax
                // ay * x / ax - y = b * (-by + ay * bx / ax)
                // b = (ay * x / ax - y) / (-by + ay * bx / ax)

                // let b = (ay * x)
                //     .checked_div(ax)?
                //     .checked_sub(y)?
                //     .checked_div((ay * bx).checked_div(ax)?.checked_sub(by)?)?;

                // let b = ((ay as f64 * x as f64 / ax as f64 - y as f64)
                //     / (-(by as f64) + ay as f64 * bx as f64 / ax as f64))
                //     as usize;

                // let b = {
                //     let ax = ax as f64;
                //     let ay = ay as f64;
                //     let bx = bx as f64;
                //     let by = by as f64;
                //     let x = x as f64;
                //     let y = y as f64;
                //     let b = (ay * x / ax - y) / (-by + ay * bx / ax);
                //     b as usize
                // };
                // let a = x.checked_sub(bx * b)?.checked_div(ax)?;

                trait DivisibleDiv {
                    fn divisible_div(self, other: Self) -> Option<Self>
                    where
                        Self: Sized;
                }

                impl DivisibleDiv for isize {
                    fn divisible_div(self, other: Self) -> Option<Self> {
                        if self % other == 0 {
                            Some(self / other)
                        } else {
                            None
                        }
                    }
                }

                let (a, b) = {
                    let ax = ax as isize;
                    let ay = ay as isize;
                    let bx = bx as isize;
                    let by = by as isize;
                    let x = x as isize;
                    let y = y as isize;
                    // Cramer's rule
                    let a = (x * by - bx * y).divisible_div(det)?;
                    let b = (ax * y - x * ay).divisible_div(det)?;
                    (a as usize, b as usize)
                };

                assert_eq!(a * ax + b * bx, x);
                assert_eq!(a * ay + b * by, y);

                Some(3 * a + b)
            })
            .sum())
    }

    // :|
    // assert_eq!(480, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
