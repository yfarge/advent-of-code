use std::collections::{HashMap, HashSet};

fn generate_visited(map: &Vec<&[u8]>, start: &(isize, isize)) -> Vec<(isize, isize)> {
    let mut visited: HashSet<(isize, isize)> = HashSet::new();
    let mut ord = 0;
    let mut offset = (0, -1);
    let (mut x, mut y) = start;
    while let Some(cell) = map
        .get((y + offset.1) as usize)
        .and_then(|row| row.get((x + offset.0) as usize))
    {
        if *cell == b'#' {
            ord = (ord + 1) % 4;
            match ord {
                0 => offset = (0, -1),
                1 => offset = (1, 0),
                2 => offset = (0, 1),
                3 => offset = (-1, 0),
                _ => {}
            }
        } else {
            (x, y) = (x + offset.0, y + offset.1);
            visited.insert((x, y));
        }
    }
    visited.into_iter().collect()
}

fn main() {
    let map = include_bytes!("../input.txt")
        .split(|&b| b == b'\n')
        .collect::<Vec<_>>();

    let start = (0..map.len() as isize)
        .flat_map(|y| (0..map[0].len() as isize).map(move |x| (x, y)))
        .find(|(x, y)| map[*y as usize][*x as usize] == b'^')
        .unwrap();

    let visited = generate_visited(&map, &start);

    let mut counter: HashMap<(isize, isize), usize> = HashMap::new();

    println!(
        "{:#?}",
        visited
            .into_iter()
            .filter(|coord| {
                let (mut x, mut y) = start;
                let mut ord = 0;
                let mut offset = (0, -1);
                counter.clear();
                while let Some(cell) = map
                    .get((y + offset.1) as usize)
                    .and_then(|row| row.get((x + offset.0) as usize))
                {
                    if *counter.entry((x, y)).or_default() > 4 {
                        return true;
                    }

                    if *cell == b'#' || *coord == (x + offset.0, y + offset.1) {
                        ord = (ord + 1) % 4;
                        match ord {
                            0 => offset = (0, -1),
                            1 => offset = (1, 0),
                            2 => offset = (0, 1),
                            3 => offset = (-1, 0),
                            _ => {}
                        }
                    } else {
                        (x, y) = (x + offset.0, y + offset.1);
                        *counter.entry((x, y)).or_insert(0) += 1;
                    }
                }
                false
            })
            .count()
    );
}
