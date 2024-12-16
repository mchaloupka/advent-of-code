use std::collections::{BinaryHeap, HashMap, HashSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

type Coord = (usize, usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Field {
    Wall,
    Nothing,
}

struct Map {
    fields: Vec<Vec<Field>>,
    start: Coord,
    end: Coord,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct PositionScore {
    coord: Coord,
    direction: Direction,
    score: i64,
    visited_coords: HashSet<Coord>,
}

impl Ord for PositionScore {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (-self.score).cmp(&-other.score)
    }
}

impl PartialOrd for PositionScore {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_input(input: &str) -> Map {
    let mut fields: Vec<Vec<Field>> = Vec::new();
    let mut start: Option<Coord> = None;
    let mut end: Option<Coord> = None;

    for (x, rline) in input.lines().enumerate() {
        let mut row: Vec<Field> = Vec::new();

        for (y, c) in rline.chars().enumerate() {
            let field = match c {
                '#' => Field::Wall,
                '.' => Field::Nothing,
                'E' => {
                    end = Some((x, y));
                    Field::Nothing
                }
                'S' => {
                    start = Some((x, y));
                    Field::Nothing
                }
                _ => panic!("Unexpected char"),
            };

            row.push(field);
        }

        fields.push(row);
    }

    Map {
        fields,
        start: start.unwrap(),
        end: end.unwrap(),
    }
}

pub fn run(input: &str) {
    let map = parse_input(input);

    let mut movement_heap: BinaryHeap<PositionScore> = BinaryHeap::new();
    movement_heap.push(PositionScore {
        coord: map.start,
        direction: Direction::Right,
        score: 0,
        visited_coords: HashSet::from([map.start]),
    });

    let mut lowest_score_per_pos_score: HashMap<(Coord, Direction), i64> = HashMap::new();

    let mut lowest_score = None;
    let mut on_shortest_path: HashSet<Coord> = HashSet::new();

    while let Some(cur) = movement_heap.pop() {
        if matches!(map.fields[cur.coord.0][cur.coord.1], Field::Wall) {
            continue;
        }

        match lowest_score {
            Some(x) if x < cur.score => {
                break;
            }
            _ => {}
        }

        if cur.coord == map.end {
            lowest_score = Some(cur.score);
            on_shortest_path.extend(cur.visited_coords.iter());
        }

        match lowest_score_per_pos_score.entry((cur.coord, cur.direction)) {
            std::collections::hash_map::Entry::Occupied(x) if *x.get() < cur.score => {
                continue;
            }
            std::collections::hash_map::Entry::Occupied(_) => {}
            std::collections::hash_map::Entry::Vacant(vacant_entry) => {
                vacant_entry.insert(cur.score);
            }
        }

        let next_in_direction = match cur.direction {
            Direction::Left => (cur.coord.0, cur.coord.1 - 1),
            Direction::Right => (cur.coord.0, cur.coord.1 + 1),
            Direction::Up => (cur.coord.0 - 1, cur.coord.1),
            Direction::Down => (cur.coord.0 + 1, cur.coord.1),
        };

        let mut next_set_of_coords = cur.visited_coords.clone();
        next_set_of_coords.insert(next_in_direction);

        movement_heap.push(PositionScore {
            coord: next_in_direction,
            direction: cur.direction,
            score: cur.score + 1,
            visited_coords: next_set_of_coords,
        });

        let next_directions = match cur.direction {
            Direction::Left | Direction::Right => {
                vec![Direction::Up, Direction::Down]
            }
            Direction::Up | Direction::Down => {
                vec![Direction::Left, Direction::Right]
            }
        };

        for next_direction in next_directions {
            movement_heap.push(PositionScore {
                coord: cur.coord,
                direction: next_direction,
                score: cur.score + 1000,
                visited_coords: cur.visited_coords.clone(),
            });
        }
    }

    println!("Part 1: {}", lowest_score.unwrap());
    println!("Part 2: {}", on_shortest_path.len());
}
