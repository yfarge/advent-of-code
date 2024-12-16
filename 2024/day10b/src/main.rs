use std::collections::{HashSet, VecDeque};

fn main() {
    let data = include_bytes!("../input.txt")
        .split(|&b| b == b'\n')
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>();

    let mut queue = VecDeque::<(isize, isize)>::new();
    let mut visited = HashSet::<(isize, isize)>::new();

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
                    if let Some(current_height) = atoi::ascii_to_digit::<usize>(
                        data[current_row as usize][current_col as usize],
                    ) {
                        if current_height == 9 {
                            count += 1;
                            continue;
                        }

                        for (next_col, next_row) in [
                            (current_col - 1, current_row),
                            (current_col, current_row - 1),
                            (current_col + 1, current_row),
                            (current_col, current_row + 1),
                        ] {
                            if next_row >= 0
                                && next_col >= 0
                                && next_col < data[0].len() as isize
                                && next_row < data.len() as isize
                                && !visited.contains(&(next_col, next_row))
                            {
                                if let Some(next_height) = atoi::ascii_to_digit::<usize>(
                                    data[next_row as usize][next_col as usize],
                                ) {
                                    if next_height == current_height + 1 {
                                        queue.push_back((next_col, next_row));
                                    }
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
