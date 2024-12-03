use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;

const DAY: &str = "02";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let answer = reader
            .lines()
            .flatten()
            .filter(|line| {
                let parts: Vec<usize> = line.split_whitespace().map(|x| x.parse()).flatten().collect();
                let order = parts[0] < parts[1];
                for i in 0..(parts.len() - 1) {
                    let a = parts[i];
                    let b = parts[i + 1];
                    if a.abs_diff(b) < 1 { return false; }
                    if a.abs_diff(b) > 3 { return false; }
                    if order != (a < b) { return false; }
                }
                return true;
            })
            .count();
        Ok(answer)
    }

    assert_eq!(2, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let answer = reader
            .lines()
            .flatten()
            .filter(|line| {
                let og_parts: Vec<usize> = line.split_whitespace().map(|x| x.parse()).flatten().collect();
                return (0..=og_parts.len()).any(|ignored_index|{
                    let parts: Vec<usize> = og_parts.iter().enumerate().filter(|(i, _)| *i != ignored_index).map(|(_,x)| *x).collect();
                    dbg!(parts.len());
                    let order = parts[0] < parts[1];
                    for i in 0..(parts.len() - 1) {
                        let a = parts[i];
                        let b = parts[i + 1];
                        if a.abs_diff(b) < 1 { return false; }
                        if a.abs_diff(b) > 3 { return false; }
                        if order != (a < b) { return false; }
                    }
                    return true;
                });
            })
            .count();
        Ok(answer)
    }

    assert_eq!(4, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
