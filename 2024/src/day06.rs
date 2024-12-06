use std::collections::HashSet;

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

type Coord = (usize, usize);
type Position = (Coord, Direction);
type Matrix = Vec<Vec<bool>>;

fn parse_input(input: &str) -> (Matrix, Position) {
    let mut output: Matrix = Vec::new();
    let mut start: Option<Position> = None;

    for (row, line) in input.lines().enumerate() {
        output.push(
            line.chars()
                .enumerate()
                .map(|(col, x)| match x {
                    '#' => true,
                    '^' => {
                        start = Some(((row, col), Direction::Up));
                        false
                    }
                    '>' => {
                        start = Some(((row, col), Direction::Right));
                        false
                    }
                    'v' => {
                        start = Some(((row, col), Direction::Down));
                        false
                    }
                    '<' => {
                        start = Some(((row, col), Direction::Left));
                        false
                    }
                    _ => false,
                })
                .collect(),
        );
    }

    (output, start.expect("Have not found start"))
}

fn next_pos(map: &Matrix, mut current_pos: Position) -> (bool, Position) {
    let mut is_next_out: bool;

    loop {
        is_next_out = match current_pos.1 {
            Direction::Up => current_pos.0 .0 == 0,
            Direction::Down => current_pos.0 .0 == map.len() - 1,
            Direction::Left => current_pos.0 .1 == 0,
            Direction::Right => current_pos.0 .1 == map[0].len() - 1,
        };

        if is_next_out {
            break;
        }

        let next_pos = match current_pos.1 {
            Direction::Up => (current_pos.0 .0 - 1, current_pos.0 .1),
            Direction::Down => (current_pos.0 .0 + 1, current_pos.0 .1),
            Direction::Left => (current_pos.0 .0, current_pos.0 .1 - 1),
            Direction::Right => (current_pos.0 .0, current_pos.0 .1 + 1),
        };

        if map[next_pos.0][next_pos.1] {
            current_pos = match current_pos.1 {
                Direction::Up => (current_pos.0, Direction::Right),
                Direction::Down => (current_pos.0, Direction::Left),
                Direction::Left => (current_pos.0, Direction::Up),
                Direction::Right => (current_pos.0, Direction::Down),
            };
        } else {
            current_pos = (next_pos, current_pos.1);
            break;
        }
    }

    (is_next_out, current_pos)
}

fn part_1(input: &str) {
    let (map, start) = parse_input(input);
    let mut visited_points: HashSet<Coord> = HashSet::new();

    let mut current_pos = start;

    loop {
        visited_points.insert(current_pos.0);

        let (is_next_out, next_pos) = next_pos(&map, current_pos);
        current_pos = next_pos;

        if is_next_out {
            break;
        }
    }

    println!("Part 1: {}", visited_points.len());
}

fn part_2(input: &str) {
    let (mut map, start) = parse_input(input);
    let mut variants = 0;

    for r in 0..map.len() {
        for c in 0..map[0].len() {
            if map[r][c] {
                continue;
            }

            map[r][c] = true;

            let mut visited_positions: HashSet<Position> = HashSet::new();
            let mut current_pos = start;

            loop {
                if visited_positions.contains(&current_pos) {
                    variants += 1;
                    break;
                }

                visited_positions.insert(current_pos);

                let (is_next_out, next_pos) = next_pos(&map, current_pos);
                current_pos = next_pos;

                if is_next_out {
                    break;
                }
            }

            map[r][c] = false;
        }
    }

    println!("Part 2: {variants}");
}

pub fn run(input: &str) {
    part_1(input);
    part_2(input);
}
