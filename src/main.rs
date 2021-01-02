extern crate clap;
use clap::{App, Arg};
use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};

fn main() -> Result<(), Error> {
    let _d = parse_args();

    let mut vec = read(File::open("./inputs/01.txt")?)?;
    vec.sort();

    let mut n1: i64 = 0;
    let mut n2: i64 = 0;

    for i in &vec {
        if vec.contains(&(2020 - i)) {
            n1 = *i;
            n2 = 2020 - i;
            break;
        }
    }

    println!("{} x {} = {}", n1, n2, n1 * n2);

    let mut n3: i64 = 0;
    let mut n4: i64 = 0;
    let mut n5: i64 = 0;

    for j in &vec {
        for k in &vec {
            if vec.contains(&(2020 - j - k)) {
                n3 = *j;
                n4 = *k;
                n5 = 2020 - j - k;
                break;
            }
        }
    }

    println!("{} x {} x {} = {}", n3, n4, n5, n3 * n4 * n5);

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

fn read<R: Read>(io: R) -> Result<Vec<i64>, Error> {
    let br = BufReader::new(io);
    let mut v = vec![];
    for line in br.lines() {
        v.push(
            line?
                .trim()
                .parse()
                .map_err(|e| Error::new(ErrorKind::InvalidData, e))?,
        );
    }
    Ok(v)
}
