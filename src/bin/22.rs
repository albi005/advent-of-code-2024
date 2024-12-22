use std::collections::{HashMap, HashSet, VecDeque};
use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::{Mul, Rem};

const DAY: &str = "22";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
1
2
3
2024
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn mix(a: usize, b: usize) -> usize {
        a ^ b
    }
    assert_eq!(mix(42, 15), 37);

    fn prune(a: usize) -> usize {
        a % 16777216
    }
    fn prune2(a: usize) -> usize {
        a & 0b111111111111111111111111
    }
    assert_eq!(prune(100000000), 16113920);
    assert_eq!(prune(100000000), prune2(100000000));

    fn forward(secret_number: usize) -> usize {
        let secret_number = prune((64 * secret_number) ^ secret_number);
        let secret_number = prune((secret_number / 32) ^ secret_number);
        let secret_number = prune((secret_number * 2048) ^ secret_number);
        secret_number
    }
    assert_eq!(forward(123), 15887950);
    assert_eq!(forward(15887950), 16495136);
    assert_eq!(forward(16495136), 527345);
    fn forward2(secret_number: usize) -> usize {
        let secret_number = prune2((secret_number << 6) ^ secret_number);
        let secret_number = prune2((secret_number >> 5) ^ secret_number);
        let secret_number = prune2((secret_number << 11) ^ secret_number);
        secret_number
    }
    assert_eq!(forward2(123), forward(123));
    assert_eq!(forward2(15887950), forward(15887950));
    assert_eq!(forward2(16495136), forward(16495136));

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        Ok(reader
            .lines()
            .flatten()
            .map(|line| {
                let mut secret_number: usize = line.parse().unwrap();
                for _ in 0..2000 {
                    secret_number = forward(secret_number);
                }
                secret_number
            })
            .sum())
    }

    // assert_eq!(37327623, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");
    
    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut map: HashMap<(i8, i8, i8, i8), usize> = HashMap::new();
        reader
            .lines()
            .flatten()
            .for_each(|line| {
                let mut secret_number: usize = line.parse().unwrap();
                let mut queue: VecDeque<i8> = VecDeque::new();
                for _ in 0..3 {
                    let prev = secret_number;
                    secret_number = forward(secret_number);
                    let a = prev.rem(10) as i8;
                    let b = secret_number.rem(10) as i8;
                    queue.push_back(b - a);
                }
                let mut hits: HashSet<(i8, i8, i8, i8)> = HashSet::new();
                for _ in 3..2000 {
                    let prev = secret_number;
                    secret_number = forward(secret_number);
                    let prev = prev.rem(10) as i8;
                    let current = secret_number.rem(10) as i8;
                    let change = current - prev;
                    queue.push_back(change);
                    let key = (queue[0], queue[1], queue[2], queue[3]);
                    if !hits.contains(&key) {
                        hits.insert(key);
                        let entry = map.entry(key).or_insert(0);
                        *entry += current as usize;
                    }
                    queue.pop_front();
                }
            });
        Ok(*map.values().max().unwrap())
    }
    
    assert_eq!(23, part2(BufReader::new(TEST.as_bytes()))?);
    
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
