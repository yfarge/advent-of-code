use std::collections::HashSet;

fn main() {
    let map = include_bytes!("../input.txt")
        .split(|&b| b == b'\n')
        .collect::<Vec<_>>();

    let (mut y, mut x) = (0..map.len() as isize)
        .flat_map(|y| (0..map[0].len() as isize).map(move |x| (y, x)))
        .find(|(y, x)| map[*y as usize][*x as usize] == b'^')
        .unwrap();

    let mut ord = 0;
    let mut offset = (-1, 0);
    let mut visited: HashSet<(isize, isize)> = HashSet::new();
    while let Some(cell) = map
        .get((y + offset.0) as usize)
        .and_then(|row| row.get((x + offset.1) as usize))
    {
        visited.insert((y, x));
        if *cell == b'#' {
            ord = (ord + 1) % 4
        };

        match ord {
            0 => offset = (-1, 0),
            1 => offset = (0, 1),
            2 => offset = (1, 0),
            3 => offset = (0, -1),
            _ => panic!(),
        }

        (y, x) = (y + offset.0, x + offset.1);
    }
    visited.insert((y, x));

    println!("{:#?}", visited.len());
}
