use std::collections::{HashSet, LinkedList};

type Map = Vec<Vec<bool>>;

type Coord = (usize, usize);

fn parse_input(input: &str) -> Vec<Coord> {
    input
        .lines()
        .map(|line| {
            let (xs, ys) = line.split_once(",").unwrap();
            (xs.parse().unwrap(), ys.parse().unwrap())
        })
        .collect()
}

fn create_empty_map() -> Map {
    (0..71).map(|_| (0..71).map(|_| false).collect()).collect()
}

fn get_path_length(falling_bytes: &[Coord], bytes_to_fall: usize) -> Option<usize> {
    let mut map = create_empty_map();

    for fallen_byte in falling_bytes.iter().take(bytes_to_fall) {
        map[fallen_byte.1][fallen_byte.0] = true;
    }

    let mut queue: LinkedList<(Coord, usize)> = LinkedList::new();
    let mut visited: HashSet<Coord> = HashSet::new();
    let mut shortest_path = None;

    queue.push_back(((0, 0), 0));

    while let Some((coord, path_length)) = queue.pop_front() {
        if coord == (map.len() - 1, map[0].len() - 1) {
            shortest_path = Some(path_length);
            break;
        }

        if visited.contains(&coord) {
            continue;
        } else {
            visited.insert(coord);
        }

        if map[coord.1][coord.0] {
            continue;
        }

        let mut next_coords = Vec::new();

        if coord.0 > 0 {
            next_coords.push((coord.0 - 1, coord.1));
        }
        if coord.0 < map.len() - 1 {
            next_coords.push((coord.0 + 1, coord.1));
        }
        if coord.1 > 0 {
            next_coords.push((coord.0, coord.1 - 1));
        }
        if coord.1 < map[0].len() - 1 {
            next_coords.push((coord.0, coord.1 + 1));
        }

        for next_coord in next_coords {
            queue.push_back((next_coord, path_length + 1));
        }
    }

    shortest_path
}

fn part_1(falling_bytes: &[Coord]) {
    let shortest_path = get_path_length(falling_bytes, 1024).unwrap();

    println!("Part 1: {}", shortest_path);
}

fn find_failing_index(falling_bytes: &[Coord], succeeding: usize, failing: usize) -> usize {
    if succeeding + 1 == failing {
        failing - 1 // We convert it from how many bytes will cause failure into the index of problematic byte
    } else {
        let mid_point = (succeeding + failing) / 2;

        match get_path_length(falling_bytes, mid_point) {
            Some(_) => find_failing_index(falling_bytes, mid_point, failing),
            None => find_failing_index(falling_bytes, succeeding, mid_point),
        }
    }
}

fn part_2(falling_bytes: &[Coord]) {
    let failing_index = find_failing_index(falling_bytes, 0, falling_bytes.len());
    let failing_byte = falling_bytes[failing_index];
    println!("Part 2: {},{}", failing_byte.0, failing_byte.1);
}

pub fn run(input: &str) {
    let falling_bytes = parse_input(input);

    part_1(&falling_bytes);
    part_2(&falling_bytes);
}
