use std::collections::{HashMap, HashSet};

type Matrix = Vec<Vec<char>>;
type Coord = (i64, i64);

fn parse_input(input: &str) -> Matrix {
    input.lines().map(|row| row.chars().collect()).collect()
}

fn get_antena_groups(matrix: &[Vec<char>]) -> HashMap<char, Vec<Coord>> {
    let mut output: HashMap<char, Vec<Coord>> = HashMap::new();

    for (r, row) in matrix.iter().enumerate() {
        for (c, x) in row.iter().enumerate() {
            if *x != '.' {
                output.entry(*x).or_default().push((r as i64, c as i64));
            }
        }
    }

    output
}

fn part_1(input: &str) {
    let matrix = parse_input(input);

    let antena_groups = get_antena_groups(&matrix);

    let mut unique_antinodes: HashSet<Coord> = HashSet::new();

    for (_, antenas_coord) in antena_groups.into_iter() {
        for x in &antenas_coord {
            for y in &antenas_coord {
                if x != y {
                    let dr = y.0 - x.0;
                    let dc = y.1 - x.1;

                    let target_point = (y.0 + dr, y.1 + dc);
                    if target_point.0 < 0
                        || target_point.1 < 0
                        || target_point.0 >= (matrix.len() as i64)
                        || target_point.1 >= (matrix[1].len() as i64)
                    {
                        continue;
                    } else {
                        unique_antinodes.insert(target_point);
                    }
                }
            }
        }
    }

    println!("Part 1: {}", unique_antinodes.len());
}

fn part_2(input: &str) {
    let matrix = parse_input(input);

    let antena_groups = get_antena_groups(&matrix);

    let mut unique_antinodes: HashSet<Coord> = HashSet::new();

    for (_, antenas_coord) in antena_groups.into_iter() {
        for x in &antenas_coord {
            for y in &antenas_coord {
                if x != y {
                    unique_antinodes.insert(*x);
                    unique_antinodes.insert(*y);

                    let dr = y.0 - x.0;
                    let dc = y.1 - x.1;

                    let mut target_point = (y.0 + dr, y.1 + dc);

                    loop {
                        if target_point.0 < 0
                            || target_point.1 < 0
                            || target_point.0 >= (matrix.len() as i64)
                            || target_point.1 >= (matrix[1].len() as i64)
                        {
                            break;
                        } else {
                            unique_antinodes.insert(target_point);
                            target_point = (target_point.0 + dr, target_point.1 + dc);
                        }
                    }
                }
            }
        }
    }

    println!("Part 2: {}", unique_antinodes.len());
}

pub fn run(input: &str) {
    part_1(input);
    part_2(input);
}
