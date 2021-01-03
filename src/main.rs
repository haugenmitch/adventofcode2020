extern crate clap;
use clap::{App, Arg};
use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::path::Path;

mod days;

fn main() -> Result<(), Error> {
    let matches = parse_args();

    let day_string = matches.value_of("day");
    let day;
    match day_string {
        None => {
            println!("A day must be provided.");
            return Ok(());
        }
        Some(s) => match s.parse::<i32>() {
            Ok(d) => day = d,
            Err(_) => {
                println!("\"{}\" could not be parsed as a number.", s);
                return Ok(());
            }
        },
    }

    let part;
    match matches.value_of("part") {
        None => part = 0,
        Some(s) => match s.parse::<i32>() {
            Ok(p) => part = p,
            Err(_) => {
                println!("Part must be given as an integer.");
                return Ok(());
            }
        },
    }
    println!("{}", part);

    let filepath = format!("./inputs/{:0>2}.txt", day);
    let lines = read_lines(filepath);

    match day {
        1 => days::day01::run(lines),
        2..=25 => println!("Not implemented yet"),
        _ => println!("You must enter a -day value from 1 to 25."),
    }

    Ok(())
}

fn parse_args() -> clap::ArgMatches<'static> {
    let matches = App::new("Advent of Code 2020")
        .version("0.1.1")
        .author("Mitchell Haugen <haugenmitch@gmail.com>")
        .about("Solutions to the Advent of Code 2020 coding challenge in Rust")
        .arg(
            Arg::with_name("day")
                .short("d")
                .long("day")
                .takes_value(true)
                .help("Day of the challenge, a value from 1-25"),
        )
        .arg(
            Arg::with_name("part")
                .short("p")
                .long("part")
                .takes_value(true)
                .help("Part of the daily challenge, either 1 or 2"),
        )
        .get_matches();

    return matches;
}

// TODO Add error checking
fn read_lines(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}
