use std::collections::HashMap;

fn main() {
    let map = include_bytes!("../input.txt")
        .split(|&b| b == b'\n')
        .collect::<Vec<_>>();

    let start = (0..map.len() as isize)
        .flat_map(|y| (0..map[0].len() as isize).map(move |x| (y, x)))
        .find(|(y, x)| map[*y as usize][*x as usize] == b'^')
        .unwrap();

    let mut counter: HashMap<(isize, isize), usize> = HashMap::new();

    println!(
        "{:#?}",
        (0..map.len() as isize)
            .flat_map(|y| (0..map[0].len() as isize).map(move |x| (y, x)))
            .filter(|coord| {
                let (mut y, mut x) = start;
                let mut ord = 0;
                let mut offset = (-1, 0);
                counter.clear();
                while let Some(cell) = map
                    .get((y + offset.0) as usize)
                    .and_then(|row| row.get((x + offset.1) as usize))
                {
                    *counter.entry((y, x)).or_insert(0) += 1;
                    if *counter.entry((y, x)).or_default() > 4 {
                        return true;
                    }

                    if *cell == b'#' || *coord == (y + offset.0, x + offset.1) {
                        ord = (ord + 1) % 4;
                        match ord {
                            0 => offset = (-1, 0),
                            1 => offset = (0, 1),
                            2 => offset = (1, 0),
                            3 => offset = (0, -1),
                            _ => panic!(),
                        }
                    } else {
                        (y, x) = (y + offset.0, x + offset.1)
                    }
                }
                false
            })
            .count()
    );
}
