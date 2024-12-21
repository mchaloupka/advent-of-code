use cached::proc_macro::cached;
use cached::SizedCache;

type Coord = (usize, usize);
type Keyboard = [[char; 3]];

fn find_symbol(s: char, keyboard: &Keyboard) -> Coord {
    let mut pos = None;

    for (x, row) in keyboard.iter().enumerate() {
        for (y, &c) in row.iter().enumerate() {
            if c == s {
                pos = Some((x, y));
            }
        }
    }

    pos.unwrap()
}

fn find_all_shortest_paths_to_symbol(pos: Coord, target: Coord, keyboard: &Keyboard) -> Vec<String> {
    if pos == target {
        vec![String::from("A")]
    } else {
        let mut nexts = Vec::new();

        if pos.1 < target.1 && keyboard[pos.0][pos.1 + 1] != '#' {
            nexts.push(((pos.0, pos.1 + 1), '>'));
        } 
        if pos.0 < target.0 && keyboard[pos.0 + 1][pos.1] != '#' {
            nexts.push(((pos.0 + 1, pos.1), 'v'));
        } 
        if pos.1 > target.1 && keyboard[pos.0][pos.1 - 1] != '#' {
            nexts.push(((pos.0, pos.1 - 1), '<'));
        } 
        if pos.0 > target.0 && keyboard[pos.0 - 1][pos.1] != '#' {
            nexts.push(((pos.0 - 1, pos.1), '^'));
        } 
        if nexts.is_empty() {
            panic!("Not found movement");
        }

        let mut output = Vec::new();

        for (next_pos, movement) in nexts {
            let mut next_outputs = find_all_shortest_paths_to_symbol(next_pos, target, keyboard);

            for next_output in next_outputs.iter_mut() {
                let mut final_output = String::new();
                final_output.push(movement);
                final_output += next_output;
                output.push(final_output);
            }
        }
        
        output
    } 
}

static NUMERIC_KEYBOARD: [[char; 3]; 4] = [
    ['7', '8', '9'],
    ['4', '5', '6'],
    ['1', '2', '3'],
    ['#', '0', 'A'],
];

static MOVEMENT_KEYBOARD: [[char; 3]; 2] = [
    ['#', '^', 'A'],
    ['<', 'v', '>'],
];

fn get_shortest_sequence(input: String, is_numeric: bool, transforms: usize) -> Vec<String> {
    if transforms == 0 {
        return vec![input];
    }
    
    let mut last_c = 'A';
    let mut outputs = vec![String::new()];

    for input_c in input.chars() {
        let mut new_outputs =  Vec::new();

        let possible_paths = get_shortest_paths(last_c, input_c, is_numeric, transforms);

        for possible_path in possible_paths {
            for output in outputs.iter() {
                let mut new_output = output.clone();
                new_output += possible_path.as_str();
                new_outputs.push(new_output);
            }
        }

        outputs = new_outputs;
        last_c = input_c;
    }

    outputs
}

#[cached]
fn get_shortest_paths(from: char, to: char, is_numeric: bool, transforms: usize) -> Vec<String> {
    let keyboard: &[[char; 3]] = match is_numeric {
        true => &NUMERIC_KEYBOARD,
        false => &MOVEMENT_KEYBOARD,
    };

    let from_pos = find_symbol(from, keyboard);
    let to_pos = find_symbol(to, keyboard);

    let mut output = Vec::new();
    let mut shortest_length = None;

    for variant in find_all_shortest_paths_to_symbol(from_pos, to_pos, keyboard) {
        for shortest_nested in get_shortest_sequence(variant, false, transforms - 1) {
            let cur_length = shortest_nested.len();

            match shortest_length {
                // Ignore if the found is longer
                Some(x) if x < cur_length => { },
                // Add if the found is minimum length
                Some(x) if x == cur_length => { output.push(shortest_nested); },
                // Replace if the found is the shorter than any other found
                _ => { 
                    output = vec![shortest_nested];
                    shortest_length = Some(cur_length);
                }
            };
        }
    }

    output
}

fn calculate_part(input: &str, transforms: usize) -> usize {
    let mut total = 0;

    for line in input.lines() {
        let numerical: usize = line[0..line.len() - 1].parse().unwrap();
        let shortest_sequences = get_shortest_sequence(String::from(line), true, transforms);
        total += numerical * shortest_sequences[0].len();
    }

    total
}

fn part_1(input: &str) {
    println!("Part 1: {}", calculate_part(input, 3));
}

fn part_2(input: &str) {
    println!("Part 2: {}", calculate_part(input, 26));
}

pub fn run(input: &str) {
    part_1(input);
    part_2(input);
}