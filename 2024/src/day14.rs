use regex::Regex;
use std::{thread, time};

#[derive(Debug, Clone, Copy)]
struct Coord {
    x: i128,
    y: i128,
}

#[derive(Debug, Clone, Copy)]
struct Robot {
    p: Coord,
    v: Coord,
}

fn parse_input(input: &str) -> Vec<Robot> {
    let re = Regex::new(r"p=(?P<px>-?\d+),(?P<py>-?\d+) v=(?P<vx>-?\d+),(?P<vy>-?\d+)").unwrap();

    input
        .lines()
        .map(|line| {
            let capture = re.captures(line).unwrap();

            Robot {
                p: Coord {
                    x: capture["px"].parse().unwrap(),
                    y: capture["py"].parse().unwrap(),
                },
                v: Coord {
                    x: capture["vx"].parse().unwrap(),
                    y: capture["vy"].parse().unwrap(),
                }
            }
        })
        .collect()
}

fn simulate_robot(robot: &Robot, steps: i128, width: i128, height: i128) -> Robot {
    Robot { 
        p: Coord {
            x: (robot.p.x + robot.v.x * steps).rem_euclid(width),
            y: (robot.p.y + robot.v.y * steps).rem_euclid(height),
        },
        ..*robot
    }
}

fn part_1(input: &str) {
    let robots = parse_input(input);
    let width = 101;
    let height = 103;

    let after_simulation: Vec<_> = robots
        .into_iter()
        .map(|r| { simulate_robot(&r, 100, width, height) })
        .collect();

    let mut quadrant_counts = vec![0, 0, 0, 0];

    for r in after_simulation {
        if r.p.x < width / 2 && r.p.y < height / 2 {
            quadrant_counts[0] += 1;
        } else if r.p.x >= width / 2 + 1 && r.p.y < height / 2 {
            quadrant_counts[1] += 1;
        } else if r.p.x < width / 2 && r.p.y >= height / 2 + 1 {
            quadrant_counts[2] += 1;
        } else if r.p.x >= width / 2 + 1 && r.p.y >= height / 2 + 1 {
            quadrant_counts[3] += 1;
        }
    }

    let mut safety_factor = 1;

    for quadrant in quadrant_counts {
        safety_factor *= quadrant;
    }

    println!("Part 1: {safety_factor}");
}

fn print_robots(robots: &Vec<Robot>, steps: i128, width: i128, height: i128) -> bool {
    let mut map: Vec<Vec<bool>> = 
        (0..height).map(|_| { (0..width).map(|_| { false }).collect() }).collect();

    for robot in robots {
        map[robot.p.y as usize][robot.p.x as usize] = true;
    }

    let mut can_be_tree = false;

    for y in 0..map.len() - 5 {
        for x in 0..map[0].len() - 5 {
            let mut found_block = true;

            for dy in 0..5 {
                for dx in 0..5 {
                    if !map[y + dy][x + dx] { found_block = false; break; }
                }

                if !found_block { break; }
            } 

            if found_block { can_be_tree = true; break; } 
        }

        if can_be_tree { break; }
    }

    if can_be_tree {
        for row in map {
            for c in row {
                if c {    
                    print!("X");
                } else {
                    print!(".");
                }
            }
    
            println!("");
        }

        println!("");
        println!("{steps}");
    }

    can_be_tree
}

fn part_2(input: &str) {
    let robots = parse_input(input);
    let width = 101;
    let height = 103;
    let mut steps = 0;

    loop {
        let after_simulation: Vec<_> = robots
            .iter()
            .map(|r| { simulate_robot(&r, steps, width, height) })
            .collect();

        let can_be_tree = print_robots(&after_simulation, steps, width, height);

        if can_be_tree {
            break;
        }
        
        steps += 1;
    }
}

pub fn run(input: &str) {
    part_1(input);
    part_2(input);
}