use std::collections::{HashMap, HashSet};
use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use adv_code_2024::*;

const DAY: &str = "08";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut width = 0;
        Ok(reader.lines()
            .flatten()
            .enumerate()
            .inspect(|(y, _)| width = *y as isize + 1)
            .map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .filter(|(_, c)| c != &'.')
                    .map(|(x, c)| (c, (x, y)))
                    .collect::<Vec<_>>()
            })
            .flatten()
            .into_group_map()
            .iter()
            .map(|(_, coords)| {
                coords
                    .iter()
                    .combinations(2)
                    .map(|pair| {
                        let (a, b) = (pair[0], pair[1]);
                        // a -> b
                        let (dx, dy) = (b.0 as isize - a.0 as isize, b.1 as isize - a.1 as isize);
                        // b + (a -> b)
                        let antinode1 = (b.0 as isize + dx, b.1 as isize + dy);
                        // a + (b -> a)
                        let antinode2 = (a.0 as isize - dx, a.1 as isize - dy);
                        
                        [antinode1, antinode2]
                    })
            })
            .flatten()
            .flatten()
            .filter(|antinode| {
                antinode.0 >= 0 && antinode.1 >= 0 && antinode.0 < width && antinode.1 < width
            })
            .collect::<HashSet<_>>()
            .iter().inspect(|x| println!("{:?}", x))
            .count())
    }

    assert_eq!(14, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");
    
    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut width = 0;
        Ok(reader.lines()
            .flatten()
            .enumerate()
            .inspect(|(y, _)| width = *y as isize + 1)
            .map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .filter(|(_, c)| c != &'.')
                    .map(|(x, c)| (c, (x, y)))
                    .collect::<Vec<_>>()
            })
            .flatten()
            .into_group_map()
            .iter()
            .map(|(_, coords)| {
                coords
                    .iter()
                    .combinations(2)
                    .map(|pair| {
                        let (a, b) = (pair[0], pair[1]);
                        // a -> b
                        let (dx, dy) = (b.0 as isize - a.0 as isize, b.1 as isize - a.1 as isize);
                        
                        let mut res: Vec<_> = Vec::new();
                        for i in 0.. {
                            let antinode1 = (b.0 as isize + dx * i, b.1 as isize + dy * i);
                            if antinode1.0 < 0 || antinode1.1 < 0 || antinode1.0 >= width || antinode1.1 >= width {
                                break;
                            }
                            res.push(antinode1);
                        }
                        for i in 0.. {
                            let antinode2 = (a.0 as isize - dx * i, a.1 as isize - dy * i);
                            if antinode2.0 < 0 || antinode2.1 < 0 || antinode2.0 >= width || antinode2.1 >= width {
                                break;
                            }
                            res.push(antinode2);
                        }
                        res
                    })
            })
            .flatten()
            .flatten()
            .collect::<HashSet<_>>()
            .iter().inspect(|x| println!("{:?}", x))
            .count())
    }
    
    assert_eq!(34, part2(BufReader::new(TEST.as_bytes()))?);
    
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
