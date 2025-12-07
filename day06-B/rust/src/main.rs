use std::{
    fs::File,
    io::{BufRead, BufReader, Result},
};

fn main() -> Result<()> {
    let file = File::open("../input.txt")?;
    let reader = BufReader::new(file);
    let mut last_line = None::<String>;
    let mut mat = Vec::new();
    for line in reader.lines() {
        let line = line?;
        if let Some(prev) = last_line.take() {
            mat.push(prev);
        }

        last_line = Some(line);
    }

    let operations = last_line
        .unwrap()
        .split_whitespace()
        .map(|v| v.as_bytes()[0])
        .collect::<Vec<_>>();

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

/*

123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +


= 175 * 581 * 32 = 3257920
= 1 * (100 + 70 + 5) * (500 + 80 + 1) * (30 + 2)
= sum over z: (5z + 70z + 100z)

*/

fn solve(matrix: Vec<String>, operations: Vec<u8>) -> u64 {
    let mut result = 0;
    let mut op_idx = 0;
    let mut cur = if operations[op_idx] == b'+' { 0 } else { 1 };
    //println!("----------: New operation {}", operations[op_idx] as char);

    for j in 0..matrix[0].len() {
        let mut spaces = 0;
        let mut num = 0u32;
        for i in 0..matrix.len() {
            let digit = matrix[i].as_bytes()[j];
            spaces += (digit == b' ') as usize;
            num = if digit == b' ' {
                num
            } else {
                num * 10 + (digit - b'0') as u32
            };
        }

        if spaces != matrix.len() {
            //println!("Num is {}", num);
            operate(num, &mut cur, operations[op_idx]);
            continue;
        }

        //println!("Previous result: {}", cur);
        result += cur;
        op_idx += 1;
        cur = if operations[op_idx] == b'+' { 0 } else { 1 };
        //println!("----------: New operation {}", operations[op_idx] as char);
    }

    //println!("Previous result: {}", cur);

    result + cur
}
