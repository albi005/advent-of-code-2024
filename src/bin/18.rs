use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::cmp::Ordering;
use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "18";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R, end_xy: usize, take: usize) -> Result<usize> {
        let size = end_xy + 1;
        let mut map: Vec<Vec<usize>> = (0..size).map(|_| vec![usize::MAX; size]).collect();
        reader
            .lines()
            .flatten()
            .enumerate()
            .take(take)
            .for_each(|(i, line)| {
                let mut parts = line.split(',');
                let x: usize = parts.next().unwrap().parse().unwrap();
                let y: usize = parts.next().unwrap().parse().unwrap();
                map[y][x] = i;
            });
        let mut queue: VecDeque<(usize, usize, usize)> = VecDeque::new();
        let mut visited: HashMap<(usize, usize), usize> = HashMap::new();
        queue.push_back((0, 0, 0));
        while let Some((x, y, dist)) = queue.pop_front() {
            let pos = (x, y);
            if map[y][x] < usize::MAX {
                continue;
            }
            if visited.contains_key(&pos) {
                continue;
            }
            visited.insert(pos, dist);

            for dir in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
                let x = x as isize;
                let x = x + dir.0;
                let y = y as isize;
                let y = y + dir.1;
                let end_xy = end_xy as isize;
                if x < 0 || y < 0 || x > end_xy || y > end_xy {
                    continue;
                }
                queue.push_back((x as usize, y as usize, dist + 1))
            }
        }

        Ok(*visited.get(&(end_xy, end_xy)).unwrap())
    }

    assert_eq!(22, part1(BufReader::new(TEST.as_bytes()), 6, 12)?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file, 70, 1024)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn is_exit_reachable(map: &Vec<Vec<usize>>, take: usize) -> bool {
        let end_xy = map.len()-1;
        let mut queue: VecDeque<(usize, usize, usize)> = VecDeque::new();
        let mut visited: HashMap<(usize, usize), usize> = HashMap::new();
        queue.push_back((0, 0, 0));
        while let Some((x, y, dist)) = queue.pop_front() {
            let pos = (x, y);
            if map[y][x] < take {
                continue;
            }
            if visited.contains_key(&pos) {
                continue;
            }
            visited.insert(pos, dist);

            for dir in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
                let x = x as isize;
                let x = x + dir.0;
                let y = y as isize;
                let y = y + dir.1;
                let end_xy = end_xy as isize;
                if x < 0 || y < 0 || x > end_xy || y > end_xy {
                    continue;
                }
                queue.push_back((x as usize, y as usize, dist + 1))
            }
        }

        visited.contains_key(&(end_xy, end_xy))
    }

    fn part2<R: BufRead>(reader: R, end_xy: usize) -> Result<String> {
        let size = end_xy + 1;
        let mut map: Vec<Vec<usize>> = (0..size).map(|_| vec![usize::MAX; size]).collect();
        let corruption: Vec<_> = reader
            .lines()
            .flatten()
            .map(|line| {
                let comma = line.find(',').unwrap();
                let x: usize = line[..comma].parse().unwrap();
                let y: usize = line[comma + 1..].parse().unwrap();
                (x, y)
            })
            .collect();
        corruption
            .iter()
            .enumerate()
            .for_each(|(i, &(x, y))| map[y][x] = i);

        let indexes: Vec<usize> = (0..corruption.len()).collect();
        let i = indexes.binary_search_by(|&i| {
            if is_exit_reachable(&map, i) {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        }).err().unwrap();

        let (x,y) = corruption[i-1];
        Ok(format!("{},{}", x, y))
    }

    assert_eq!("6,1", part2(BufReader::new(TEST.as_bytes()), 6)?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file, 70)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
