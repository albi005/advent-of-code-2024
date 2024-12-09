use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, LinkedList, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "09";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
2333133121414131402
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn print_fs(fs: &[i16]) {
        if fs.len() > 100 {
            return;
        }
        for &x in fs.iter() {
            if x == -1 {
                print!(".");
            } else {
                print!("{}", x);
            }
        }
        println!();
    }

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let input: Vec<u8> = reader
            .lines()
            .flatten()
            .next()
            .unwrap()
            .bytes()
            .map(|b| (b - b'0'))
            .collect(); // yikes
        let mut fs: Vec<i16> = Vec::new();
        for i in 0..((input.len() / 2) + 1) {
            let file_size = input[i * 2];
            for _ in 0..file_size {
                fs.push(i as i16);
            }
            if i == input.len() / 2 {
                // yikes yikes
                break;
            }
            let empty = input[i * 2 + 1];
            for _ in 0..empty {
                fs.push(-1);
            }
        }

        let mut start = 0;
        let mut end = fs.len() - 1;
        print_fs(&fs);
        loop {
            while start < end && fs[end] == -1 {
                end -= 1;
            }
            while start < end && fs[start] != -1 {
                start += 1;
            }
            while start < end && fs[start] == -1 && fs[end] != -1 {
                fs[start] = fs[end];
                fs[end] = -1;
                start += 1;
                end -= 1;
            }
            if start >= end {
                break;
            }
        }
        print_fs(&fs);
        let checksum: usize = fs
            .iter()
            .take_while(|&&x| x != -1)
            .enumerate()
            .map(|(i, &x)| x as usize * i)
            .sum();

        Ok(checksum)
    }

    assert_eq!(1928, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    struct File2 {
        start: usize,
        size: u16,
    }

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let input: Vec<u8> = reader
            .lines()
            .flatten()
            .next()
            .unwrap()
            .bytes()
            .map(|b| (b - b'0'))
            .collect(); // yikes
        let mut empty_starts: Vec<BinaryHeap<Reverse<usize>>> =
            (0..10).map(|_| BinaryHeap::new()).collect();
        let mut files: Vec<File2> = Vec::new();
        let mut start = 0;
        for pair in input.chunks(2) {
            let file = File2 {
                size: pair[0] as u16,
                start,
            };
            start += file.size as usize;
            files.push(file);

            if pair.len() == 2 {
                let empty_len = pair[1] as u16;
                empty_starts[empty_len as usize].push(Reverse(start));
                start += empty_len as usize;
            }
        }

        files.iter_mut().rev().for_each(|file| {
            let space = empty_starts
                .iter()
                .enumerate()
                .skip(file.size as usize)
                .filter_map(|(space_len, starts)| {
                    if let Some(Reverse(start)) = starts.peek() {
                        if start < &file.start {
                            return Some((space_len as u16, *start));
                        }
                    }
                    return None;
                })
                .min_by_key(|(_, start)| *start);
            if let Some((space_len, space_start)) = space {
                empty_starts[space_len as usize].pop();
                file.start = space_start;
                let leftover_len = space_len - file.size;
                let leftover_start = space_start + file.size as usize;
                empty_starts[leftover_len as usize].push(Reverse(leftover_start));
            }
        });

        Ok(files
            .iter()
            .enumerate()
            .map(|(file_id, file)| {
                (file.start..file.start + file.size as usize)
                    .map(|i| i * file_id)
                    .sum::<usize>()
            })
            .sum())
    }

    assert_eq!(2858, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
