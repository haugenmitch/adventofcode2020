pub fn run(lines: Vec<String>) {
    let mut vec: Vec<i64> = vec![];
    for line in lines {
        match line.trim().parse::<i64>() {
            Ok(n) => vec.push(n),
            Err(_) => println!("\"{}\" could not be parsed as an integer", line),
        }
    }
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
}
