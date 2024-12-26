type Heights = Vec<Vec<usize>>;

#[derive(Debug, Clone)]
struct Input {
    keys: Heights,
    locks: Heights,
    height: usize,
    length: usize,
}

fn parse_block(input: &Vec<&str>, output: &mut Option<Input>) -> Input {
    let height = input.len();
    let length = input[0].len();

    let leading_char = if input[0].chars().all(|x| x == '.') {
        '.'
    } else if input[0].chars().all(|x| x == '#') {
        '#'
    } else {
        panic!("Not sure whether it is key or char");
    };

    let mut heights = vec![0; length];

    for row in input {
        for (idx, x) in row.chars().enumerate() {
            if x == leading_char {
                heights[idx] += 1;
            }
        }
    }

    let inv_heights = heights.iter().map(|h| height - h).collect();

    match (leading_char, output) {
        ('.', None) => Input {
            height,
            length,
            keys: vec![inv_heights],
            locks: Vec::new(),
        },
        ('#', None) => Input {
            height,
            length,
            keys: Vec::new(),
            locks: vec![heights],
        },
        ('.', Some(prev)) => {
            prev.keys.push(inv_heights);
            prev.clone()
        }
        ('#', Some(prev)) => {
            prev.locks.push(heights);
            prev.clone()
        }
        _ => panic!("Unexpected case"),
    }
}

fn parse_input(input: &str) -> Input {
    let mut output = None;
    let mut lines_block = Vec::new();

    for line in input.lines() {
        if line.is_empty() {
            output = Some(parse_block(&lines_block, &mut output));
            lines_block.clear();
        } else {
            lines_block.push(line);
        }
    }

    output = Some(parse_block(&lines_block, &mut output));

    output.unwrap()
}

fn part_1(input: &str) {
    let data = parse_input(input);

    let mut valid_pairs = 0;

    for lock in data.locks.iter() {
        for key in data.keys.iter() {
            let mut is_valid = true;

            for idx in 0..data.length {
                if lock[idx] + key[idx] > data.height {
                    is_valid = false;
                    break;
                }
            }

            if is_valid {
                valid_pairs += 1;
            }
        }
    }

    println!("Part 1: {}", valid_pairs);
}

fn part_2() {
    println!("Part 2: No work this time");
}

pub fn run(input: &str) {
    part_1(input);
    part_2();
}
