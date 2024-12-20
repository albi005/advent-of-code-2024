use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Add;

const DAY: &str = "20";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let map = reader
            .lines()
            .flatten()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let start = map
            .iter()
            .enumerate()
            .find_map(|(y, row)| {
                row.iter().enumerate().find_map(
                    |(x, &c)| {
                        if c == 'S' {
                            Some((x, y))
                        } else {
                            None
                        }
                    },
                )
            })
            .unwrap();
        let end = map
            .iter()
            .enumerate()
            .find_map(|(y, row)| {
                row.iter().enumerate().find_map(
                    |(x, &c)| {
                        if c == 'E' {
                            Some((x, y))
                        } else {
                            None
                        }
                    },
                )
            })
            .unwrap();

        let mut visited: HashSet<(usize, usize)> = HashSet::new();
        let mut queue: Vec<(usize, usize, usize)> = Vec::new();
        let mut dist_from_start: Vec<Vec<usize>> = vec![vec![usize::MAX; map.len()]; map.len()];
        let mut dist_from_end: Vec<Vec<usize>> = vec![vec![usize::MAX; map.len()]; map.len()];

        queue.push((start.0, start.1, 0));
        while let Some((x, y, dist)) = queue.pop() {
            let c = map[y][x];
            if c == '#' {
                continue;
            }
            if visited.contains(&(x, y)) {
                continue;
            }
            visited.insert((x, y));
            dist_from_start[y][x] = dist;
            let turns = [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)];
            for (x, y) in turns {
                queue.push((x, y, dist + 1));
            }
        }

        visited.clear();
        queue.clear();
        queue.push((end.0, end.1, 0));
        while let Some((x, y, dist)) = queue.pop() {
            let c = map[y][x];
            if c == '#' {
                continue;
            }
            if visited.contains(&(x, y)) {
                continue;
            }
            visited.insert((x, y));
            dist_from_end[y][x] = dist;
            let turns = [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)];
            for (x, y) in turns {
                queue.push((x, y, dist + 1));
            }
        }

        let dist = dist_from_start[end.1][end.0];
        let max_dist = dist - 100;
        let mut answer = 0;
        for y in 1..map.len() - 1 {
            for x in 1..map[y].len() - 1 {
                let dirs = [((x - 1, y), (x + 1, y)), ((x, y - 1), (x, y + 1))];
                for ((x1, y1), (x2, y2)) in dirs {
                    if map[y1][x1] == '#' || map[y2][x2] == '#' {
                        continue;
                    }
                    let dist1 = dist_from_start[y1][x1] + dist_from_end[y2][x2] + 2;
                    if dist1 <= max_dist {
                        answer += 1;
                    }
                    let dist2 = dist_from_start[y2][x2] + dist_from_end[y1][x1] + 2;
                    if dist2 <= max_dist {
                        answer += 1;
                    }
                }
            }
        }

        Ok(answer)
    }

    // let _ = part1(BufReader::new(TEST.as_bytes()));

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let map = reader
            .lines()
            .flatten()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let start = map
            .iter()
            .enumerate()
            .find_map(|(y, row)| {
                row.iter().enumerate().find_map(
                    |(x, &c)| {
                        if c == 'S' {
                            Some((x, y))
                        } else {
                            None
                        }
                    },
                )
            })
            .unwrap();
        let end = map
            .iter()
            .enumerate()
            .find_map(|(y, row)| {
                row.iter().enumerate().find_map(
                    |(x, &c)| {
                        if c == 'E' {
                            Some((x, y))
                        } else {
                            None
                        }
                    },
                )
            })
            .unwrap();

        let mut visited: HashSet<(usize, usize)> = HashSet::new();
        let mut queue: Vec<(usize, usize, usize)> = Vec::new();
        let mut dist_from_start: Vec<Vec<usize>> = vec![vec![usize::MAX; map.len()]; map.len()];
        let mut dist_from_end: Vec<Vec<usize>> = vec![vec![usize::MAX; map.len()]; map.len()];

        queue.push((start.0, start.1, 0));
        while let Some((x, y, dist)) = queue.pop() {
            let c = map[y][x];
            if c == '#' {
                continue;
            }
            if visited.contains(&(x, y)) {
                continue;
            }
            visited.insert((x, y));
            dist_from_start[y][x] = dist;
            let turns = [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)];
            for (x, y) in turns {
                queue.push((x, y, dist + 1));
            }
        }

        visited.clear();
        queue.clear();
        queue.push((end.0, end.1, 0));
        while let Some((x, y, dist)) = queue.pop() {
            let c = map[y][x];
            if c == '#' {
                continue;
            }
            if visited.contains(&(x, y)) {
                continue;
            }
            visited.insert((x, y));
            dist_from_end[y][x] = dist;
            let turns = [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)];
            for (x, y) in turns {
                queue.push((x, y, dist + 1));
            }
        }

        let dist = dist_from_start[end.1][end.0];
        let max_dist = dist - 100;
        let mut cheats = 0;
        for cheat_start_y in 1..map.len() - 1 {
            for cheat_start_x in 1..map.len() - 1 {
                if map[cheat_start_y][cheat_start_x] == '#' {
                    continue;
                }
                
                const LEN: usize = 20;

                for cheat_end_y in
                    cheat_start_y.saturating_sub(LEN).max(1)..=cheat_start_y.saturating_add(LEN).min(map.len() - 1)
                {
                    let dy = cheat_end_y.abs_diff(cheat_start_y);
                    let remaining_cheat = LEN - dy;
                    for cheat_end_x in
                        cheat_start_x.saturating_sub(remaining_cheat).max(1)..=cheat_start_x.saturating_add(remaining_cheat).min(map.len() - 1)
                    {
                        if map[cheat_end_y][cheat_end_x] == '#' {
                            continue;
                        }
                        
                        let dx = cheat_end_x.abs_diff(cheat_start_x);
                        
                        let dist = dist_from_start[cheat_start_y][cheat_start_x]
                            + dist_from_end[cheat_end_y][cheat_end_x]
                            + dy + dx;
                        if dist <= max_dist {
                            cheats += 1;
                        }
                    }
                }
            }
        }

        Ok(cheats)
    }

    // assert_eq!(0, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
