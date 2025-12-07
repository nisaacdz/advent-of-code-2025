use std::{fs::File, io::{BufRead, BufReader, Result}};

fn main() -> Result<()> {
    let file = File::open("../input.txt")?;
    let reader = BufReader::new(file);
    let mut last_line = None::<String>;
    let mut mat = Vec::new();
    for line in reader.lines() {
        let line = line?;
        if let Some(prev) = &last_line {
            let row = prev.split_whitespace().map(|s| s.trim().parse::<u32>().unwrap()).collect::<Vec<_>>();
            mat.push(row);
        }

        last_line = Some(line);
    }

    let operations = last_line.unwrap().split_whitespace().map(|v| v.as_bytes()[0]).collect::<Vec<_>>();

    let result = solve(mat, operations);
    println!("Result: {}", result);

    Ok(())
}

fn operate(value: u32, cur: &mut u64, op: u8) {
    match op {
        b'+' => *cur += value as u64,
        b'*' => *cur *= value as u64,
        _ => panic!("Unknown operation"),
    }
}

fn solve(matrix: Vec<Vec<u32>>, operations: Vec<u8>) -> u64 {
    let mut result = 0;

    for j in 0..operations.len() {
        let mut cur = if operations[j] == b'+' { 0 } else { 1 };
        for i in 0..matrix.len() {
            operate(matrix[i][j], &mut cur, operations[j]);
        }
        result += cur;
    }

    result
}