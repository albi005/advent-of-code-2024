use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter;

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

    fn get_min_moves(
        prev: (usize, usize),
        curr: (usize, usize),
        target: (usize, usize),
        keypad: usize,
        memo: &mut HashMap<((usize, usize), (usize, usize), (usize, usize), usize), usize>,
    ) -> usize {
        if keypad == 3 {
            return 1;
        }
        if start == target {
            return 0;
        }
        if let Some(&min_moves) = memo.get(&(start, prev, target, keypad)) {
            return min_moves;
        }

        let directional_keypad_empty = (0, 0);
        let directional_keypad_up = ((1, 0), (0, -1));
        let directional_keypad_a = (2, 0);
        let directional_keypad_left = ((0, 1), (-1, 0));
        let directional_keypad_down = ((1, 1), (0, 1));
        let directional_keypad_right = ((2, 1), (1, 0));
        let numeric_keypad_empty = (3, 0);

        [
            directional_keypad_up,
            directional_keypad_left,
            directional_keypad_down,
            directional_keypad_right,
        ]
        .iter()
        .filter_map(|&(key, diff)| {
            if let (Some(x), Some(y)) = (
                start.0.checked_add_signed(diff.0),
                start.1.checked_add_signed(diff.1),
            ) {
                let next = (x, y);
                let empty = match keypad {
                    0 => numeric_keypad_empty,
                    _ => directional_keypad_empty,
                };
                if next != empty {
                    return Some(
                        get_min_moves(next, target, key, keypad, memo)
                            + get_min_moves(prev, key, prev, keypad - 1, memo),
                    );
                }
            }
            return None;
        })
        .min()
        .unwrap()
    }

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut memo = HashMap::new();
        Ok(reader
            .lines()
            .flatten()
            .map(|line| {
                let numeric_keypad = [
                    (1, 3),
                    (0, 2),
                    (1, 2),
                    (2, 2),
                    (0, 1),
                    (1, 1),
                    (2, 1),
                    (0, 0),
                    (1, 0),
                    (2, 0),
                ];
                let numeric_keypad_a = (2, 3);
                line[..line.len() - 1]
                    .chars()
                    .map(|c| numeric_keypad[c.to_digit(10).unwrap() as usize])
                    .chain(iter::once(numeric_keypad_a))
                    .fold((numeric_keypad_a, 0), |(prev, sum), curr| {
                        (curr, sum + get_min_moves(prev, curr, prev, 0, &mut memo))
                    })
                    .1
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
