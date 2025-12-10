use std::{
    collections::{HashMap, VecDeque},
    rc::Rc,
};

use z3::{ast::Int, Optimize, SatResult};

#[derive(Debug)]
struct Machine {
    machine_size: u32,
    expected: u32,
    buttons: Vec<u32>,
    joltage: Vec<u32>,
}

fn parse_expected(part: &str) -> u32 {
    let mut expected = 0;

    assert_eq!(part.chars().next().unwrap(), '[');
    assert_eq!(part.chars().last().unwrap(), ']');

    for c in part.chars().skip(1).take(part.len() - 2) {
        match c {
            '.' => expected <<= 1,
            '#' => expected = (expected << 1) | 1,
            _ => panic!("Unexpected character in expected pattern"),
        }
    }
    expected
}

fn parse_button(part: &str, machine_len: u32) -> u32 {
    let mut button = 0;

    assert_eq!(part.chars().next().unwrap(), '(');
    assert_eq!(part.chars().last().unwrap(), ')');

    let button_numbers = part[1..part.len() - 1].split(',');

    for num_str in button_numbers {
        let num: u32 = num_str.parse().expect("Invalid button number");
        button += 1 << (machine_len - num - 1);
    }

    button
}

fn parse_joltage(part: &str) -> Vec<u32> {
    assert_eq!(part.chars().next().unwrap(), '{');
    assert_eq!(part.chars().last().unwrap(), '}');

    part[1..part.len() - 1]
        .split(',')
        .map(|num_str| num_str.trim().parse().expect("Invalid joltage number"))
        .collect()
}

fn parse_input(input: &str) -> Vec<Machine> {
    input
        .lines()
        .map(|line| {
            let parts: Vec<_> = line.split_whitespace().collect();
            let expected = parse_expected(parts[0]);
            let machine_size = parts[0].len() as u32 - 2;
            let buttons = parts[1..parts.len() - 1]
                .iter()
                .map(|&p| parse_button(p, machine_size))
                .collect();
            let joltage = parse_joltage(parts[parts.len() - 1]);
            Machine {
                machine_size,
                expected,
                buttons,
                joltage,
            }
        })
        .collect()
}

fn fewest_steps_on_machine(machine: &Machine) -> u32 {
    let mut steps_to: HashMap<u32, u32> = HashMap::new();
    let mut queue: VecDeque<(u32, u32)> = VecDeque::new();

    // Start from state 0 with 0 steps
    queue.push_back((0, 0));

    while let Some((state, steps)) = queue.pop_front() {
        if steps_to.contains_key(&state) {
            continue;
        }

        if state == machine.expected {
            return steps;
        }

        steps_to.insert(state, steps);

        for &button in &machine.buttons {
            let next_state = state ^ button;
            queue.push_back((next_state, steps + 1));
        }
    }

    panic!("No solution found for machine");
}

fn fewest_steps_on_machine_joltage(machine: &Machine) -> u32 {
    let mut joltage_changes: HashMap<_, Vec<Rc<Int>>> = HashMap::new();
    let mut minimize_button_presses = Vec::new();

    let solver = Optimize::new();

    for button in machine.buttons.iter() {
        let b_var = Rc::new(Int::new_const(format!("b_{}", button)));
        solver.assert(&(*b_var).ge(0));
        minimize_button_presses.push(b_var.clone());

        for i in 0..machine.machine_size {
            if (button & (1 << (machine.machine_size - i - 1))) != 0 {
                joltage_changes
                    .entry(i)
                    .or_insert_with(Vec::new)
                    .push(b_var.clone());
            }
        }
    }

    for (i, joltage) in machine.joltage.iter().enumerate() {
        let button_vars = joltage_changes.get(&(i as u32)).unwrap();

        solver.assert(
            &Int::add(
                &button_vars
                    .iter()
                    .map(|v| v.as_ref())
                    .collect::<Vec<&Int>>(),
            )
            .eq(*joltage),
        );
    }

    solver.minimize(&Int::add(
        &minimize_button_presses
            .iter()
            .map(|v| v.as_ref())
            .collect::<Vec<&Int>>(),
    ));

    match solver.check(&[]) {
        SatResult::Sat => {
            let model = solver.get_model().unwrap();
            let mut total_presses = 0;

            for b_var in minimize_button_presses.iter() {
                let presses = model.eval(b_var.as_ref(), true).unwrap().as_i64().unwrap();
                total_presses += presses as u32;
            }

            total_presses
        }
        _ => panic!("No solution found for machine with joltage"),
    }
}

pub fn run(input: &str) {
    let machines = parse_input(input);

    let part_1 = machines.iter().map(fewest_steps_on_machine).sum::<u32>();

    println!("Part 1: {}", part_1);

    let part_2 = machines
        .iter()
        .map(fewest_steps_on_machine_joltage)
        .sum::<u32>();

    println!("Part 2: {}", part_2);
}
