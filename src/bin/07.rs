use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use std::arch::x86_64::__m128;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "07";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn has_variation(prod: usize, target: usize, remaining: &[usize]) -> bool {
        if remaining.len() == 0 {
            return prod == target;
        }
        has_variation(prod + remaining[0], target, &remaining[1..])
        || has_variation(prod * remaining[0], target, &remaining[1..])
    }

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        Ok(reader
            .lines()
            .flatten()
            .map(|line| {
                let vec: Vec<_> = line.split_ascii_whitespace().collect();
                let target: usize = vec[0][..vec[0].len() - 1].parse().unwrap();
                let members: Vec<_> = vec
                    .iter()
                    .skip(1)
                    .map(|x| x.parse::<usize>())
                    .flatten()
                    .collect();
                if has_variation(members[0], target, &members[1..]) {
                    target
                } else {
                    0
                }
            })
            .sum())
    }

    assert_eq!(3749, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn has_variation2(prod: usize, target: usize, remaining: &[usize]) -> bool {
        if remaining.len() == 0 {
            return prod == target;
        }
        has_variation2(prod + remaining[0], target, &remaining[1..])
            || has_variation2(prod * remaining[0], target, &remaining[1..])
            || has_variation2(concat(prod, remaining[0]), target, &remaining[1..])
    }
    
    fn concat(a: usize, b: usize) -> usize {
        (a.to_string() + &b.to_string()).parse().unwrap()
    }

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        Ok(reader
            .lines()
            .flatten()
            .map(|line| {
                let vec: Vec<_> = line.split_ascii_whitespace().collect();
                let target: usize = vec[0][..vec[0].len() - 1].parse().unwrap();
                let members: Vec<_> = vec
                    .iter()
                    .skip(1)
                    .map(|x| x.parse::<usize>())
                    .flatten()
                    .collect();
                if has_variation2(members[0], target, &members[1..]) {
                    target
                } else {
                    0
                }
            })
            .sum())
    }
    
    assert_eq!(11387, part2(BufReader::new(TEST.as_bytes()))?);
    
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
