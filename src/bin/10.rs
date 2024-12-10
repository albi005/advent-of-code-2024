use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "10";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let map: Vec<Vec<u8>> = reader
            .lines()
            .flatten()
            .map(|line| line.as_bytes().iter().map(|b| *b - b'0').collect())
            .collect();

        let width = map[0].len();
        let height = map.len();
        let mut sum = 0;

        fn get_score(
            prev: isize,
            x: isize,
            y: isize,
            map: &Vec<Vec<u8>>,
            width: isize,
            height: isize,
        ) -> Option<Vec<(usize, usize)>> {
            let dirs = [(1, 0), (0, 1), (-1, 0), (0, -1)];

            if x < 0 || y < 0 || x >= width || y >= height {
                return None;
            }
            let curr = map[y as usize][x as usize];
            if curr != (prev + 1) as u8 {
                return None;
            }
            if curr == 9 {
                return Some(vec![(x as usize, y as usize)]);
            }
            Some(
                dirs.iter()
                    .map(|(dx, dy)| get_score(curr as isize, x + dx, y + dy, map, width, height))
                    .flatten()
                    .flatten()
                    .unique()
                    .collect(),
            )
        }

        for start_y in 0..height {
            for start_x in 0..width {
                if map[start_y][start_x] != 0 {
                    continue;
                }

                sum += get_score(
                    -1,
                    start_x as isize,
                    start_y as isize,
                    &map,
                    width as isize,
                    height as isize,
                ).map_or(0, |v| v.len());
            }
        }

        Ok(sum)
    }

    assert_eq!(36, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let map: Vec<Vec<u8>> = reader
            .lines()
            .flatten()
            .map(|line| line.as_bytes().iter().map(|b| *b - b'0').collect())
            .collect();

        let width = map[0].len();
        let height = map.len();
        let mut sum = 0;

        fn get_score(
            prev: isize,
            x: isize,
            y: isize,
            map: &Vec<Vec<u8>>,
            width: isize,
            height: isize,
        ) -> usize {
            let dirs = [(1, 0), (0, 1), (-1, 0), (0, -1)];

            if x < 0 || y < 0 || x >= width || y >= height {
                return 0;
            }
            let curr = map[y as usize][x as usize];
            if curr != (prev + 1) as u8 {
                return 0;
            }
            if curr == 9 {
                return 1;
            }
            dirs.iter()
                .map(|(dx, dy)| {
                    get_score(
                        curr as isize,
                        x + dx,
                        y + dy,
                        map,
                        width,
                        height,
                    )
                })
                .sum()
        }

        for start_y in 0..height {
            for start_x in 0..width {
                if map[start_y][start_x] != 0 {
                    continue;
                }

                sum += get_score(-1, start_x as isize, start_y as isize, &map, width as isize, height as isize);
            }
        }

        Ok(sum)
    }
    
    assert_eq!(81, part2(BufReader::new(TEST.as_bytes()))?);
    
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
