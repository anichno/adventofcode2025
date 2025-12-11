use std::{collections::HashMap, hash::RandomState};

use petgraph::{Directed, prelude::GraphMap};

fn parse_input<'a>(input: impl Iterator<Item = &'a str>) -> GraphMap<&'a str, i32, Directed> {
    let mut conn_graph = petgraph::graphmap::DiGraphMap::new();

    for (node_a_str, node_b_str) in input.flat_map(|line| {
        let (start, targets) = line.split_once(": ").unwrap();
        targets
            .split_ascii_whitespace()
            .map(move |tgt| (start, tgt))
    }) {
        conn_graph.add_edge(node_a_str, node_b_str, 1);
    }

    conn_graph
}

fn solve1(input: &GraphMap<&str, i32, Directed>) -> usize {
    petgraph::algo::all_simple_paths::<Vec<_>, _, RandomState>(input, "you", "out", 0, None).count()
}

fn solve2(input: &GraphMap<&str, i32, Directed>) -> usize {
    fn num_paths_to_out(
        cur_location: &str,
        dac_hit: bool,
        fft_hit: bool,
        graph: &GraphMap<&str, i32, Directed>,
        memory: &mut HashMap<(String, bool, bool), usize>,
    ) -> usize {
        if cur_location == "out" {
            if dac_hit && fft_hit {
                return 1;
            } else {
                return 0;
            }
        }

        let dac_hit = dac_hit | (cur_location == "dac");
        let fft_hit = fft_hit | (cur_location == "fft");

        if let Some(known) = memory.get(&(cur_location.to_string(), dac_hit, fft_hit)) {
            *known
        } else {
            let result = graph
                .neighbors(cur_location)
                .map(|n| num_paths_to_out(n, dac_hit, fft_hit, graph, memory))
                .sum();
            memory.insert((cur_location.to_string(), dac_hit, fft_hit), result);
            result
        }
    }

    let mut memory = HashMap::new();
    num_paths_to_out("svr", false, false, input, &mut memory)
}

fn main() {
    let parsed = parse_input(include_str!("input.txt").lines());

    println!("Part 1: {}", solve1(&parsed));
    println!("Part 2: {}", solve2(&parsed));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: [&str; 10] = [
        "aaa: you hhh",
        "you: bbb ccc",
        "bbb: ddd eee",
        "ccc: ddd eee fff",
        "ddd: ggg",
        "eee: out",
        "fff: out",
        "ggg: out",
        "hhh: ccc fff iii",
        "iii: out",
    ];

    const INPUT2: [&str; 13] = [
        "svr: aaa bbb",
        "aaa: fft",
        "fft: ccc",
        "bbb: tty",
        "tty: ccc",
        "ccc: ddd eee",
        "ddd: hub",
        "hub: fff",
        "eee: dac",
        "dac: fff",
        "fff: ggg hhh",
        "ggg: out",
        "hhh: out",
    ];

    #[test]
    fn test1() {
        let parsed = parse_input(INPUT1.iter().cloned());
        assert_eq!(solve1(&parsed), 5);
    }

    #[test]
    fn test2() {
        let parsed = parse_input(INPUT2.iter().cloned());
        assert_eq!(solve2(&parsed), 2);
    }
}
