use std::collections::LinkedList;

use itertools::Itertools;

#[derive(Debug, Clone, Copy)]
enum Field {
    Robot,
    Nothing,
    Box,
    Wall,
}

enum Field2 {
    Robot,
    Nothing,
    BoxR,
    BoxL,
    Wall,
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

type Coord = (usize, usize);

struct State {
    fields: Vec<Vec<Field>>,
    robot: Coord,
    instructions: LinkedList<Direction>,
}

struct State2 {
    fields: Vec<Vec<Field2>>,
    robot: Coord,
    instructions: LinkedList<Direction>,
}

fn parse_input(input: &str) -> State {
    let map_line_count = input.lines().find_position(|x| x.is_empty()).unwrap().0;

    let fields: Vec<Vec<Field>> = input
        .lines()
        .take(map_line_count)
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '#' => Field::Wall,
                    'O' => Field::Box,
                    '@' => Field::Robot,
                    '.' => Field::Nothing,
                    _ => panic!("Unknown char {c}"),
                })
                .collect()
        })
        .collect();

    let mut robot: Option<Coord> = None;

    for (x, row) in fields.iter().enumerate() {
        for (y, cell) in row.iter().enumerate() {
            if matches!(cell, Field::Robot) {
                robot = Some((x, y));
            }
        }
    }

    let instruction_lines = input.lines().skip(map_line_count + 1);

    let instructions: LinkedList<_> = instruction_lines
        .flat_map(|line| {
            line.chars().map(|c| match c {
                '>' => Direction::Right,
                'v' => Direction::Down,
                '<' => Direction::Left,
                '^' => Direction::Up,
                _ => panic!("Unexpected character in instructions"),
            })
        })
        .collect();

    State {
        fields,
        robot: robot.unwrap(),
        instructions,
    }
}

fn try_move_field(state: &mut State, (x, y): Coord, direction: Direction) -> Option<Coord> {
    match state.fields[x][y] {
        Field::Robot | Field::Box => {
            let next_position = match direction {
                Direction::Up => (x - 1, y),
                Direction::Down => (x + 1, y),
                Direction::Left => (x, y - 1),
                Direction::Right => (x, y + 1),
            };

            match try_move_field(state, next_position, direction) {
                Some(_) => {
                    state.fields[next_position.0][next_position.1] = state.fields[x][y];
                    state.fields[x][y] = Field::Nothing;
                    Some(next_position)
                }
                None => None,
            }
        }
        Field::Nothing => Some((x, y)),
        Field::Wall => None,
    }
}

fn perform_move(state: &mut State) {
    let instruction = state.instructions.pop_front().unwrap();
    let robot_move = try_move_field(state, state.robot, instruction);
    if let Some(new_coord) = robot_move {
        state.robot = new_coord;
    }
}

fn calculate_coords<T, X>(fields: &[Vec<T>], matcher: X) -> usize
where
    X: Fn(&T) -> bool,
{
    let mut total: usize = 0;

    for (x, row) in fields.iter().enumerate() {
        for (y, cell) in row.iter().enumerate() {
            if matcher(cell) {
                total += 100 * x + y;
            }
        }
    }

    total
}

fn calculate_coords_1(state: &State) -> usize {
    calculate_coords(&state.fields, |x| matches!(x, Field::Box))
}

fn part_1(input: &str) {
    let mut state = parse_input(input);

    while !state.instructions.is_empty() {
        perform_move(&mut state);
    }

    let outcome = calculate_coords_1(&state);

    println!("Part 1: {outcome}");
}

fn expand_state(input: &State) -> State2 {
    let fields = input
        .fields
        .iter()
        .map(|row| {
            row.iter()
                .flat_map(|field| match field {
                    Field::Robot => vec![Field2::Robot, Field2::Nothing],
                    Field::Nothing => vec![Field2::Nothing, Field2::Nothing],
                    Field::Box => vec![Field2::BoxL, Field2::BoxR],
                    Field::Wall => vec![Field2::Wall, Field2::Wall],
                })
                .collect()
        })
        .collect();

    State2 {
        fields,
        instructions: input.instructions.clone(),
        robot: (input.robot.0, input.robot.1 * 2),
    }
}

fn move_field_2(state: &mut State2, (x, y): Coord, direction: Direction) -> Coord {
    let (nx, ny) = match direction {
        Direction::Up => (x - 1, y),
        Direction::Down => (x + 1, y),
        Direction::Left => (x, y - 1),
        Direction::Right => (x, y + 1),
    };

    match state.fields[x][y] {
        Field2::Robot => {
            move_field_2(state, (nx, ny), direction);
            state.fields[nx][ny] = Field2::Robot;
            state.fields[x][y] = Field2::Nothing;
            (nx, ny)
        }
        Field2::Nothing => (nx, ny),
        Field2::BoxR => match direction {
            Direction::Left | Direction::Right => {
                move_field_2(state, (nx, ny), direction);
                state.fields[nx][ny] = Field2::BoxR;
                state.fields[x][y] = Field2::Nothing;
                (nx, ny)
            }
            Direction::Up | Direction::Down => {
                move_field_2(state, (nx, ny), direction);
                state.fields[nx][ny] = Field2::BoxR;
                state.fields[x][y] = Field2::Nothing;
                move_field_2(state, (x, y - 1), direction);
                (nx, ny)
            }
        },
        Field2::BoxL => match direction {
            Direction::Left | Direction::Right => {
                move_field_2(state, (nx, ny), direction);
                state.fields[nx][ny] = Field2::BoxL;
                state.fields[x][y] = Field2::Nothing;
                (nx, ny)
            }
            Direction::Up | Direction::Down => {
                move_field_2(state, (nx, ny), direction);
                state.fields[nx][ny] = Field2::BoxL;
                state.fields[x][y] = Field2::Nothing;
                move_field_2(state, (x, y + 1), direction);
                (nx, ny)
            }
        },
        Field2::Wall => panic!("Should not be called"),
    }
}

fn can_move_field_2(state: &State2, (x, y): Coord, direction: Direction) -> bool {
    match state.fields[x][y] {
        Field2::Nothing => true,
        Field2::Wall => false,
        Field2::Robot => {
            let next_position = match direction {
                Direction::Up => (x - 1, y),
                Direction::Down => (x + 1, y),
                Direction::Left => (x, y - 1),
                Direction::Right => (x, y + 1),
            };

            can_move_field_2(state, next_position, direction)
        }
        Field2::BoxR => {
            let next_positions = match direction {
                Direction::Up => vec![(x - 1, y), (x - 1, y - 1)],
                Direction::Down => vec![(x + 1, y), (x + 1, y - 1)],
                Direction::Left => vec![(x, y - 1)],
                Direction::Right => vec![(x, y + 1)],
            };

            next_positions
                .iter()
                .all(|pos| can_move_field_2(state, *pos, direction))
        }
        Field2::BoxL => {
            let next_positions = match direction {
                Direction::Up => vec![(x - 1, y), (x - 1, y + 1)],
                Direction::Down => vec![(x + 1, y), (x + 1, y + 1)],
                Direction::Left => vec![(x, y - 1)],
                Direction::Right => vec![(x, y + 1)],
            };

            next_positions
                .iter()
                .all(|pos| can_move_field_2(state, *pos, direction))
        }
    }
}

fn perform_move_2(state: &mut State2) {
    let instruction = state.instructions.pop_front().unwrap();

    if can_move_field_2(state, state.robot, instruction) {
        state.robot = move_field_2(state, state.robot, instruction);
    }
}

fn calculate_coords_2(state: &State2) -> usize {
    calculate_coords(&state.fields, |x| matches!(x, Field2::BoxL))
}

fn part_2(input: &str) {
    let mut state = expand_state(&parse_input(input));

    while !state.instructions.is_empty() {
        perform_move_2(&mut state);
    }

    let outcome = calculate_coords_2(&state);

    println!("Part 2: {outcome}");
}

pub fn run(input: &str) {
    part_1(input);
    part_2(input);
}
