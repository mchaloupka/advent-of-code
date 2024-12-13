use std::collections::{HashMap, HashSet};

type Map = Vec<Vec<char>>;

#[derive(Debug, Clone, Copy)]
struct Group {
    perimeter: usize,
    area: usize,
}

fn parse_input(input: &str) -> Map {
    input
        .lines()
        .map(|line| line.trim().chars().collect())
        .collect()
}

pub fn run(input: &str) {
    let map = parse_input(input);
    let mut group_assignment: HashMap<(usize, usize), usize> = HashMap::new();
    let mut group_locations: HashMap<usize, HashSet<(usize, usize)>> = HashMap::new();
    let mut group_values: HashMap<usize, Group> = HashMap::new();
    let mut cur_id = 0;

    for x in 0..map.len() {
        for y in 0..map.len() {
            let c = map[x][y];

            let mut top = None;
            let mut left = None;

            if x > 0 && map[x - 1][y] == c {
                top = group_assignment.get(&(x - 1, y)).copied();
            }

            if y > 0 && map[x][y - 1] == c {
                left = group_assignment.get(&(x, y - 1)).copied();
            }

            match (left, top) {
                (None, None) => {
                    group_assignment.insert((x, y), cur_id);
                    group_locations.insert(cur_id, HashSet::from([(x, y)]));
                    group_values.insert(
                        cur_id,
                        Group {
                            area: 1,
                            perimeter: 4,
                        },
                    );
                    cur_id += 1;
                }
                (None, Some(group_id)) | (Some(group_id), None) => {
                    group_assignment.insert((x, y), group_id);
                    group_locations.get_mut(&group_id).unwrap().insert((x, y));
                    let group = group_values.get_mut(&group_id).unwrap();
                    group.area += 1;
                    group.perimeter += 2;
                }
                (Some(group_id), Some(other_group_id)) if group_id == other_group_id => {
                    group_assignment.insert((x, y), group_id);
                    group_locations.get_mut(&group_id).unwrap().insert((x, y));
                    let group = group_values.get_mut(&group_id).unwrap();
                    group.area += 1;
                }
                (Some(group_id), Some(other_group_id)) => {
                    let other_group = group_values.get(&other_group_id).unwrap();
                    let other_area = other_group.area;
                    let other_perimeter = other_group.perimeter;

                    let group = group_values.get_mut(&group_id).unwrap();

                    group.area += other_area + 1;
                    group.perimeter += other_perimeter;

                    group_values.remove(&other_group_id);

                    let to_remap = group_locations.remove(&other_group_id).unwrap();

                    for remap in to_remap {
                        group_locations.get_mut(&group_id).unwrap().insert(remap);
                        group_assignment.insert(remap, group_id);
                    }

                    group_locations.get_mut(&group_id).unwrap().insert((x, y));
                    group_assignment.insert((x, y), group_id);
                }
            }
        }
    }

    let mut total_count_1 = 0;

    for group in group_values.values() {
        total_count_1 += group.area * group.perimeter;
    }

    println!("Part 1: {total_count_1}");

    let mut total_count_2 = 0;

    for (group_id, group) in group_values {
        let locations = group_locations.get(&group_id).unwrap();
        let mut corners = 0;

        for (x, y) in locations {
            let mut groups_of_four = Vec::new();

            // Doing groups of 4 locations that contain the point
            let x_coord = *x as i32;
            let y_coord = *y as i32;

            // In the group it is ordered as [opposite, adjacent, adjacent, point]
            groups_of_four.push(vec![
                (x_coord - 1, y_coord - 1),
                (x_coord - 1, y_coord),
                (x_coord, y_coord - 1),
                (x_coord, y_coord),
            ]);
            groups_of_four.push(vec![
                (x_coord - 1, y_coord + 1),
                (x_coord - 1, y_coord),
                (x_coord, y_coord + 1),
                (x_coord, y_coord),
            ]);
            groups_of_four.push(vec![
                (x_coord + 1, y_coord - 1),
                (x_coord + 1, y_coord),
                (x_coord, y_coord - 1),
                (x_coord, y_coord),
            ]);
            groups_of_four.push(vec![
                (x_coord + 1, y_coord + 1),
                (x_coord + 1, y_coord),
                (x_coord, y_coord + 1),
                (x_coord, y_coord),
            ]);

            for group_of_four in groups_of_four {
                let in_group: HashSet<_> = group_of_four
                    .iter()
                    .filter(|(x_c, y_c)| {
                        *x_c >= 0
                            && *y_c >= 0
                            && (*x_c as usize) < map.len()
                            && (*y_c as usize) < map[0].len()
                            && locations.contains(&(*x_c as usize, *y_c as usize))
                    })
                    .collect();

                // Outer corner is when the adjacents are not in the group
                if !in_group.contains(&group_of_four[1]) && !in_group.contains(&group_of_four[2]) {
                    corners += 1;
                }

                // Inner corner is when the opposite is not in the group but adjacents are
                if !in_group.contains(&group_of_four[0])
                    && in_group.contains(&group_of_four[1])
                    && in_group.contains(&group_of_four[2])
                {
                    corners += 1;
                }
            }
        }

        total_count_2 += corners * group.area;
    }

    println!("Part 2: {total_count_2}");
}
