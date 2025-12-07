use std::collections::{HashMap, HashSet};

fn parse_input<'a>(mut input: impl Iterator<Item = &'a str>) -> (usize, Vec<Vec<usize>>) {
    let start_idx = input
        .next()
        .unwrap()
        .chars()
        .enumerate()
        .find_map(|c| if c.1 == 'S' { Some(c.0) } else { None })
        .unwrap();

    let rows = input
        .filter_map(|row| {
            let row: Vec<usize> = row
                .chars()
                .enumerate()
                .filter_map(|(i, c)| if c == '^' { Some(i) } else { None })
                .collect();
            if !row.is_empty() { Some(row) } else { None }
        })
        .collect();

    (start_idx, rows)
}

fn solve1(input: &(usize, Vec<Vec<usize>>)) -> usize {
    let mut beams = HashSet::new();
    beams.insert(input.0);

    let mut splits = 0;
    for row in &input.1 {
        for col_idx in row {
            if beams.contains(col_idx) {
                splits += 1;
                beams.remove(col_idx);
                beams.insert(*col_idx - 1);
                beams.insert(*col_idx + 1);
            }
        }
    }

    splits
}

fn solve2(input: &(usize, Vec<Vec<usize>>)) -> usize {
    fn split(
        beam: usize,
        splitters: &[Vec<usize>],
        memory: &mut HashMap<(usize, usize), usize>,
    ) -> usize {
        let mut paths = 1;
        if let Some(row) = splitters.first() {
            if row.contains(&beam) {
                if let Some(memorized_result) = memory.get(&(beam, splitters.len())) {
                    paths = *memorized_result;
                } else {
                    paths = split(beam - 1, &splitters[1..], memory)
                        + split(beam + 1, &splitters[1..], memory);
                    memory.insert((beam, splitters.len()), paths);
                }
            } else {
                paths = split(beam, &splitters[1..], memory)
            }
        }

        paths
    }

    let mut beams = HashSet::new();
    beams.insert(input.0);

    let mut memory = HashMap::new();

    split(input.0, &input.1, &mut memory)
}

fn main() {
    let parsed = parse_input(include_str!("input.txt").lines());

    println!("Part 1: {}", solve1(&parsed));
    println!("Part 2: {}", solve2(&parsed));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: [&str; 16] = [
        ".......S.......",
        "...............",
        ".......^.......",
        "...............",
        "......^.^......",
        "...............",
        ".....^.^.^.....",
        "...............",
        "....^.^...^....",
        "...............",
        "...^.^...^.^...",
        "...............",
        "..^...^.....^..",
        "...............",
        ".^.^.^.^.^...^.",
        "...............",
    ];

    #[test]
    fn test1() {
        let parsed = parse_input(INPUT.iter().cloned());
        assert_eq!(solve1(&parsed), 21);
    }

    #[test]
    fn test2() {
        let parsed = parse_input(INPUT.iter().cloned());
        assert_eq!(solve2(&parsed), 40);
    }
}
