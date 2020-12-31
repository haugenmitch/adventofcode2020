use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};

fn main() -> Result<(), Error> {
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

    Ok(())
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
