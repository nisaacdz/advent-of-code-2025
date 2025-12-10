use itertools::Itertools;

fn main() {
    let input = include_str!("..\\..\\input.txt");

    let red_tiles = input
        .split_whitespace()
        .map(|v| {
            let mut v = v.split(",");
            (
                v.next().unwrap().parse::<i64>().unwrap(),
                v.next().unwrap().parse::<i64>().unwrap(),
            )
        })
        .collect::<Vec<_>>();

    //println!("{red_tiles:?}");

    let result = solve(red_tiles);

    println!("Result is : {result}");
}

fn solve(red_tiles: Vec<(i64, i64)>) -> i64 {
    red_tiles
        .into_iter()
        .tuple_combinations()
        .filter(|((ax, ay), (bx, by))| ax != bx && ay != by)
        .map(|(a, b)| area(a, b))
        .max()
        .unwrap()
}

fn area(p1: (i64, i64), p2: (i64, i64)) -> i64 {
    (i64::abs(p1.0 - p2.0) + 1) * (i64::abs(p1.1 - p2.1) + 1)
}
