use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;

const DAY: &str = "03";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn check_next(s: &mut &str) -> Option<usize> {
        let start = s.find("mul(")?;
        *s = &s[start + 4..];
        let comma = s.find(',')?;
        let a: usize = s[..comma].parse().ok()?;
        *s = &s[comma + 1..];
        let end = s.find(')')?;
        let b: usize = s[..end].parse().ok()?;
        *s = &s[end + 1..];

        Some(a * b)
    }

    fn part1<R: BufRead>(mut reader: R) -> Result<usize> {
        let mut s = String::new();
        reader.read_to_string(&mut s)?;

        let mut slice = s.as_str();

        let mut sum = 0;

        loop {
            if let Some(prod) = check_next(&mut slice) {
                sum += prod;
            }
            if !slice.contains("mul(") { break; }
        }

        Ok(sum)
    }

    assert_eq!(161, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn parse_mul(s: &str) -> Option<usize> {
        let mut s = s;
        s = &s[4..];
        let comma = s.find(',')?;
        let a: usize = s[..comma].parse().ok()?;
        s = &s[comma + 1..];
        let end = s.find(')')?;
        let b: usize = s[..end].parse().ok()?;
        s = &s[end + 1..];

        Some(a * b)
    }

    fn part2<R: BufRead>(mut reader: R) -> Result<usize> {
        let mut s = String::new();
        reader.read_to_string(&mut s)?;

        let mut sum = 0;
        let mut _do = true;

        for i in 0..s.len() {
            let slice = &s[i..];
            if slice.starts_with("do()") { _do = true; }
            if slice.starts_with("don't()") { _do = false; }
            if _do && slice.starts_with("mul(") {
                match parse_mul(&slice) {
                    None => {}
                    Some(prod) => {
                        sum += prod;
                    }
                }
            }
        }

        Ok(sum)
    }

    assert_eq!(48, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
