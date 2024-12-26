use std::collections::HashMap;

use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Oper {
    And,
    Xor,
    Or,
}

impl Oper {
    fn exec(&self, l: bool, r: bool) -> bool {
        match self {
            Oper::And => l && r,
            Oper::Xor => l != r,
            Oper::Or => l || r,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Gate<'a> {
    l: &'a str,
    r: &'a str,
    op: Oper,
}

struct Input<'a> {
    inputs: HashMap<&'a str, bool>,
    operations: HashMap<&'a str, Gate<'a>>,
}

impl<'a> Input<'a> {
    fn ensure_input_set(&mut self, input: &'a str) -> bool {
        if !self.inputs.contains_key(input) {
            let gate = *self
                .operations
                .get(input)
                .expect("Missing gate for operand");

            let l = self.ensure_input_set(gate.l);
            let r = self.ensure_input_set(gate.r);
            let output = gate.op.exec(l, r);

            self.inputs.insert(input, output);
        }

        *self.inputs.get(input).unwrap()
    }

    fn swap_wires(&mut self, a: &'a str, b: &'a str) {
        let temp = self.operations[a];
        self.operations.insert(a, self.operations[b]);
        self.operations.insert(b, temp);
    }

    fn get_number_with_prefix(&mut self, prefix: &str) -> u128 {
        let wires: Vec<_> = self
            .operations
            .keys()
            .filter(|k| k.starts_with(prefix))
            .sorted()
            .rev()
            .cloned()
            .collect();

        let mut output: u128 = 0;

        for wire in wires {
            if output > 0 {
                output <<= 1;
            }

            if self.ensure_input_set(wire) {
                output += 1;
            }
        }

        output
    }
}

fn parse_input(input: &str) -> Input<'_> {
    let mut output = Input {
        inputs: HashMap::new(),
        operations: HashMap::new(),
    };

    for line in input.lines() {
        if line.contains(":") {
            let (wire, raw_val) = line.split_once(":").unwrap();
            let num_val = raw_val.trim().parse::<u8>().unwrap();
            output.inputs.insert(wire, num_val == 1);
        } else if line.contains("->") {
            let parts: Vec<_> = line.split_whitespace().collect();
            let l = parts[0];
            let r = parts[2];
            let o = parts[4];

            let op = match parts[1] {
                "AND" => Oper::And,
                "OR" => Oper::Or,
                "XOR" => Oper::Xor,
                _ => panic!("Unexpected operator"),
            };

            output.operations.insert(o, Gate { l, r, op });
        }
    }

    output
}

fn part_1(input: &str) {
    let mut data = parse_input(input);
    println!("Part 1: {}", data.get_number_with_prefix("z"));
}

fn make_wire_id(prefix: &str, bit: u8) -> String {
    format!("{}{:0>2}", prefix, bit)
}

fn matches_gate_on_x_y(data: &Input, wire: &str, bit: u8, op: Oper) -> bool {
    match data.operations.get(wire) {
        Some(gate) if gate.op == op => {
            let mut operands = [gate.l, gate.r];
            operands.sort();
            operands == [make_wire_id("x", bit), make_wire_id("y", bit)]
        }
        _ => false,
    }
}

fn is_ok_xor(data: &Input, wire: &str, bit: u8) -> bool {
    matches_gate_on_x_y(data, wire, bit, Oper::Xor)
}

fn is_ok_direct_carry(data: &Input, wire: &str, bit: u8) -> bool {
    // Direct carry happens when both underlying bits are set
    matches_gate_on_x_y(data, wire, bit, Oper::And)
}

fn is_ok_carryover(data: &Input, wire: &str, bit: u8) -> bool {
    // Carry over happens when one bit is not set on the inputs and there is one bit carried over
    match data.operations.get(wire) {
        Some(gate) if gate.op == Oper::And => {
            (is_ok_xor(data, gate.l, bit) && is_ok_carry(data, gate.r, bit))
                || (is_ok_xor(data, gate.r, bit) && is_ok_carry(data, gate.l, bit))
        }
        _ => false,
    }
}

fn is_ok_carry(data: &Input, wire: &str, bit: u8) -> bool {
    // If it is the second bit, then the carryover is x0 AND y0
    if bit == 1 {
        matches_gate_on_x_y(data, wire, bit - 1, Oper::And)
    } else {
        // Otherwise it is either dirrect carry (both previous bits are 1 on inputs), or it is carried over previous bit
        match data.operations.get(wire) {
            Some(gate) if matches!(gate.op, Oper::Or) => {
                (is_ok_direct_carry(data, gate.l, bit - 1)
                    && is_ok_carryover(data, gate.r, bit - 1))
                    || (is_ok_direct_carry(data, gate.r, bit - 1)
                        && is_ok_carryover(data, gate.l, bit - 1))
            }
            _ => false,
        }
    }
}

fn is_z_wire_correct(data: &Input, bit: u8) -> bool {
    let z_wire = make_wire_id("z", bit);

    // First bit is a simple XOR on the two input bits
    if bit == 0 {
        is_ok_xor(data, z_wire.as_str(), bit)
    } else if let Some(gate) = data.operations.get(z_wire.as_str()) {
        // All other bits are calculated using: ((Xn XOR Yn) XOR Cn) where Cn is carry bit contributed to bit n
        if matches!(gate.op, Oper::Xor) {
            (is_ok_xor(data, gate.l, bit) && is_ok_carry(data, gate.r, bit))
                || (is_ok_xor(data, gate.r, bit) && is_ok_carry(data, gate.l, bit))
        } else {
            false
        }
    } else {
        false
    }
}

fn bits_correct(data: &Input) -> u8 {
    for bit in 0..128 {
        if !is_z_wire_correct(data, bit) {
            return bit;
        }
    }

    128
}

fn part_2(input: &str) {
    let mut data = parse_input(input);
    let mut swaps = Vec::new();

    let wires: Vec<_> = data.operations.keys().copied().collect();

    for _ in 0..4 {
        let baseline = bits_correct(&data);

        for (&a, &b) in wires.iter().tuple_combinations() {
            data.swap_wires(a, b);

            if bits_correct(&data) > baseline {
                swaps.push([a, b]);
                break;
            }

            data.swap_wires(a, b);
        }
    }

    let output: String =
        Itertools::intersperse(swaps.into_iter().flatten().sorted(), ",").collect();

    println!("Part 2: {}", output);
}

pub fn run(input: &str) {
    part_1(input);
    part_2(input);
}
