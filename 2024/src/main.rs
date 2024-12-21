use git2::Config as gitConfig;
use reqwest::blocking::Client;
use reqwest::header::{COOKIE, USER_AGENT};
use std::env;
use std::fs;
use std::io::Read;
use std::io::{BufReader, Write};
use std::time::Instant;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;

mod util;

fn day_func(day: i32) -> fn(&str) {
    match day {
        1 => day01::run,
        2 => day02::run,
        3 => day03::run,
        4 => day04::run,
        5 => day05::run,
        6 => day06::run,
        7 => day07::run,
        8 => day08::run,
        9 => day09::run,
        10 => day10::run,
        11 => day11::run,
        12 => day12::run,
        13 => day13::run,
        14 => day14::run,
        15 => day15::run,
        16 => day16::run,
        17 => day17::run,
        18 => day18::run,
        19 => day19::run,
        20 => day20::run,
        _ => unreachable!(),
    }
}

fn input_file_path(day: i32) -> String {
    format!("./input/day{:02}.txt", day)
}

fn run_day(day: i32) {
    let path = input_file_path(day);
    let input = fs::read_to_string(&path).unwrap();

    let start_time = Instant::now();
    day_func(day)(&input);
    println!(
        "Time: {:.3}",
        (start_time.elapsed().as_millis() as f64) / 1000.0
    );
}

fn init_day(day: i32) {
    let url = format!("https://adventofcode.com/2024/day/{day}/input");

    let session = fs::read_to_string(".session")
        .expect("Was not able to load the 'session' cookie from .session file. Sign in to AOC pages, get the 'session' cookie and store it in .session file.");

    let name = gitConfig::open_default()
        .expect("Failed to get git config")
        .get_string("user.name")
        .expect("Failed to get 'user.name' from git config");

    let client = Client::new();

    let response = client
        .get(url)
        .header(USER_AGENT, name)
        .header(COOKIE, format!("session={}", session.trim()))
        .send()
        .unwrap();

    if !response.status().is_success() {
        panic!("Failed file download, most likely not authorized.");
    }

    let file_path = input_file_path(day);
    let mut file = fs::File::create(file_path.clone()).expect("Failed to create file for input.");

    let mut reader = BufReader::new(response);
    let mut buffer = Vec::new();
    reader
        .read_to_end(&mut buffer)
        .expect("Failed to read input from web");
    file.write_all(&buffer)
        .expect("Failed to write to output file");

    println!("Succeeded to write to file '{}'", file_path);
}

fn write_usage() {
    println!("Incorrect arguments, either day number to run code, or init followed by the day number to download input and create source file.");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        2 => {
            run_day(args[1].parse().unwrap());
        }
        3 => {
            if args[1] != "init" {
                write_usage();
            } else {
                init_day(args[2].parse().unwrap());
            }
        }
        _ => {
            write_usage();
        }
    }
}
