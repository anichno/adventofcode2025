fn parse_input<'a>(input: impl Iterator<Item = &'a str>) -> Vec<i32> {
    input
        .into_iter()
        .map(|i| {
            let split = i.split_at(1);
            let val: i32 = split.1.parse().unwrap();

            match split.0 {
                "L" => -val,
                "R" => val,
                _ => panic!("Invalid direction: {}", split.0),
            }
        })
        .collect()
}

fn solve1(input: &[i32]) -> i32 {
    let mut location = 50;
    let mut count_zeros = 0;
    for val in input {
        location = (location + val).rem_euclid(100);
        if location == 0 {
            count_zeros += 1;
        }
    }

    count_zeros
}

fn solve2(input: &[i32]) -> i32 {
    let mut location: i32 = 50;
    let mut count_zeros = 0;
    for val in input {
        for _ in 0..val.abs() {
            if *val < 0 {
                location = (location - 1).rem_euclid(100);
            } else {
                location = (location + 1).rem_euclid(100);
            }

            if location == 0 {
                count_zeros += 1;
            }
        }
    }

    count_zeros
}

fn main() {
    let parsed = parse_input(include_str!("input.txt").lines());

    println!("Part 1: {}", solve1(&parsed));
    println!("Part 2: {}", solve2(&parsed));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: [&str; 10] = [
        "L68", "L30", "R48", "L5", "R60", "L55", "L1", "L99", "R14", "L82",
    ];

    #[test]
    fn test1() {
        let parsed = parse_input(INPUT.iter().cloned());
        assert_eq!(solve1(&parsed), 3);
    }

    #[test]
    fn test2() {
        let parsed = parse_input(INPUT.iter().cloned());
        assert_eq!(solve2(&parsed), 6);
    }
}
