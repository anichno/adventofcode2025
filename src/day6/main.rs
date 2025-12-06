#[derive(Debug)]
enum Operation {
    Add,
    Mult,
}

mod part1 {
    use super::Operation;

    pub fn parse_input<'a>(
        input: impl Iterator<Item = &'a str>,
    ) -> (Vec<Vec<i64>>, Vec<Operation>) {
        let mut number_lines = Vec::new();
        let mut operations = Vec::new();

        for line in input {
            if line.trim().chars().next().unwrap().is_ascii_digit() {
                number_lines.push(
                    line.split_ascii_whitespace()
                        .map(|s| s.parse().unwrap())
                        .collect(),
                );
            } else {
                operations = line
                    .split_ascii_whitespace()
                    .map(|o| match o {
                        "+" => Operation::Add,
                        "*" => Operation::Mult,
                        _ => panic!("Invalid operation: {o}"),
                    })
                    .collect();
            }
        }

        (number_lines, operations)
    }

    pub fn solve1(input: &(Vec<Vec<i64>>, Vec<Operation>)) -> i64 {
        let mut total = 0;

        for i in 0..input.1.len() {
            let vals = input.0.iter().map(|inp| inp[i]);
            total += match input.1[i] {
                Operation::Add => vals.sum::<i64>(),
                Operation::Mult => vals.product::<i64>(),
            };
        }

        total
    }
}

fn solve2<'a>(input: impl Iterator<Item = &'a str>) -> i64 {
    let mut grid: Vec<Vec<char>> = Vec::new();
    for line in input {
        grid.push(line.chars().collect());
    }

    let operations_line = {
        let mut operations_line = grid.pop().unwrap();
        operations_line.push('!');
        operations_line
    };

    let mut total = 0;

    let mut cur_operation = Operation::Add;
    let mut cur_nums = Vec::new();
    for (i, chr) in operations_line.into_iter().enumerate() {
        if chr != ' ' {
            // eval accumulated nums
            total += match cur_operation {
                Operation::Add => cur_nums.iter().sum::<i64>(),
                Operation::Mult => cur_nums.iter().product::<i64>(),
            };

            cur_operation = match chr {
                '+' => Operation::Add,
                '*' => Operation::Mult,
                '!' => break,
                _ => panic!("invalid operation: {chr}"),
            };
            cur_nums.clear();
        }

        let mut num = 0;
        for row in &grid {
            let cur_num_chr = row[i];
            if cur_num_chr.is_ascii_digit() {
                num = num * 10 + cur_num_chr.to_digit(10).unwrap() as i64;
            }
        }

        if num != 0 {
            cur_nums.push(num);
        }
    }

    total
}

fn main() {
    let parsed = part1::parse_input(include_str!("input.txt").lines());

    println!("Part 1: {}", part1::solve1(&parsed));
    println!("Part 2: {}", solve2(include_str!("input.txt").lines()));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: [&str; 4] = [
        "123 328  51 64 ",
        " 45 64  387 23 ",
        "  6 98  215 314",
        "*   +   *   +  ",
    ];

    #[test]
    fn test1() {
        let parsed = part1::parse_input(INPUT.iter().cloned());
        assert_eq!(part1::solve1(&parsed), 4277556);
    }

    #[test]
    fn test2() {
        assert_eq!(solve2(INPUT.iter().cloned()), 3263827);
    }
}
