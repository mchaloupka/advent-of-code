use std::collections::{HashMap, HashSet};

type Coord = (usize, usize);

type Map = Vec<Vec<bool>>;

fn parse_input(input: &str) -> (Map, Coord, Coord) {
    let mut map: Map = Vec::new();
    let mut start: Option<Coord> = None;
    let mut end: Option<Coord> = None;

    for (x, row) in input.lines().enumerate() {
        let mut cur_row = Vec::new();

        for (y, c) in row.chars().enumerate() {
            match c {
                '#' => {
                    cur_row.push(true);
                }
                '.' => {
                    cur_row.push(false);
                }
                'S' => {
                    cur_row.push(false);
                    start = Some((x, y));
                }
                'E' => {
                    cur_row.push(false);
                    end = Some((x, y));
                }
                _ => panic!("Unexpected char"),
            }
        }

        map.push(cur_row);
    }

    (map, start.unwrap(), end.unwrap())
}

fn get_next_steps((x, y): Coord, map: &Map) -> Vec<Coord> {
    let mut next_coords = Vec::new();
    if x > 0 {
        next_coords.push((x - 1, y));
    }
    if x < map.len() - 1 {
        next_coords.push((x + 1, y));
    }
    if y > 0 {
        next_coords.push((x, y - 1));
    }
    if y < map[0].len() - 1 {
        next_coords.push((x, y + 1));
    }
    next_coords
}

fn get_next_steps_with_cheats((x, y): Coord, max_cheats: usize, map: &Map) -> HashSet<Coord> {
    let mut output = HashSet::new();

    for cheat_length in 1..max_cheats + 1 {
        for x_diff in 0..cheat_length + 1 {
            let y_diff = cheat_length - x_diff;

            if x >= x_diff && y >= y_diff {
                output.insert((x - x_diff, y - y_diff));
            }
            if x >= x_diff && y + y_diff < map[0].len() {
                output.insert((x - x_diff, y + y_diff));
            }
            if x + x_diff < map.len() && y >= y_diff {
                output.insert((x + x_diff, y - y_diff));
            }
            if x + x_diff < map.len() && y + y_diff < map[0].len() {
                output.insert((x + x_diff, y + y_diff));
            }
        }
    }

    output
}

fn create_shortest_no_cheating_paths(start: Coord, map: &Map) -> HashMap<Coord, usize> {
    let mut distances = HashMap::new();
    distances.insert(start, 0);

    let mut m_prev_pos: Option<Coord> = None;
    let mut cur_pos: Coord = start;
    let mut path_length: usize = 0;

    loop {
        let mut m_next_pos: Option<Coord> = None;

        for next_option in get_next_steps(cur_pos, map) {
            // Skip if it is wall
            if map[next_option.0][next_option.1] {
                continue;
            }

            // Skip if it is the previous pos to not go back
            if let Some(prev_pos) = m_prev_pos {
                if prev_pos == next_option {
                    continue;
                }
            }

            if m_next_pos.is_some() {
                panic!("Found multiple options where to go");
            }

            m_next_pos = Some(next_option);
        }

        match m_next_pos {
            None => {
                break;
            }
            Some(next_pos) => {
                path_length += 1;
                distances.insert(next_pos, path_length);
                m_prev_pos = Some(cur_pos);
                cur_pos = next_pos;
            }
        }
    }

    distances
}

fn calculate_part(input: &str, cheating_length: usize) -> i32 {
    let (map, start, end) = parse_input(input);

    let no_cheat_map = create_shortest_no_cheating_paths(start, &map);
    let &no_cheat_distance = no_cheat_map.get(&end).unwrap();

    let mut total_count = 0;

    for cheat_start in no_cheat_map.keys().cloned() {
        let &distance_to_cheat_start = no_cheat_map.get(&cheat_start).unwrap();

        for cheat_end in get_next_steps_with_cheats(cheat_start, cheating_length, &map) {
            if map[cheat_end.0][cheat_end.1] {
                continue;
            }

            let distance_from_cheat_end = no_cheat_distance - no_cheat_map.get(&cheat_end).unwrap();

            let cheat_length =
                cheat_end.0.abs_diff(cheat_start.0) + cheat_end.1.abs_diff(cheat_start.1);

            let total_distance = distance_to_cheat_start + distance_from_cheat_end + cheat_length;

            if total_distance <= no_cheat_distance - 100 {
                // println!("{:?}-{:?} ({}): {}", cheat_start, cheat_end, cheat_length, no_cheat_distance - total_distance);
                total_count += 1;
            }
        }
    }

    total_count
}

fn part_1(input: &str) {
    println!("Part 1: {}", calculate_part(input, 2));
}

fn part_2(input: &str) {
    println!("Part 2: {}", calculate_part(input, 20));
}

pub fn run(input: &str) {
    part_1(input);
    part_2(input);
}
