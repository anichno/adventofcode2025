use z3::ast::Int;

#[derive(Debug, Clone)]
struct Machine {
    goal: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    joltage_req: Vec<usize>,
}

fn parse_input<'a>(input: impl Iterator<Item = &'a str>) -> Vec<Machine> {
    let mut machines = Vec::new();
    for line in input {
        let (indicators, rem) = line.split_once("] ").unwrap();

        let mut goal = Vec::new();
        for c in indicators.chars().skip(1) {
            match c {
                '.' => goal.push(false),
                '#' => goal.push(true),
                _ => panic!("Invalid indicator: {c}"),
            }
        }

        let (buttons_str, rem) = rem.split_once(" {").unwrap();

        let mut buttons = Vec::new();
        for button_str in buttons_str.split_ascii_whitespace() {
            let button_str = button_str.trim_matches(['(', ')']);
            let button = button_str.split(",").map(|s| s.parse().unwrap()).collect();
            buttons.push(button);
        }

        let joltage_req = rem
            .trim_matches('}')
            .split(',')
            .map(|s| s.parse().unwrap())
            .collect();

        machines.push(Machine {
            goal,
            buttons,
            joltage_req,
        })
    }
    machines
}

fn solve1(input: &[Machine]) -> u64 {
    let mut tot_num_presses = 0;
    for machine in input {
        let solver = z3::Optimize::new();

        let buttons: Vec<Int> = (0..machine.buttons.len())
            .map(|_b| {
                let btn = Int::new_const(format!("btn_{_b}"));
                solver.assert(&btn.ge(0));
                btn
            })
            .collect();

        for (g_idx, g) in machine.goal.iter().enumerate() {
            let mut indicator = Int::from_u64(0);
            for (btn_var, btn) in buttons.iter().zip(&machine.buttons) {
                if btn.contains(&g_idx) {
                    indicator += btn_var;
                }
            }

            let result = if *g { 1 } else { 0 };

            solver.assert(&indicator.rem(2).eq(result));
        }

        let tot_presses: Int = buttons.iter().sum();
        solver.minimize(&tot_presses);

        let _sat = solver.check(&[]);
        let model = solver.get_model().unwrap();
        let presses = model.eval(&tot_presses, true).unwrap().as_u64().unwrap();
        tot_num_presses += presses;
    }
    tot_num_presses
}

fn solve2(input: &[Machine]) -> u64 {
    let mut tot_num_presses = 0;
    for machine in input {
        let solver = z3::Optimize::new();

        let buttons: Vec<Int> = (0..machine.buttons.len())
            .map(|_b| {
                let btn = Int::new_const(format!("btn_{_b}"));
                solver.assert(&btn.ge(0));
                btn
            })
            .collect();

        for (g_idx, g) in machine.joltage_req.iter().enumerate() {
            let mut indicator = Int::from_u64(0);
            for (btn_var, btn) in buttons.iter().zip(&machine.buttons) {
                if btn.contains(&g_idx) {
                    indicator += btn_var;
                }
            }

            solver.assert(&indicator.eq(*g as i32));
        }

        let tot_presses: Int = buttons.iter().sum();
        solver.minimize(&tot_presses);

        let _sat = solver.check(&[]);
        let model = solver.get_model().unwrap();
        let presses = model.eval(&tot_presses, true).unwrap().as_u64().unwrap();
        tot_num_presses += presses;
    }
    tot_num_presses
}

fn main() {
    let parsed = parse_input(include_str!("input.txt").lines());

    println!("Part 1: {}", solve1(&parsed));
    println!("Part 2: {}", solve2(&parsed));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: [&str; 3] = [
        "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}",
        "[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}",
        "[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}",
    ];

    #[test]
    fn test1() {
        let parsed = parse_input(INPUT.iter().cloned());
        assert_eq!(solve1(&parsed), 7);
    }

    #[test]
    fn test2() {
        let parsed = parse_input(INPUT.iter().cloned());
        assert_eq!(solve2(&parsed), 33);
    }
}
