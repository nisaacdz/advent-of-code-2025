use std::{
    fs::File,
    io::{BufRead, BufReader, Result},
};

fn main() -> Result<()> {
    let file = File::open("../input.txt")?;
    let reader = BufReader::new(file);
    let mut mat = Vec::new();
    for line in reader.lines() {
        let line = line?;
        mat.push(line.into_bytes());
    }

    let result = solve(mat);
    println!("Result: {}", result);

    Ok(())
}

fn solve(mut mat: Vec<Vec<u8>>) -> u64 {
    let mut result = 0;

    let idx = mat[0].iter().position(|&c| c == b'S').unwrap();
    mat[0][idx] = b'|';

    for i in 1..mat.len() {
        for j in 0..mat[i].len() {
            if mat[i][j] == b'.' {
                if mat[i - 1][j] == b'|' {
                    mat[i][j] = b'|';
                }
            } else if mat[i][j] == b'^' {
                if mat[i - 1][j] == b'|' {
                    result += 1;
                    if j > 0 && mat[i][j - 1] == b'.' {
                        mat[i][j - 1] = b'|'
                    };
                    if i + 1 < mat[0].len() && mat[i][j + 1] == b'.' {
                        mat[i][j + 1] = b'|'
                    };
                }
            }
        }
    }

    //let mat = mat.into_iter().map(|v| String::from_utf8(v).unwrap()).collect::<Vec<_>>();
    //println!("{:#?}", mat);

    result
}
