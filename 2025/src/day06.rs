#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Operator {
    Plus,
    Multiply,
}

fn parse_operator(op: char) -> Operator {
    match op {
        '+' => Operator::Plus,
        '*' => Operator::Multiply,
        _ => panic!("Invalid operator"),
    }
}

fn parse_input_1(input: &str) -> Vec<(Operator, Vec<usize>)> {
    let lines = input.lines().collect::<Vec<&str>>();
    let number_rows: Vec<Vec<usize>> = lines[..lines.len() - 1]
        .iter()
        .map(|line| {
            line.split_whitespace()
                .map(|number| number.parse().unwrap())
                .collect()
        })
        .collect();

    let operator_row: Vec<Operator> = lines[lines.len() - 1]
        .split_whitespace()
        .map(|operator| parse_operator(operator.chars().next().unwrap()))
        .collect();

    let mut result: Vec<(Operator, Vec<usize>)> = Vec::new();

    for (i, operator) in operator_row.iter().enumerate() {
        let numbers: Vec<usize> = number_rows.iter().map(|number_row| number_row[i]).collect();

        result.push((*operator, numbers));
    }

    result
}

fn process_section(
    from: usize,
    to: usize,
    lines: &Vec<&str>,
    result: &mut Vec<(Operator, Vec<usize>)>,
) {
    let last_line = lines.last().unwrap();
    let operator = parse_operator(last_line.chars().nth(from).unwrap());
    let mut numbers: Vec<usize> = Vec::new();

    for i in from..to {
        let mut s = String::new();

        for line in lines.iter().take(lines.len() - 1) {
            let c = line.chars().nth(i).unwrap();
            if c.is_whitespace() {
                continue;
            }

            s.push(c);
        }

        if s.is_empty() {
            continue;
        }

        numbers.push(s.parse().unwrap());
    }

    result.push((operator, numbers));
}

fn parse_input_2(input: &str) -> Vec<(Operator, Vec<usize>)> {
    let lines = input.lines().collect::<Vec<&str>>();
    let mut result: Vec<(Operator, Vec<usize>)> = Vec::new();

    let mut cur_index = 0;
    let last_line = lines.last().unwrap();

    for (i, c) in last_line.chars().enumerate() {
        if c.is_whitespace() {
            continue;
        } else {
            if i > 0 {
                process_section(cur_index, i, &lines, &mut result);
            }

            cur_index = i;
        }
    }

    process_section(cur_index, last_line.len(), &lines, &mut result);

    result
}

pub fn run(input: &str) {
    let part_1 = parse_input_1(input)
        .iter()
        .map(|(operator, numbers)| match operator {
            Operator::Plus => numbers.iter().sum::<usize>(),
            Operator::Multiply => numbers.iter().product::<usize>(),
        })
        .sum::<usize>();

    println!("Part 1: {}", part_1);

    let part_2 = parse_input_2(input)
        .iter()
        .map(|(operator, numbers)| match operator {
            Operator::Plus => numbers.iter().sum::<usize>(),
            Operator::Multiply => numbers.iter().product::<usize>(),
        })
        .sum::<usize>();

    println!("Part 2: {}", part_2);
}
