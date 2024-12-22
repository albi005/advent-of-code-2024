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

    const DIRECTIONAL_KEYPAD_A: (usize, usize) = (2, 0);

    // 0: numerical
    // 1: directional  | controlled by
    // 2: directional  V
    // 3: directional
    // user
    fn get_min_moves(
        curr: (usize, usize),
        target: (usize, usize),
        last_key: (usize, usize), // how we got here; the last key pressed on the controlling keypad
        keypad: usize,
        memo: &mut HashMap<((usize, usize), (usize, usize), (usize, usize), usize), usize>,
    ) -> usize {
        if keypad == 300 {
            return 1;
        }
        if curr == target {
            return get_min_moves(
                last_key,
                DIRECTIONAL_KEYPAD_A,
                DIRECTIONAL_KEYPAD_A,
                keypad + 1,
                memo,
            );
        }
        if let Some(&min_moves) = memo.get(&(curr, target, last_key, keypad)) {
            return min_moves;
        }
        memo.insert((curr, target, last_key, keypad), 69000000000000);

        let directional_keypad_empty = (0, 0);
        let directional_keypad_up = ((1, 0), (0, -1));
        let directional_keypad_left = ((0, 1), (-1, 0));
        let directional_keypad_down = ((1, 1), (0, 1));
        let directional_keypad_right = ((2, 1), (1, 0));
        let numeric_keypad_empty = (0, 3);

        let res = [
            directional_keypad_down,
            directional_keypad_left,
            directional_keypad_right,
            directional_keypad_up,
        ]
        .iter()
        .filter_map(|&(key, diff)| {
            if let (Some(x), Some(y)) = (
                curr.0.checked_add_signed(diff.0),
                curr.1.checked_add_signed(diff.1),
            ) {
                let next = (x, y);
                let (max_x, max_y) = match keypad {
                    0 => (2, 3),
                    _ => (2, 1),
                };
                if x > max_x || y > max_y {
                    return None;
                }
                let empty = match keypad {
                    0 => numeric_keypad_empty,
                    _ => directional_keypad_empty,
                };
                if next == empty {
                    return None;
                }
                let res = Some(
                    get_min_moves(last_key, key, DIRECTIONAL_KEYPAD_A, keypad + 1, memo)
                    + get_min_moves(next, target, key, keypad, memo),
                );
                return res;
            }
            return None;
        })
        .min()
        .unwrap();
        memo.insert((curr, target, last_key, keypad), res);
        res
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
                let num: usize = line[..line.len() - 1].parse().unwrap();
                let res = line[..line.len() - 1]
                    .chars()
                    .map(|c| numeric_keypad[c.to_digit(10).unwrap() as usize])
                    .chain(iter::once(numeric_keypad_a))
                    .fold((numeric_keypad_a, 0), |(prev, sum), curr| {
                        let res = (
                            curr,
                            sum + get_min_moves(prev, curr, DIRECTIONAL_KEYPAD_A, 0, &mut memo),
                        );
                        res
                    })
                    .1
                    * num;
                dbg!(res / num, num);
                res
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
