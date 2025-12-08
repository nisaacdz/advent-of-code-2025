use std::{
    fs::File,
    io::{BufRead, BufReader, Result},
};

fn main() -> Result<()> {
    let file = File::open("../input2.txt")?;
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
    let (n, m) = (mat.len(), mat[0].len());
    let mut dp = vec![vec![0; mat[0].len()]; mat.len()];

    let idx = mat[0].iter().position(|&c| c == b'S').unwrap();
    mat[0][idx] = b'|';
    
    dp[n - 1].iter_mut().for_each(|v| *v = 1);

    for i in (0..(n - 1)).rev() {
        for j in 0..m {
            if mat[i][j] == b'^' {
                dp[i][j] += if j > 0 { dp[i + 1][j - 1] } else { 0 };
                dp[i][j] += if j + 1 < m { dp[i + 1][j + 1] } else { 0 };
            } else {
                dp[i][j] += dp[i + 1][j];
            }
        }
    }

    //let mat = mat.into_iter().map(|v| String::from_utf8(v).unwrap()).collect::<Vec<_>>();
    //dp.iter().for_each(|r| println!("{:?}", r));

    dp[0][idx]
}
