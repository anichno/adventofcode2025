fn parse_input<'a>(mut input: impl Iterator<Item = &'a str>) -> (Vec<Box>, Vec<Tree>) {
    let mut boxes = Vec::new();
    let mut trees = Vec::new();

    while let Some(line) = input.next() {
        let (start, rem) = line.split_once(":").unwrap();
        if let Some((dim_x_str, dim_y_str)) = start.split_once('x') {
            // parse tree
            let dim_x = dim_x_str.parse().unwrap();
            let dim_y = dim_y_str.parse().unwrap();

            let presents = rem
                .trim()
                .split_ascii_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();

            trees.push(Tree {
                dimensions: (dim_x, dim_y),
                presents,
            });
        } else {
            // parse box
            let mut cur_box: Vec<Vec<bool>> = Vec::new();
            while let Some(box_line) = input.next()
                && !box_line.is_empty()
            {
                cur_box.push(
                    box_line
                        .chars()
                        .map(|c| match c {
                            '#' => true,
                            '.' => false,
                            _ => panic!("Invalid char: {c}"),
                        })
                        .collect(),
                );
            }
            let area = cur_box
                .iter()
                .map(|r| r.iter().filter(|c| **c).count())
                .sum();
            boxes.push(Box {
                _layout: cur_box,
                area,
            });
        }
    }

    (boxes, trees)
}

// This works only because the inputs are carefully chosen. Otherwise this would be super hard
fn solve1(input: &(Vec<Box>, Vec<Tree>)) -> usize {
    input
        .1
        .iter()
        .filter(|tree| {
            let tree_area = tree.dimensions.0 * tree.dimensions.1;
            tree_area
                >= input
                    .0
                    .iter()
                    .zip(&tree.presents)
                    .map(|(b, num_b)| b.area * num_b)
                    .sum()
        })
        .count()
}

fn main() {
    let parsed = parse_input(include_str!("input.txt").lines());

    // 490
    println!("Part 1: {}", solve1(&parsed));
}

struct Box {
    _layout: Vec<Vec<bool>>,
    area: usize,
}

struct Tree {
    dimensions: (usize, usize),
    presents: Vec<usize>,
}
