use std::fs::File;
use std::io::{self, BufRead};

fn main() -> std::io::Result<()> {
    // println!("Hello, world!");
    let file = File::open("../input.txt")?;
    let buf = io::BufReader::new(file);

    let lines: Vec<Vec<u8>> = buf.lines().map(|l| l.unwrap().into_bytes()).collect();

    println!("Read {} lines", lines.len());

    let result = solve(lines);

    println!("Result: {}", result);

    Ok(())
}

const DIRS: [(isize, isize); 8] = [
    (1, 0),
    (0, 1),
    (-1, 0),
    (0, -1),
    (1, 1),
    (1, -1),
    (-1, 1),
    (-1, -1),
];

fn solve(lines: Vec<Vec<u8>>) -> u64 {
    let (n, m) = (lines.len() as isize, lines[0].len() as isize);

    let lines = RefCell::new(lines);

    let new_point = |x: isize, y: isize| Point {
        lines: &lines,
        x,
        y,
    };

    use std::cell::RefCell;
    use std::cmp::*;

    #[derive(PartialEq, Eq, Clone, Copy)]
    struct Point<'a> {
        lines: &'a RefCell<Vec<Vec<u8>>>,
        x: isize,
        y: isize,
    }

    impl<'a> std::fmt::Debug for Point<'a> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "Point({}, {}) nb={}", self.x, self.y, self.nb())
        }
    }

    impl<'a> Point<'a> {
        fn nb(&self) -> usize {
            let mut count = 0;

            for (dx, dy) in DIRS {
                let np = Self {
                    lines: self.lines,
                    x: self.x + dx,
                    y: self.y + dy,
                };

                if np.is_within_bounds() && np.is_roll() {
                    count += 1;
                }
            }

            count
        }

        fn is_within_bounds(&self) -> bool {
            self.x >= 0
                && self.x < self.lines.borrow().len() as isize
                && self.y >= 0
                && self.y < self.lines.borrow()[0].len() as isize
        }

        fn is_roll(&self) -> bool {
            self.lines.borrow()[self.x as usize][self.y as usize] == b'@'
        }

        fn can_be_removed(&self) -> bool {
            self.is_within_bounds() && self.is_roll() && self.nb() < 4
        }
    }

    impl<'a> PartialOrd for Point<'a> {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(
                self.nb()
                    .cmp(&other.nb())
                    .then_with(|| (self.x, self.y).cmp(&(other.x, other.y))),
            )
        }
    }

    impl<'a> Ord for Point<'a> {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.nb()
                .cmp(&other.nb())
                .then_with(|| (self.x, self.y).cmp(&(other.x, other.y)))
        }
    }

    let mut points = std::collections::btree_set::BTreeSet::new();
    let mut z = 0;
    for i in 0..n {
        for j in 0..m {
            let p = new_point(i, j);
            if p.is_roll() {
                z += 1;
                points.insert(p);
            }
        }
    }

    let mut ans = 0;

    //println!("Remaining points: {:?}", points.iter().map(|p| p.nb()).collect::<Vec<_>>());

    println!(
        "total points: {}, {}, {}",
        points.len(),
        z,
        lines
            .borrow()
            .iter()
            .flatten()
            .filter(|&&c| c == b'@')
            .count()
    );

    while matches!(points.first(), Some(p) if p.can_be_removed()) {
        //println!("Next to remove: {:?}", points.first());
        ans += 1;
        let p = points.first().copied().unwrap();
        let nbs: Vec<Point> = DIRS
            .iter()
            .map(|(dx, dy)| new_point(p.x + dx, p.y + dy))
            .filter(|np| np.is_within_bounds() && np.is_roll())
            .collect();

        for nb in nbs.iter() {
            points.remove(nb);
        }

        points.remove(&p);

        lines.borrow_mut()[p.x as usize][p.y as usize] = b'.';

        for nb in nbs {
            points.insert(nb);
        }
    }

    //println!("Remaining points: {:?}", points.iter().map(|p| p.nb()).collect::<Vec<_>>());

    // println!("{:#?}", lines.borrow().iter().map(|l| String::from_utf8_lossy(l)).collect::<Vec<_>>());

    ans
}
