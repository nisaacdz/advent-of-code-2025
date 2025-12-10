use glam::IVec3;
use itertools::Itertools;
use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::complete::{self, line_ending},
    multi::separated_list1,
};

fn main() {
    let input = include_str!("..\\..\\input.txt");

    let result = process(input);

    println!("Result is : {result}");
}

pub fn process(input: &str) -> i64 {
    let (_, positions) = parse(input).unwrap();
    let output = groups(positions);
    output
}

fn groups(positions: Vec<IVec3>) -> i64 {
    let mut connections: Vec<Vec<IVec3>> = vec![];
    let mut last = None::<(IVec3, IVec3)>;
    for (a, b, _) in positions
        .iter()
        .tuple_combinations()
        .map(|(a, b)| (a, b, a.as_vec3().distance(b.as_vec3())))
        .sorted_by(|a, b| a.2.partial_cmp(&b.2).unwrap())
    {
        let matches = [
            connections.iter().position(|v| v.contains(a)),
            connections.iter().position(|v| v.contains(b)),
        ];
        match matches {
            [None, None] => {
                let _ = last.insert((*a, *b));
                connections.push(vec![*a, *b]);
            }
            [Some(index), None] => {
                let _ = last.insert((*a, *b));
                connections[index].push(*b);
            }
            [None, Some(index)] => {
                let _ = last.insert((*a, *b));
                connections[index].push(*a);
            }
            [Some(index_a), Some(index_b)] if index_a != index_b => {
                let _ = last.insert((*a, *b));
                let a = connections.remove(index_a.max(index_b));
                let b = connections.remove(index_b.min(index_a));
                let new_cluster = a
                    .into_iter()
                    .chain(b.into_iter())
                    .unique()
                    .collect::<Vec<IVec3>>();
                connections.push(new_cluster);
            }
            [Some(_), Some(_)] => {}
        }

        if connections.len() == 1 && connections[0].len() == positions.len() {
            println!("Terminating due to condition");
            break;
        }
    }
    connections.sort_by(|a, b| b.len().cmp(&a.len()));

    last.map(|(u, v)| u.x as i64 * v.x as i64).unwrap()
}

fn parse(input: &str) -> IResult<&str, Vec<IVec3>> {
    separated_list1(
        line_ending,
        separated_list1(tag(","), complete::i32).map(|v| IVec3::from_slice(&v)),
    )
    .parse(input)
}
