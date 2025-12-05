use std::fs::File;
use std::io::{self, BufRead};

fn main() -> std::io::Result<()> {
    // println!("Hello, world!");
    let file = File::open("../input.txt")?;
    let mut buf = io::BufReader::new(file);

    let mut line = String::new();

    let mut intervals = Vec::new();

    while buf.read_line(&mut line)? > 2 {
        let read_line = std::mem::replace(&mut line, String::new());
        let trimed = read_line.trim();
        //println!("Read line (len = {}): `{}`", read_line.len(), trimed);
        let sep_idx = trimed.find('-').unwrap_or(trimed.len());
        intervals.push((
            trimed[..sep_idx].parse::<u64>().unwrap(),
            trimed[sep_idx + 1..].parse::<u64>().unwrap(),
        ));
    }

    let result = solve(intervals);

    println!("Result: {}", result);

    Ok(())
}

fn solve(mut intervals: Vec<(u64, u64)>) -> u128 {
    if intervals.is_empty() {
        return 0;
    }

    intervals.sort_unstable();
    let mut cur = intervals[0];
    let mut count = 0;

    for i in 1..intervals.len() {
        if intervals[i].0 <= cur.1 {
            cur.1 = cur.1.max(intervals[i].1);
        } else {
            count += (cur.1 - cur.0 + 1) as u128;
            cur = intervals[i];
        }
    }

    count + (cur.1 - cur.0 + 1) as u128
}
