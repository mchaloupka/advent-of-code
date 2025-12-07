use std::collections::{HashMap, HashSet};

type Location = (usize, usize);

enum Place {
    Empty,
    Splitter,
}

fn parse_input(input: &str) -> (Location, Vec<Vec<Place>>) {
    let mut starting_point = None;

    let parsed_points: Vec<Vec<Place>> = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| match c {
                    '.' => Place::Empty,
                    '^' => Place::Splitter,
                    'S' => {
                        starting_point = Some((x, y));
                        Place::Empty
                    }
                    _ => panic!("Invalid character: {}", c),
                })
                .collect::<Vec<Place>>()
        })
        .collect();

    (starting_point.unwrap(), parsed_points)
}

pub fn run(input: &str) {
    let (starting_point, matrix) = parse_input(input);

    let mut part_1 = 0;

    let mut rays: HashSet<usize> = HashSet::new();
    rays.insert(starting_point.0);

    for row in matrix.iter().skip(starting_point.1 + 1) {
        let mut new_rays: HashSet<usize> = HashSet::new();

        for r in rays.iter() {
            match row[*r] {
                Place::Empty => {
                    new_rays.insert(*r);
                }
                Place::Splitter => {
                    part_1 += 1;
                    new_rays.insert(r - 1);
                    new_rays.insert(r + 1);
                }
            }
        }

        rays = new_rays;
    }

    println!("Part 1: {}", part_1);

    let mut triggered_worlds: HashMap<Location, usize> = HashMap::new();

    for y in (0..matrix.len()).rev() {
        let row = &matrix[y];

        for (x, p) in row.iter().enumerate() {
            if y == matrix.len() - 1 {
                triggered_worlds.insert((x, y), 1);
            } else {
                match p {
                    Place::Empty => {
                        triggered_worlds
                            .insert((x, y), *triggered_worlds.get(&(x, y + 1)).unwrap());
                    }
                    Place::Splitter => {
                        triggered_worlds.insert(
                            (x, y),
                            *triggered_worlds.get(&(x - 1, y + 1)).unwrap()
                                + *triggered_worlds.get(&(x + 1, y + 1)).unwrap(),
                        );
                    }
                }
            }
        }
    }

    let part_2 = triggered_worlds.get(&starting_point).unwrap();

    println!("Part 2: {}", part_2);
}
