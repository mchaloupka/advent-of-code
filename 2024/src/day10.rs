use std::collections::{HashSet, LinkedList};

type Map = Vec<Vec<i32>>;
type Coord = (usize, usize);

fn parse_input(input: &str) -> Map {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|x| x.to_string().parse().expect("Has to be number"))
                .collect()
        })
        .collect()
}

fn get_elevation(map: &Map, coord: Coord) -> i32 {
    map[coord.0][coord.1]
}

fn get_neighbohours(map: &Map, coord: Coord) -> Vec<Coord> {
    let mut neighbohours = Vec::new();

    if coord.0 > 0 {
        neighbohours.push((coord.0 - 1, coord.1));
    }

    if coord.0 < map.len() - 1 {
        neighbohours.push((coord.0 + 1, coord.1));
    }

    if coord.1 > 0 {
        neighbohours.push((coord.0, coord.1 - 1));
    }

    if coord.1 < map[0].len() - 1 {
        neighbohours.push((coord.0, coord.1 + 1));
    }

    neighbohours
}

fn part_1(input: &str) {
    let map = parse_input(input);

    let mut total_score = 0;
    let mut trailheads: Vec<Coord> = Vec::new();

    for (r, row) in map.iter().enumerate() {
        for (c, cell) in row.iter().enumerate() {
            if *cell == 0 {
                trailheads.push((r, c));
            }
        }
    }

    for trailhead in trailheads {
        let mut queue: LinkedList<Coord> = LinkedList::new();
        let mut found_nines: HashSet<Coord> = HashSet::new();

        queue.push_back(trailhead);

        while let Some(current) = queue.pop_front() {
            let elevation = get_elevation(&map, current);

            if elevation == 9 {
                found_nines.insert(current);
            }

            let neighbohours = get_neighbohours(&map, current);

            for next_step in neighbohours {
                let next_elevation = get_elevation(&map, next_step);

                if next_elevation == elevation + 1 {
                    queue.push_back(next_step);
                }
            }
        }

        total_score += found_nines.len();
    }

    println!("Part 1: {total_score}");
}

fn part_2(input: &str) {
    let map = parse_input(input);

    let mut total_score = 0;
    let mut queue: LinkedList<Coord> = LinkedList::new();

    for (r, row) in map.iter().enumerate() {
        for (c, cell) in row.iter().enumerate() {
            if *cell == 0 {
                queue.push_back((r, c));
            }
        }
    }

    while let Some(current) = queue.pop_front() {
        let elevation = get_elevation(&map, current);

        if elevation == 9 {
            total_score += 1;
        }

        let neighbohours = get_neighbohours(&map, current);

        for next_step in neighbohours {
            let next_elevation = get_elevation(&map, next_step);

            if next_elevation == elevation + 1 {
                queue.push_back(next_step);
            }
        }
    }

    println!("Part 2: {total_score}");
}

pub fn run(input: &str) {
    part_1(input);
    part_2(input);
}
