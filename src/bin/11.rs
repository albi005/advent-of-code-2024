use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "11";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
125 17
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(mut reader: R) -> Result<usize> {
        let mut s = String::new();
        reader.read_to_string(&mut s)?;
        let mut state: Vec<usize> = s
            .split_ascii_whitespace()
            .map(|x| x.parse::<usize>())
            .flatten()
            .collect();

        for _ in 0..25 {
            let mut next = Vec::new();
            state.iter().for_each(|x| {
                if *x == 0 {
                    next.push(1)
                } else {
                    let digits = x.ilog10() + 1;
                    if digits % 2 == 0 {
                        let y = 10usize.pow(digits / 2);
                        let left = x / y;
                        let right = x % y;
                        next.push(left);
                        next.push(right);
                    } else {
                        next.push(x * 2024)
                    }
                }
            });
            state = next;
        }

        Ok(state.len())
    }

    assert_eq!(55312, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(mut reader: R) -> Result<usize> {
        let mut s = String::new();
        reader.read_to_string(&mut s)?;
        let state: Vec<usize> = s
            .split_ascii_whitespace()
            .map(|x| x.parse::<usize>())
            .flatten()
            .collect();

        let mut memo: HashMap<(usize, usize), usize> = HashMap::new();

        fn get_count_after_steps(num: usize, steps_left: usize, memo: &mut HashMap<(usize, usize), usize>) -> usize {
            if steps_left == 0 {
                return 1;
            }
            if let Some(res) = memo.get(&(num, steps_left)) {
                return *res;
            }

            let res = if num == 0 {
                get_count_after_steps(1, steps_left - 1, memo)
            } else {
                let digits = num.ilog10() + 1;
                if digits % 2 == 0 {
                    let y = 10usize.pow(digits / 2);
                    let left = num / y;
                    let right = num % y;
                    let left = get_count_after_steps(left, steps_left - 1, memo);
                    let right = get_count_after_steps(right, steps_left - 1, memo);
                    left + right
                } else {
                    get_count_after_steps(num * 2024, steps_left - 1, memo)
                }
            };
            memo.insert((num, steps_left), res);

            res
        }

        Ok(state.iter().map(|x| get_count_after_steps(*x, 75, &mut memo)).sum())
    }

    // huh
    // assert_eq!(55312, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
