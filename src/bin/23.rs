use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "23";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut map: HashMap<&str, Vec<&str>> = HashMap::new();
        let lines: Vec<String> = reader.lines().flatten().collect();
        for line in &lines {
            let from = &line[..2];
            let to = &line[3..5];
            map.entry(from).or_insert(Vec::new()).push(to);
            map.entry(to).or_insert(Vec::new()).push(from);
        }

        let candidates: Vec<_> = map
            .iter()
            .filter(|(from, _)| from.starts_with('t'))
            .collect();

        dbg!(candidates.len()); // 20 possible Chief Historians

        println!(
            "{:?}",
            candidates
                .iter()
                .map(|(_, tos)| { tos.len() })
                .max()
                .unwrap()
        ); // max possible degree for Chief Historian is 13

        dbg!(lines.len()); // 3380 edges

        let mut visited = HashSet::new();
        fn rec<'a>(
            node: &'a str,
            visited: &mut HashSet<&'a str>,
            map: &HashMap<&str, Vec<&'a str>>,
        ) {
            if visited.contains(node) {
                return;
            }
            visited.insert(node);
            let neighbors = map.get(&node).unwrap();
            for &neighbor in neighbors {
                rec(neighbor, visited, map);
            }
        }
        for line in &lines {
            if visited.contains(&line[..2]) {
                continue;
            }
            rec(&line[..2], &mut visited, &map);
            dbg!(visited.len()); // 1 component with 520 nodes
        }
        dbg!(map.values().map(|v| v.len()).max().unwrap()); // max degree is 13

        let mut cliques = HashSet::new();
        for &(candidate, neighbors) in &candidates {
            for &neighbor1 in neighbors {
                for &neighbor2 in &neighbors[1..] {
                    if map.get(&neighbor1).unwrap().contains(&neighbor2) {
                        let mut res = [candidate, neighbor1, neighbor2];
                        res.sort();
                        cliques.insert(res);
                    }
                }
            }
        }

        Ok(cliques.len())
    }

    assert_eq!(7, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<String> {
        let mut map: HashMap<&str, Vec<&str>> = HashMap::new();
        let lines: Vec<String> = reader.lines().flatten().collect();
        for line in &lines {
            let from = &line[..2];
            let to = &line[3..5];
            map.entry(from).or_insert(Vec::new()).push(to);
            map.entry(to).or_insert(Vec::new()).push(from);
        }

        fn get_largest_clique<'a>(
            clique: Vec<&'a str>,
            neighbors: &[&'a str],
            map: &HashMap<&str, Vec<&str>>,
        ) -> Vec<&'a str> {
            if neighbors.len() == 0 {
                return clique;
            }

            let with = {
                let mut set = clique.clone();
                let new_element = neighbors[0];
                let new_element_neighbors = map.get(&new_element).unwrap();
                if clique
                    .iter()
                    .all(|node| new_element_neighbors.contains(node))
                {
                    set.push(new_element);
                    let new_clique = set;
                    Some(get_largest_clique(new_clique, &neighbors[1..], map))
                } else {
                    None
                }
            };
            let without = get_largest_clique(clique, &neighbors[1..], map);
            if let Some(with) = with {
                if with.len() > without.len() {
                    with
                } else {
                    without
                }
            } else {
                without
            }
        }
        let largest = map
            .iter()
            .map(|(&node, neighbors)| get_largest_clique(vec![node], neighbors, &map))
            .max_by_key(|clique| clique.len())
            .unwrap();

        Ok(largest.iter().sorted().join(","))
    }

    assert_eq!("co,de,ka,ta", part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
