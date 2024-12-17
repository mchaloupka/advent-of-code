use regex::Regex;

#[derive(Debug, Clone, Copy)]
struct MachineState {
    a: u64,
    b: u64,
    c: u64,
    pos: usize,
}

type Instructions = Vec<u64>;

#[derive(Debug, Clone)]
struct Machine {
    state: MachineState,
    instructions: Instructions,
}

fn parse_input(input: &str) -> Machine {
    let lines: Vec<_> = input.lines().collect();
    let register_re = Regex::new(r"Register .: (?P<value>-?\d+)").unwrap();
    let register_a = register_re.captures(lines[0]).unwrap()["value"]
        .parse()
        .unwrap();
    let register_b = register_re.captures(lines[1]).unwrap()["value"]
        .parse()
        .unwrap();
    let register_c = register_re.captures(lines[2]).unwrap()["value"]
        .parse()
        .unwrap();

    let instructions = lines[4]
        .split_once(":")
        .unwrap()
        .1
        .trim()
        .split(",")
        .map(|num| num.parse().unwrap())
        .collect();

    Machine {
        state: MachineState {
            a: register_a,
            b: register_b,
            c: register_c,
            pos: 0,
        },
        instructions,
    }
}

fn run_program(machine_state: &mut MachineState, instructions: &Instructions) -> Vec<u64> {
    let mut output = Vec::new();

    while machine_state.pos < instructions.len() {
        let literal = instructions[machine_state.pos + 1];
        let combo = match literal {
            0 => 0,
            1 => 1,
            2 => 2,
            3 => 3,
            4 => machine_state.a,
            5 => machine_state.b,
            6 => machine_state.c,
            _ => panic!("Invalid operand"),
        };

        match instructions[machine_state.pos] {
            0 => {
                // adv
                machine_state.a >>= combo;
            }
            1 => {
                // bxl
                machine_state.b ^= instructions[machine_state.pos + 1];
            }
            2 => {
                // bst
                machine_state.b = combo & 0b111;
            }
            3 => {
                // jnz
                if machine_state.a != 0 {
                    machine_state.pos = literal as usize;
                    continue; // Prevent instruction increase
                }
            }
            4 => {
                // bxc
                machine_state.b ^= machine_state.c;
            }
            5 => {
                // out
                output.push(combo & 0b111);
            }
            6 => {
                // bdv
                machine_state.b = machine_state.a >> combo;
            }
            7 => {
                // bdv
                machine_state.c = machine_state.a >> combo;
            }
            _ => panic!("Invalid instruction"),
        }

        machine_state.pos += 2;
    }

    output
}

fn part_1(input: &str) {
    let machine = parse_input(input);
    let mut machine_state = machine.state;

    let output = run_program(&mut machine_state, &machine.instructions);

    println!(
        "Part 1: {}",
        output
            .into_iter()
            .map(|i| i.to_string())
            .collect::<Vec<_>>()
            .join(",")
    );
}

fn part_2(input: &str) {
    let machine = parse_input(input);

    // Seems that the machine output is based on the individual bytes of
    // the register A. So, we try to find the register A byte by byte. Seems
    // like the first byte of the number results in last number of the output.
    // And this way it iterates in loop.

    let mut oct_factors = vec![0; machine.instructions.len()];

    loop {
        let mut init_a = 0;

        for (i, f) in oct_factors.iter().enumerate() {
            init_a += 8u64.pow(i as u32) * f;
        }

        let mut machine_state = MachineState {
            a: init_a,
            b: 0,
            c: 0,
            pos: 0,
        };

        let output = run_program(&mut machine_state, &machine.instructions);

        if output == machine.instructions {
            println!("Part 2: {}", init_a);
            break;
        }

        // We go from last instructions to find the highest bytes first
        for i in (0..machine.instructions.len()).rev() {
            if output.len() < i {
                oct_factors[i] += 1;
                break;
            }
            if output[i] != machine.instructions[i] {
                oct_factors[i] += 1;
                break;
            }
        }
    }
}

pub fn run(input: &str) {
    part_1(input);
    part_2(input);
}
