fn parse_input<'a>(input: impl Iterator<Item = &'a str>) -> Vec<(i64, i64)> {
    input
        .map(|line| {
            let (left, right) = line.split_once(",").unwrap();
            (left.parse().unwrap(), right.parse().unwrap())
        })
        .collect()
}

fn solve1(input: &[(i64, i64)]) -> i64 {
    let mut max_area = 0;
    for (a_idx, a) in input.iter().enumerate() {
        for b in input.iter().skip(a_idx + 1) {
            max_area = max_area.max(((a.0 - b.0).abs() + 1) * ((a.1 - b.1).abs() + 1));
        }
    }
    max_area
}

fn solve2(input: &[(i64, i64)]) -> i64 {
    use geo::{Coord, Covers, LineString, Polygon, Rect};

    let coords = input
        .iter()
        .map(|(x, y)| Coord {
            x: *x as f64,
            y: *y as f64,
        })
        .collect();
    let outer_shape = Polygon::new(LineString::new(coords), vec![]);

    let mut max_area = 0;
    for (a_idx, a) in input.iter().enumerate() {
        for b in input.iter().skip(a_idx + 1) {
            let rect = Rect::new(
                Coord {
                    x: a.0 as f64,
                    y: a.1 as f64,
                },
                Coord {
                    x: b.0 as f64,
                    y: b.1 as f64,
                },
            );
            let test_polygon = rect.to_polygon();
            if outer_shape.covers(&test_polygon) {
                max_area = max_area.max(((a.0 - b.0).abs() + 1) * ((a.1 - b.1).abs() + 1));
            }
        }
    }
    max_area
}

fn main() {
    let parsed = parse_input(include_str!("input.txt").lines());

    println!("Part 1: {}", solve1(&parsed));
    println!("Part 2: {}", solve2(&parsed));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: [&str; 8] = ["7,1", "11,1", "11,7", "9,7", "9,5", "2,5", "2,3", "7,3"];

    #[test]
    fn test1() {
        let parsed = parse_input(INPUT.iter().cloned());
        assert_eq!(solve1(&parsed), 50);
    }

    #[test]
    fn test2() {
        let parsed = parse_input(INPUT.iter().cloned());
        assert_eq!(solve2(&parsed), 24);
    }
}
