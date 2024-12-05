use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use adv_code_2024::*;

const DAY: &str = "05";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(mut reader: R) -> Result<usize> {
        let mut s = String::new();
        reader.read_to_string(&mut s)?;

        let split: Vec<_> = s.split("\n\n").collect();
        let rules = split[0].lines()
            .map(|l| {
                let split: Vec<_> = l.split('|').collect();
                let a: usize = split[0].parse().unwrap();
                let b: usize = split[1].parse().unwrap();
                (a, b)
            })
            .collect::<Vec<_>>();
        let updates = split[1].lines()
            .map(|l| l.split(',').map(|n| n.parse::<usize>().unwrap()).collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let answer = updates.iter()
            .filter(|update| {
                rules.iter().all(|(a, b)| {
                    let a = update.iter().position(|&n| n == *a);
                    let b = update.iter().position(|&n| n == *b);
                    if let (Some(a), Some(b)) = (a, b) {
                        a < b
                    } else {
                        true
                    }
                })
            })
            .map(|page| page[page.len()/2])
            .sum();

        Ok(answer)
    }

    assert_eq!(143, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");
    
    fn part2<R: BufRead>(mut reader: R) -> Result<usize> {
        let mut s = String::new();
        reader.read_to_string(&mut s)?;

        let split: Vec<_> = s.split("\n\n").collect();
        let rules = split[0].lines()
            .map(|l| {
                let split: Vec<_> = l.split('|').collect();
                let a: usize = split[0].parse().unwrap();
                let b: usize = split[1].parse().unwrap();
                (a, b)
            })
            .collect::<Vec<_>>();
        let updates = split[1].lines()
            .map(|l| l.split(',').map(|n| n.parse::<usize>().unwrap()).collect::<Vec<_>>())
            .collect::<Vec<_>>();
        
        let mut less_than: HashMap<usize, HashSet<usize>> = HashMap::new();
        for (a, b) in rules.iter() {
            less_than.entry(*a).or_default().insert(*b);
        }
        let mut more_than: HashMap<usize, HashSet<usize>> = HashMap::new();
        for (a, b) in rules.iter() {
            more_than.entry(*b).or_default().insert(*a);
        }

        let answer = updates.iter()
            .filter(|update| {
                !rules.iter().all(|(a, b)| {
                    let a = update.iter().position(|&n| n == *a);
                    let b = update.iter().position(|&n| n == *b);
                    if let (Some(a), Some(b)) = (a, b) {
                        a < b
                    } else {
                        true
                    }
                })
            })
            .map(|page| {
                let page: Vec<_> = page.iter().sorted_by(|a, b| {
                    let a_less_than = less_than.get(a);
                    let b_less_than = less_than.get(b);
                    let a_more_than = more_than.get(a);
                    let b_more_than = more_than.get(b);
                    if let Some(a_less_than) = a_less_than {
                        if a_less_than.contains(b) {
                            return Ordering::Less;
                        }
                    }
                    if let Some(b_less_than) = b_less_than {
                        if b_less_than.contains(a) {
                            return Ordering::Greater;
                        }
                    }
                    if let Some(a_more_than) = a_more_than {
                        if a_more_than.contains(b) {
                            return Ordering::Greater;
                        }
                    }
                    if let Some(b_more_than) = b_more_than {
                        if b_more_than.contains(a) {
                            return Ordering::Less;
                        }
                    }
                    Ordering::Equal
                }).collect();
                page[page.len() / 2]
            })
            .sum();

        Ok(answer)
    }
    
    assert_eq!(123, part2(BufReader::new(TEST.as_bytes()))?);
    
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
