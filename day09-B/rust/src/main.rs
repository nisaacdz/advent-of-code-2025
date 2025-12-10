use std::cmp::Reverse;

use geo::{Contains, LineString, Polygon, Rect, coord};
use itertools::Itertools;

fn main() {
    let input = include_str!("..\\..\\input.txt");

    let red_tiles = input
        .split_whitespace()
        .map(|v| {
            let mut v = v.split(",");
            (
                v.next().unwrap().parse::<f64>().unwrap(),
                v.next().unwrap().parse::<f64>().unwrap(),
            )
        })
        .collect::<Vec<_>>();

    //println!("{red_tiles:?}");

    let result = solve(red_tiles);

    println!("Result is : {result}");
}

fn solve(red_tiles: Vec<(f64, f64)>) -> f64 {
    let polygon = Polygon::new(
        LineString::from_iter(red_tiles.iter().copied().chain(red_tiles.first().copied())),
        vec![],
    );
    red_tiles
        .into_iter()
        .tuple_combinations()
        .sorted_by_key(|&(a, b)| Reverse(area(a, b) as i64))
        .find(|&((ax, ay), (bx, by))| {
            polygon.contains(&Rect::new(coord! { x: ax, y: ay }, coord! { x: bx, y: by}))
        })
        .map(|(a, b)| area(a, b))
        .unwrap()
}

fn area(p1: (f64, f64), p2: (f64, f64)) -> f64 {
    (f64::abs(p1.0 - p2.0) + 1.0) * (f64::abs(p1.1 - p2.1) + 1.0)
}
