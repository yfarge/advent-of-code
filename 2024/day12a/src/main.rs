use std::collections::{HashSet, VecDeque};

fn main() {
    let map = include_bytes!("../input.txt")
        .split(|&b| b == b'\n')
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>();

    let mut queue = VecDeque::with_capacity(5000);
    let mut visited = HashSet::with_capacity(20000);

    let directions = [(-1, 0), (0, -1), (1, 0), (0, 1)];

    println!(
        "{:#?}",
        (0..map.len() as isize)
            .flat_map(|y| (0..map[0].len() as isize).map(move |x| (x, y)))
            .map(|coord| {
                queue.clear();
                queue.push_back(coord);
                let (mut area, mut perimeter) = (0, 0);

                while let Some((col, row)) = queue.pop_front() {
                    if !visited.insert((col, row)) {
                        continue;
                    };

                    area += 1;

                    for &(dx, dy) in directions.iter() {
                        let (next_col, next_row) = (col + dx, row + dy);
                        if map
                            .get(next_row as usize)
                            .and_then(|row| row.get(next_col as usize))
                            .map_or(true, |&plot| plot != map[row as usize][col as usize])
                        {
                            perimeter += 1;
                        } else if !visited.contains(&(next_col, next_row)) {
                            queue.push_back((next_col, next_row));
                        }
                    }
                }
                area * perimeter
            })
            .sum::<usize>()
    );
}
