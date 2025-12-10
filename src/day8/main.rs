use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Location {
    x: i64,
    y: i64,
    z: i64,
}

impl Location {
    fn distance(&self, other: &Self) -> f64 {
        (((self.x - other.x).pow(2) + (self.y - other.y).pow(2) + (self.z - other.z).pow(2)) as f64)
            .sqrt()
    }
}

fn parse_input<'a>(input: impl Iterator<Item = &'a str>) -> Vec<Location> {
    input
        .map(|line| {
            let mut parts = line.split(",");
            Location {
                x: parts.next().unwrap().parse().unwrap(),
                y: parts.next().unwrap().parse().unwrap(),
                z: parts.next().unwrap().parse().unwrap(),
            }
        })
        .collect()
}

fn solve1(input: &[Location], num_pairs: usize) -> usize {
    let mut circuits: Vec<HashSet<&Location>> =
        input.iter().map(|l| HashSet::from_iter([l])).collect();

    let mut box_pairs = Vec::new();
    for (a_idx, box_a) in input.iter().enumerate() {
        for box_b in input.iter().skip(a_idx + 1) {
            let dist = box_a.distance(box_b);
            box_pairs.push((dist, box_a, box_b));
        }
    }

    box_pairs.sort_unstable_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    for (_, box_a, box_b) in box_pairs.into_iter().take(num_pairs) {
        let circuit_a_idx = circuits
            .iter()
            .enumerate()
            .find_map(|c| if c.1.contains(box_a) { Some(c.0) } else { None })
            .unwrap();

        let circuit_b_idx = circuits
            .iter()
            .enumerate()
            .find_map(|c| if c.1.contains(box_b) { Some(c.0) } else { None })
            .unwrap();

        if circuit_a_idx != circuit_b_idx {
            // connect them
            let circuit_b = circuits[circuit_b_idx].clone();
            circuits[circuit_a_idx].extend(&circuit_b);
            circuits.remove(circuit_b_idx);
        }
    }

    let mut circuit_lengths: Vec<usize> = circuits.iter().map(|c| c.len()).collect();
    circuit_lengths.sort();
    circuit_lengths.reverse();

    circuit_lengths[0] * circuit_lengths[1] * circuit_lengths[2]
}

fn solve2(input: &[Location]) -> i64 {
    let mut circuits: Vec<HashSet<&Location>> =
        input.iter().map(|l| HashSet::from_iter([l])).collect();

    let mut box_pairs = Vec::new();
    for (a_idx, box_a) in input.iter().enumerate() {
        for box_b in input.iter().skip(a_idx + 1) {
            let dist = box_a.distance(box_b);
            box_pairs.push((dist, box_a, box_b));
        }
    }

    box_pairs.sort_unstable_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    for (_, box_a, box_b) in box_pairs.into_iter() {
        let circuit_a_idx = circuits
            .iter()
            .enumerate()
            .find_map(|c| if c.1.contains(box_a) { Some(c.0) } else { None })
            .unwrap();

        let circuit_b_idx = circuits
            .iter()
            .enumerate()
            .find_map(|c| if c.1.contains(box_b) { Some(c.0) } else { None })
            .unwrap();

        if circuit_a_idx != circuit_b_idx {
            // connect them
            let circuit_b = circuits[circuit_b_idx].clone();
            circuits[circuit_a_idx].extend(&circuit_b);
            circuits.remove(circuit_b_idx);

            if circuits.len() == 1 {
                return box_a.x * box_b.x;
            }
        }
    }

    0
}

fn main() {
    let parsed = parse_input(include_str!("input.txt").lines());

    // too high: 1778638
    println!("Part 1: {}", solve1(&parsed, 1000));
    println!("Part 2: {}", solve2(&parsed));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: [&str; 20] = [
        "162,817,812",
        "57,618,57",
        "906,360,560",
        "592,479,940",
        "352,342,300",
        "466,668,158",
        "542,29,236",
        "431,825,988",
        "739,650,466",
        "52,470,668",
        "216,146,977",
        "819,987,18",
        "117,168,530",
        "805,96,715",
        "346,949,466",
        "970,615,88",
        "941,993,340",
        "862,61,35",
        "984,92,344",
        "425,690,689",
    ];

    #[test]
    fn test1() {
        let parsed = parse_input(INPUT.iter().cloned());
        assert_eq!(solve1(&parsed, 10), 40);
    }

    #[test]
    fn test2() {
        let parsed = parse_input(INPUT.iter().cloned());
        assert_eq!(solve2(&parsed), 25272);
    }
}
