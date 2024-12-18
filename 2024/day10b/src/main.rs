use std::collections::{HashSet, VecDeque};

fn main() {
    let data = include_bytes!("../input.txt")
        .split(|&b| b == b'\n')
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>();

    let mut queue = VecDeque::<(isize, isize)>::new();
    let mut visited = HashSet::<(isize, isize)>::new();

    let directions = [(-1, 0), (0, -1), (1, 0), (0, 1)];

    println!(
        "{:#?}",
        (0..data.len() as isize)
            .flat_map(|y| (0..data[0].len() as isize).map(move |x| (x, y)))
            .filter(|(x, y)| {
                atoi::ascii_to_digit::<usize>(data[*y as usize][*x as usize])
                    .is_some_and(|height| height == 0)
            })
            .map(|trailhead| {
                queue.clear();
                visited.clear();
                queue.push_back(trailhead);
                let mut count = 0;

                while let Some((current_col, current_row)) = queue.pop_front() {
                    visited.insert((current_col, current_row));
                    if let Some(current_height) = data
                        .get(current_row as usize)
                        .and_then(|row| row.get(current_col as usize))
                        .and_then(|&current_height| atoi::ascii_to_digit::<usize>(current_height))
                    {
                        if current_height == 9 {
                            count += 1;
                            continue;
                        }

                        for &(dx, dy) in directions.iter() {
                            let (next_col, next_row) = (current_col + dx, current_row + dy);

                            if visited.contains(&(next_col, next_row)) {
                                continue;
                            }

                            if let Some(next_height) = data
                                .get(next_row as usize)
                                .and_then(|row| row.get(next_col as usize))
                                .and_then(|&next_height| atoi::ascii_to_digit::<usize>(next_height))
                            {
                                if next_height == current_height + 1 {
                                    queue.push_back((next_col, next_row));
                                }
                            }
                        }
                    }
                }

                count
            })
            .sum::<usize>()
    );
}
