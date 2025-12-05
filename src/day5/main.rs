use std::ops::RangeInclusive;

fn parse_input<'a>(
    mut input: impl Iterator<Item = &'a str>,
) -> (Vec<RangeInclusive<i64>>, Vec<i64>) {
    let mut ranges = Vec::new();
    while let Some(line) = input.next()
        && !line.is_empty()
    {
        let (start, end) = line.split_once("-").unwrap();
        let start = start.parse().unwrap();
        let end = end.parse().unwrap();

        ranges.push(start..=end);
    }

    let ingredient_ids = input.map(|line| line.parse().unwrap()).collect();

    (ranges, ingredient_ids)
}

fn solve1(input: &(Vec<RangeInclusive<i64>>, Vec<i64>)) -> usize {
    input
        .1
        .iter()
        .filter(|ingredient| input.0.iter().any(|range| range.contains(*ingredient)))
        .count()
}

fn solve2(input: &(Vec<RangeInclusive<i64>>, Vec<i64>)) -> usize {
    let mut ranges = input.0.clone();
    ranges.sort_by(|a, b| {
        if a.start() != b.start() {
            (*a.start()).cmp(b.start())
        } else {
            (*a.end()).cmp(b.end())
        }
    });

    let mut new_ranges = vec![ranges[0].clone()];
    for i in ranges.into_iter().skip(1) {
        let prev = new_ranges.last().unwrap();

        let new_start = if *i.start() < *prev.start() {
            *prev.end() + 1
        } else {
            *i.start()
        };

        if prev.contains(&new_start) && prev.contains(i.end()) {
            // redundant range, continue
            continue;
        }

        if !prev.contains(&new_start) && !prev.contains(i.end()) {
            // disjoint, this range can just be added
            new_ranges.push(new_start..=*i.end());
            continue;
        }

        new_ranges.push(*prev.end() + 1..=*i.end());
    }

    new_ranges.into_iter().map(|r| r.count()).sum()
}

fn main() {
    let parsed = parse_input(include_str!("input.txt").lines());

    println!("Part 1: {}", solve1(&parsed));
    println!("Part 2: {}", solve2(&parsed));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: [&str; 11] = [
        "3-5", "10-14", "16-20", "12-18", "", "1", "5", "8", "11", "17", "32",
    ];

    #[test]
    fn test1() {
        let parsed = parse_input(INPUT.iter().cloned());
        assert_eq!(solve1(&parsed), 3);
    }

    #[test]
    fn test2() {
        let parsed = parse_input(INPUT.iter().cloned());
        assert_eq!(solve2(&parsed), 14);
    }
}
