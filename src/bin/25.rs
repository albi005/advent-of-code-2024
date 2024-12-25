use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "25";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####

";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut keys = Vec::new();
        let mut locks = Vec::new();
        reader
            .lines()
            .flatten()
            .chunks(8)
            .into_iter()
            .for_each(|x| {
                let mut lines: Vec<_> = x.collect();
                lines.pop();
                let collection = if lines[0].chars().nth(0) == Some('#') {
                    &mut locks
                } else {
                    lines.reverse();
                    &mut keys
                };
                let res = (0..5)
                    .map(|i| {
                        lines
                            .iter()
                            .find_position(|line| line.as_bytes()[i] == b'.')
                            .unwrap()
                            .0
                            - 1
                    })
                    .collect_vec();
                collection.push(res);
            });

        Ok(keys
            .iter()
            .map(|key| {
                locks
                    .iter()
                    .filter(|lock| (0..5).all(|i| lock[i] + key[i] <= 5))
                    .count()
            })
            .sum())
    }

    assert_eq!(3, part1(BufReader::new(TEST.as_bytes()))?);

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
