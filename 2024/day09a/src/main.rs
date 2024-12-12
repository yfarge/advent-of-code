use std::ops::{Div, Mul};

fn compact(v: &mut Vec<isize>) -> usize {
    let mut left = 0;
    let mut right = v.len() - 1;
    while left < right {
        if v[right] < 0 {
            right -= 1;
            continue;
        } else if v[left] >= 0 {
            left += 1;
            continue;
        } else {
            v.swap(left, right);
            left += 1;
            right -= 1;
        }
    }

    right
}

fn main() {
    let mut file_system = include_bytes!("../input.txt").iter().enumerate().fold(
        Vec::new(),
        |mut acc, (index, &value)| {
            let count = atoi::ascii_to_digit::<u8>(value).unwrap_or(0);
            let fill = if index % 2 == 0 {
                index.div(2) as isize
            } else {
                -1
            };
            for _ in 0..count {
                acc.push(fill);
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
                acc += (index as isize).mul(file_id);
                acc
            })
    )
}
