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

    // The input program can be rewritten as:
    // 0: b = a & 0b111
    // 2: b ^= 0b1
    // 4: c = a >> b
    // 6: b ^= 0b101
    // 8: b ^= c
    // 10: print b & 0b111
    // 12: shift a to right by 3 bits
    // 14: jmp 0 if a > 0
    //
    // Important observation is that the output is always processed by 3 bits and the only thing
    // we do with A is to shift it by 3 bits to the right. So, output N is based on A shifted N*3 times
    // to right. Therefore, for each output the A has 3 bits set to some value. And we can find
    // them from highest 3-bits as the output ignores all lower bits.

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
