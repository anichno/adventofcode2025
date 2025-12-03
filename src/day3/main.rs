fn parse_input<'a>(input: impl Iterator<Item = &'a str>) -> Vec<Vec<u32>> {
    input
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

fn solve1(input: &[Vec<u32>]) -> u32 {
    let mut sum = 0;

    for bank in input {
        let mut max_val = 0;
        for i in 0..bank.len() - 1 {
            for j in i + 1..bank.len() {
                let val = bank[i] * 10 + bank[j];
                max_val = max_val.max(val);
            }
        }
        sum += max_val;
    }

    sum
}

fn solve2(input: &[Vec<u32>]) -> u64 {
    fn maximize_bank(bank: &[u32], cur_val: u64, cur_digits: usize, best: &mut u64) {
        if bank.len() + cur_digits < 12 {
            return;
        }

        if cur_digits == 12 {
            *best = (*best).max(cur_val);
            return;
        }

        // early exit if we can't do better than our best
        let mut max_achievable = cur_val;
        for _ in cur_digits..12 {
            max_achievable = max_achievable * 10 + 9;
        }

        if max_achievable >= *best {
            maximize_bank(
                &bank[1..],
                cur_val * 10 + (bank[0] as u64),
                cur_digits + 1,
                best,
            );
            maximize_bank(&bank[1..], cur_val, cur_digits, best);
        }
    }

    let mut sum = 0;
    for bank in input {
        let mut best = 0;
        maximize_bank(bank, 0, 0, &mut best);
        sum += best;
    }

    sum
}

fn main() {
    let parsed = parse_input(include_str!("input.txt").lines());

    println!("Part 1: {}", solve1(&parsed));
    println!("Part 2: {}", solve2(&parsed));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: [&str; 4] = [
        "987654321111111",
        "811111111111119",
        "234234234234278",
        "818181911112111",
    ];

    #[test]
    fn test1() {
        let parsed = parse_input(INPUT.iter().cloned());
        assert_eq!(solve1(&parsed), 357);
    }

    #[test]
    fn test2() {
        let parsed = parse_input(INPUT.iter().cloned());
        assert_eq!(solve2(&parsed), 3121910778619);
    }
}
