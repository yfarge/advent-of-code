use std::collections::{HashMap, HashSet};

fn main() {
    let map = include_bytes!("../input.txt")
        .split(|&b| b == b'\n')
        .collect::<Vec<_>>();

    let antennas: HashMap<u8, Vec<(isize, isize)>> = (0..map.len())
        .flat_map(|y| (0..map[0].len()).map(move |x| (x, y)))
        .filter_map(|(x, y)| {
            let cell = map
                .get(y)
                .and_then(|row| row.get(x))
                .filter(|cell| cell.is_ascii_alphanumeric())?;
            Some((*cell, (x as isize, y as isize)))
        })
        .fold(HashMap::new(), |mut acc, (key, value)| {
            acc.entry(key).or_insert(Vec::new()).push(value);
            acc
        });

    println!(
        "{:#?}",
        antennas
            .values()
            .map(|coords| (0..coords.len())
                .flat_map(move |i| (i + 1..coords.len()).map(move |j| (coords[i], coords[j]))))
            .fold(HashSet::<(isize, isize)>::new(), |mut acc, pairs| {
                for (a, b) in pairs {
                    acc.insert(a);
                    acc.insert(b);
                    let (dx, dy) = (b.0 - a.0, b.1 - a.1);
                    let mut c = (a.0 - dx, a.1 - dy);
                    while let Some(_) = map.get(c.1 as usize).and_then(|row| row.get(c.0 as usize))
                    {
                        acc.insert(c);
                        c = (c.0 - dx, c.1 - dy);
                    }

                    let mut d = (b.0 + dx, b.1 + dy);
                    while let Some(_) = map.get(d.1 as usize).and_then(|row| row.get(d.0 as usize))
                    {
                        acc.insert(d);
                        d = (d.0 + dx, d.1 + dy);
                    }
                }
                acc
            })
            .len()
    );
}
