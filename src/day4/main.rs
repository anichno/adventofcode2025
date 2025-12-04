use std::ops::Deref;

#[derive(Debug, Clone, Copy)]
enum Space {
    Empty,
    Paper,
}

fn parse_input<'a>(input: impl Iterator<Item = &'a str>) -> Vec<Vec<Space>> {
    input
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => Space::Empty,
                    '@' => Space::Paper,
                    _ => panic!("Invalid char {c}"),
                })
                .collect()
        })
        .collect()
}

fn grid_8_neighbors<'a, T: 'a, S>(grid: &'a [S], x: usize, y: usize) -> impl Iterator<Item = &'a T>
where
    S: Deref<Target = [T]>,
{
    (-1..=1)
        .filter_map(move |y_diff| {
            y.checked_add_signed(y_diff)
                .filter(|&new_y| new_y < grid.len())
        })
        .flat_map(move |new_y| {
            (-1..=1).filter_map(move |x_diff| {
                if let Some(new_x) = x.checked_add_signed(x_diff) {
                    if new_x < grid[new_y].len() && !(new_x == x && new_y == y) {
                        Some(&grid[new_y][new_x])
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
        })
}

fn solve1(input: &[Vec<Space>]) -> i64 {
    let mut total = 0;

    for (y, row) in input.iter().enumerate() {
        for (x, _col) in row
            .iter()
            .enumerate()
            .filter(|c| matches!(c.1, Space::Paper))
        {
            if grid_8_neighbors(input, x, y)
                .filter(|neighbor| matches!(**neighbor, Space::Paper))
                .count()
                < 4
            {
                total += 1;
            }
        }
    }

    total
}

fn solve2(input: &[Vec<Space>]) -> i64 {
    let mut cur_grid: Vec<Vec<Space>> = Vec::from_iter(input.iter().cloned());
    let mut total = 0;

    loop {
        let mut round_total = 0;
        let mut new_grid = Vec::new();
        for (y, row) in cur_grid.iter().enumerate() {
            let mut new_row = Vec::new();
            for (x, col) in row.iter().enumerate() {
                match col {
                    Space::Empty => new_row.push(Space::Empty),
                    Space::Paper => {
                        if grid_8_neighbors(&cur_grid, x, y)
                            .filter(|neighbor| matches!(**neighbor, Space::Paper))
                            .count()
                            < 4
                        {
                            round_total += 1;
                            new_row.push(Space::Empty);
                        } else {
                            new_row.push(Space::Paper)
                        }
                    }
                }
            }
            new_grid.push(new_row);
        }

        if round_total == 0 {
            break;
        } else {
            total += round_total;
            cur_grid = new_grid;
        }
    }

    total
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
        "..@@.@@@@.",
        "@@@.@.@.@@",
        "@@@@@.@.@@",
        "@.@@@@..@.",
        "@@.@@@@.@@",
        ".@@@@@@@.@",
        ".@.@.@.@@@",
        "@.@@@.@@@@",
        ".@@@@@@@@.",
        "@.@.@@@.@.",
    ];

    #[test]
    fn test1() {
        let parsed = parse_input(INPUT.iter().cloned());
        assert_eq!(solve1(&parsed), 13);
    }

    #[test]
    fn test2() {
        let parsed = parse_input(INPUT.iter().cloned());
        assert_eq!(solve2(&parsed), 43);
    }
}
