use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use adv_code_2024::*;

const DAY: &str = "01";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
3   4
4   3
2   5
1   3
3   9
3   3
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let lines: Vec<_> = reader.lines().flatten().collect();
        let mut a: Vec<usize> = Vec::new();
        let mut b: Vec<usize> = Vec::new();
        for line in lines {
            let mut parts = line.split_whitespace();
            a.push(parts.next().unwrap().parse()?);
            b.push(parts.next().unwrap().parse()?);
        }
        a.sort();
        b.sort();
        Ok(
            a.iter().zip(b).map(|(a, b)| a.abs_diff(b)).sum()
        )
    }

    assert_eq!(11, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let lines: Vec<_> = reader.lines().flatten().collect();
        let mut left: Vec<usize> = Vec::new();
        let mut right: Vec<usize> = Vec::new();
        for line in lines {
            let mut parts = line.split_whitespace();
            left.push(parts.next().unwrap().parse()?);
            right.push(parts.next().unwrap().parse()?);
        }

        let counts = right.iter().counts();

        Ok(
            left.iter().map(|a| a * counts.get(a).unwrap_or(&0)).sum()
        )
    }

    assert_eq!(31, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
