use std::collections::HashMap;
use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;

const DAY: &str = "19";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");
    
    fn is_possible(design: &str, towels: &Vec<&str>) -> bool {
        if design.len() == 0 {
            return true;
        }
        for towel in towels {
            if design.ends_with(towel) {
                if is_possible(&design[..design.len()-towel.len()], towels) {
                    return true;
                }
            }
        }
        false
    }

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut lines = reader.lines().flatten();
        let towels = lines.next().unwrap();
        let towels: Vec<_> = towels.split(", ").collect();
        lines.next();
        Ok(lines.filter(|design| is_possible(design, &towels)).count())
    }

    assert_eq!(6, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn get_possibilities<'a>(design: &'a str, towels: &Vec<&str>, memo: &mut HashMap<&'a str, usize>) -> usize {
        if design.len() == 0 {
            return 1;
        }
        if let Some(&res) = memo.get(design){
            return res;
        }
        let res = towels.iter().map(|towel|{
            if design.ends_with(towel) {
                get_possibilities(&design[..design.len()-towel.len()], towels, memo) 
            }
            else {
                0
            }
        }).sum();
        memo.insert(design, res);
        res
    }

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut lines = reader.lines().flatten();
        let towels = lines.next().unwrap();
        let towels: Vec<_> = towels.split(", ").collect();
        lines.next();
        let designs: Vec<_> = lines.collect();
        let mut memo: HashMap<&str, usize> = HashMap::new();
        Ok(designs.iter().map(|design| get_possibilities(&design, &towels, &mut memo)).sum())
    }
    
    assert_eq!(16, part2(BufReader::new(TEST.as_bytes()))?);
    
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
