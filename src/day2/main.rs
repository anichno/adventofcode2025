use std::ops::RangeInclusive;

fn parse_input(input: &str) -> Vec<RangeInclusive<i64>> {
    input
        .split(",")
        .map(|s| {
            let (start, end) = s.split_once("-").unwrap();
            start.parse().unwrap()..=end.parse().unwrap()
        })
        .collect()
}

fn solve1(input: &[RangeInclusive<i64>]) -> i64 {
    let mut sum = 0;

    for range in input.iter().cloned() {
        for i in range {
            let id_str = i.to_string();
            if id_str.len() % 2 == 0 {
                let (left, right) = id_str.split_at(id_str.len() / 2);
                if left == right {
                    sum += i;
                }
            }
        }
    }

    sum
}

fn solve2(input: &[RangeInclusive<i64>]) -> i64 {
    let mut sum = 0;
    let invalid_id_regex = fancy_regex::Regex::new(r"^([0-9]+?)\1+$").unwrap();

    for range in input.iter().cloned() {
        for i in range {
            let id_str = i.to_string();
            if invalid_id_regex.is_match(&id_str).unwrap() {
                sum += i;
            }
        }
    }

    sum
}

fn main() {
    let parsed = parse_input(include_str!("input.txt"));

    println!("Part 1: {}", solve1(&parsed));
    println!("Part 2: {}", solve2(&parsed));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn test1() {
        let parsed = parse_input(INPUT);
        assert_eq!(solve1(&parsed), 1227775554);
    }

    #[test]
    fn test2() {
        let parsed = parse_input(INPUT);
        assert_eq!(solve2(&parsed), 4174379265);
    }
}
