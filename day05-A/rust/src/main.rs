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

    let ids = buf
        .lines()
        .map(|l| l.unwrap().parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    let result = solve(intervals, ids);

    println!("Result: {}", result);

    Ok(())
}

fn solve(mut intervals: Vec<(u64, u64)>, ids: Vec<u64>) -> usize {
    if intervals.is_empty() {
        return ids.len();
    }

    intervals.sort_unstable();

    let mut txf_intervals = vec![intervals[0].0, intervals[0].1];

    for i in 0..intervals.len() {
        if Some(intervals[i].0) <= txf_intervals.last().copied() {
            *txf_intervals.last_mut().unwrap() =
                txf_intervals.last().copied().unwrap().max(intervals[i].1);
        } else {
            txf_intervals.push(intervals[i].0);
            txf_intervals.push(intervals[i].1);
        }
    }

    let mut count = 0;

    for id in ids {
        match txf_intervals.binary_search(&id) {
            Ok(_) => count += 1,
            Err(v) => count += v % 2,
        }
    }

    count
}
