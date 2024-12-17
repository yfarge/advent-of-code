use std::collections::{HashSet, VecDeque};

fn main() {
    let map = include_bytes!("../input.txt")
        .split(|&b| b == b'\n')
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>();

    let mut queue = VecDeque::with_capacity(5000);
    let mut visited = HashSet::with_capacity(20000);

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

                    for (next_col, next_row) in [
                        (col - 1, row),
                        (col, row - 1),
                        (col + 1, row),
                        (col, row + 1),
                    ] {
                        if 0 <= next_col
                            && 0 <= next_row
                            && (next_col as usize) < map[0].len()
                            && (next_row as usize) < map.len()
                            && map[next_row as usize][next_col as usize]
                                == map[row as usize][col as usize]
                        {
                            if !visited.contains(&(next_col, next_row)) {
                                queue.push_back((next_col, next_row));
                            }
                        } else {
                            perimeter += 1;
                        }
                    }
                }
                area * perimeter
            })
            .sum::<usize>()
    );
}
