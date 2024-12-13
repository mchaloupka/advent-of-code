use regex::Regex;

struct Machine {
    a_button_offset: (i64, i64),
    b_button_offset: (i64, i64),
    price_location: (i64, i64),
}

fn parse_input(input: &str) -> Vec<Machine> {
    let button_re = Regex::new(r"X\+(?P<x>\d+), Y\+(?P<y>\d+)").unwrap();
    let price_re = Regex::new(r"X=(?P<x>\d+), Y=(?P<y>\d+)").unwrap();

    let lines: Vec<&str> = input.lines().collect();

    lines
        .chunks(4)
        .map(|chunk| {
            let first_line = button_re.captures(chunk[0]).unwrap();
            let second_line = button_re.captures(chunk[1]).unwrap();
            let third_line = price_re.captures(chunk[2]).unwrap();

            Machine {
                a_button_offset: (
                    first_line["x"].parse().unwrap(),
                    first_line["y"].parse().unwrap(),
                ),
                b_button_offset: (
                    second_line["x"].parse().unwrap(),
                    second_line["y"].parse().unwrap(),
                ),
                price_location: (
                    third_line["x"].parse().unwrap(),
                    third_line["y"].parse().unwrap(),
                ),
            }
        })
        .collect()
}

fn get_steps(machine: &Machine) -> Option<i64> {
    // a * x_a + b * x_b = X
    // a * y_a + b * y_b = Y

    let x_a = machine.a_button_offset.0;
    let x_b = machine.b_button_offset.0;
    let y_a = machine.a_button_offset.1;
    let y_b = machine.b_button_offset.1;
    let x = machine.price_location.0;
    let y = machine.price_location.1;

    // Calculate determinant
    let det = x_a * y_b - x_b * y_a;

    if det == 0 {
        // No unique solution exists (determinant is zero).
        None
    } else {
        // Calculate a and b using Cramer's Rule
        let a_numerator = x * y_b - y * x_b;
        let b_numerator = y * x_a - x * y_a;

        if a_numerator % det == 0 && b_numerator % det == 0 {
            let a = a_numerator / det;
            let b = b_numerator / det;
            Some(a * 3 + b)
        } else {
            // No integer solution exists.
            None
        }
    }
}

fn part_1(input: &str) {
    let machines = parse_input(input);
    let mut total_count = 0;

    for machine in machines {
        if let Some(steps) = get_steps(&machine) {
            total_count += steps;
        }
    }

    println!("Part 1: {total_count}");
}

fn part_2(input: &str) {
    let machines: Vec<_> = parse_input(input)
        .into_iter()
        .map(|machine| Machine {
            a_button_offset: machine.a_button_offset,
            b_button_offset: machine.b_button_offset,
            price_location: (
                machine.price_location.0 + 10000000000000,
                machine.price_location.1 + 10000000000000,
            ),
        })
        .collect();

    let mut total_count = 0;

    for machine in machines {
        if let Some(steps) = get_steps(&machine) {
            total_count += steps;
        }
    }

    println!("Part 2: {total_count}");
}

pub fn run(input: &str) {
    part_1(input);
    part_2(input);
}
