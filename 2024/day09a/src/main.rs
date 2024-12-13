use std::ops::{Div, Mul};

fn compact(v: &mut Vec<isize>) -> usize {
    let mut left = 0;
    let mut right = v.len() - 1;
    while left < right {
        if v[left] >= 0 {
            left += 1;
            continue;
        } else if v[right] < 0 {
            right -= 1;
            continue;
        } else {
            v[left] = v[right];
            left += 1;
            right -= 1;
        }
    }

    right
}

fn main() {
    let mut file_system = include_bytes!("../input.txt").iter().enumerate().fold(
        Vec::new(),
        |mut acc, (index, &num)| {
            if let Some(count) = atoi::ascii_to_digit::<usize>(num) {
                if index % 2 == 0 {
                    acc.extend(vec![index.div(2) as isize; count])
                } else {
                    acc.extend(vec![-1; count])
                }
            }
            acc
        },
    );

    let end = compact(&mut file_system);

    println!(
        "{:?}",
        file_system[0..=end]
            .iter()
            .enumerate()
            .fold(0, |mut acc, (index, &file_id)| {
                acc += index.mul(file_id as usize);
                acc
            })
    )
}
