use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use log::log;
use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ptr::write;

const DAY: &str = "21";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
029A
980A
179A
456A
379A
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    struct Keypad {
        width: usize,
        height: usize,
        illegal: (usize, usize),
    }

    // given a directional or numeric keypad and two keys on it, return coordinates of keys to be pressed on a directional keypad
    fn get_moves(
        from: (usize, usize),
        to: (usize, usize),
        keypad: &Vec<Vec<Option<char>>>,
    ) -> Vec<(usize, usize)> {
        let mut moves: Vec<(usize, usize)> = Vec::new();
        let mut queue: VecDeque<((usize, usize), Option<((usize, usize), (usize, usize))>)> =
            VecDeque::new();
        let mut visited: HashMap<(usize, usize), Option<((usize, usize), (usize, usize))>> =
            HashMap::new();
        queue.push_back((from, None));
        loop {
            let (curr, prev) = queue.pop_front().unwrap();
            if curr == to {
                break;
            }
            if visited.contains_key(&curr) {
                continue;
            }
            visited.insert(curr, prev);
            let (x, y) = curr;
            for ((dx, dy), key) in [
                ((0, 1), (1, 1)),
                ((0, -1), (1, 0)),
                ((1, 0), (2, 1)),
                ((-1, 0), (0, 1)),
            ] {
                if let (Some(x), Some(y)) = (x.checked_add_signed(dx), y.checked_add_signed(dy)) {
                    if x < keypad[0].len() && y < keypad.len() && keypad[y][x].is_some() {
                        queue.push_back(((x, y), Some((curr, key))));
                    }
                }
            }
        }

        todo!()
    }

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        Ok(reader
            .lines()
            .flatten()
            .map(|line| {
                let code = line[..line.len() - 1].parse::<usize>().unwrap();
                code
            })
            .sum())
    }

    assert_eq!(126384, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    // println!("\n=== Part 2 ===");
    //
    // fn part2<R: BufRead>(reader: R) -> Result<usize> {
    //     Ok(0)
    // }
    //
    // assert_eq!(0, part2(BufReader::new(TEST.as_bytes()))?);
    //
    // let input_file = BufReader::new(File::open(INPUT_FILE)?);
    // let result = time_snippet!(part2(input_file)?);
    // println!("Result = {}", result);
    //endregion

    Ok(())
}
