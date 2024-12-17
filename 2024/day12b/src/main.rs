use std::collections::{HashSet, VecDeque};

fn main() {
    let map = include_bytes!("../input.txt")
        .split(|&b| b == b'\n')
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>();

    let mut queue = VecDeque::with_capacity(5000);
    let mut visited = HashSet::with_capacity(20000);

    let directions = [(-1, 0), (0, -1), (1, 0), (0, 1)];
    let diagonals = [(-1, -1), (1, -1), (1, 1), (-1, 1)];

    println!(
        "{:#?}",
        (0..map.len() as isize)
            .flat_map(|y| (0..map[0].len() as isize).map(move |x| (x, y)))
            .map(|coord| {
                queue.clear();
                queue.push_back(coord);
                let (mut area, mut sides) = (0, 0);

                while let Some((col, row)) = queue.pop_front() {
                    if !visited.insert((col, row)) {
                        continue;
                    };

                    area += 1;

                    let is_valid = |x: isize, y: isize| {
                        if let Some(&plot) = map.get(y as usize).and_then(|row| row.get(x as usize))
                        {
                            return plot == map[row as usize][col as usize];
                        };
                        false
                    };

                    for (i, &(dx, dy)) in directions.iter().enumerate() {
                        if is_valid(col + dx, row + dy) {
                            if !visited.contains(&(col + dx, row + dy)) {
                                queue.push_back((col + dx, row + dy));
                            }
                        } else {
                            let (adj_dx, adj_dy) = directions[(i + 1) % 4];
                            let is_adjacent_valid = is_valid(col + adj_dx, row + adj_dy);

                            let (diag_dx, diag_dy) = diagonals[i];
                            let is_diagonal_valid = is_valid(col + diag_dx, row + diag_dy);

                            if !is_adjacent_valid || is_diagonal_valid {
                                sides += 1;
                            }
                        }
                    }
                }
                area * sides
            })
            .sum::<usize>()
    );
}
