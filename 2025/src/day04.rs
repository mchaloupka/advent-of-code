use std::collections::VecDeque;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Entry {
    Roll,
    Nothing,
}

fn parse_entry(c: char) -> Entry {
    match c {
        '.' => Entry::Nothing,
        '@' => Entry::Roll,
        _ => panic!("Invalid entry"),
    }
}

fn get_adjacent_positions(x: usize, y: usize, width: usize, height: usize) -> Vec<(usize, usize)> {
    let mut result = Vec::new();

    let x_i64 = i64::try_from(x).unwrap();
    let y_i64 = i64::try_from(y).unwrap();

    for xi_i64 in x_i64 - 1..=x_i64 + 1 {
        for yi_i64 in y_i64 - 1..=y_i64 + 1 {
            if xi_i64 < 0 || yi_i64 < 0 {
                continue;
            }

            let xi = usize::try_from(xi_i64).unwrap();
            let yi = usize::try_from(yi_i64).unwrap();

            if xi == x && yi == y {
                continue;
            }
            if xi >= width || yi >= height {
                continue;
            }

            result.push((xi, yi));
        }
    }

    result
}

pub fn run(input: &str) {
    let matrix: Vec<Vec<_>> = input
        .lines()
        .map(|line| line.chars().map(parse_entry).collect())
        .collect();

    let height = matrix.len();
    let width = matrix[0].len();

    let mut part_1 = 0;

    for y in 0..height {
        for x in 0..width {
            if Entry::Roll != matrix[y][x] {
                continue;
            }

            let count_rolls = get_adjacent_positions(x, y, width, height)
                .iter()
                .filter(|(x, y)| Entry::Roll == matrix[*y][*x])
                .count();

            if count_rolls < 4 {
                part_1 += 1;
            }
        }
    }

    println!("Part 1: {}", part_1);

    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
    let mut matrix_state = matrix.clone();

    for (y, matrix_row) in matrix_state.iter().enumerate() {
        for (x, entry) in matrix_row.iter().enumerate() {
            if Entry::Roll == *entry {
                queue.push_back((x, y));
            }
        }
    }

    let mut part_2 = 0;

    while let Some((x, y)) = queue.pop_front() {
        if Entry::Roll != matrix_state[y][x] {
            continue;
        }

        let adjacent = get_adjacent_positions(x, y, width, height);
        let adjacent_rolls = adjacent
            .iter()
            .filter(|(x, y)| Entry::Roll == matrix_state[*y][*x])
            .count();

        if adjacent_rolls < 4 {
            part_2 += 1;
            matrix_state[y][x] = Entry::Nothing;
            for (x, y) in adjacent {
                if Entry::Roll == matrix_state[y][x] {
                    queue.push_back((x, y));
                }
            }
        }
    }

    println!("Part 2: {}", part_2);
}
