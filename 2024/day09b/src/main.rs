use std::{
    collections::HashMap,
    ops::{Div, Mul},
};

fn find_span(v: &[isize], n: usize) -> Option<(usize, usize)> {
    if n == 0 || v.len() < n {
        return None;
    }

    let mut left = 0;

    for right in 0..v.len() {
        if v[right] == -1 {
            if right - left + 1 == n {
                return Some((left, right));
            }
        } else {
            left = right + 1;
        }
    }

    None
}

fn compact(v: &mut Vec<isize>, h: &HashMap<isize, usize>) {
    let mut right = v.len() - 1;
    while right > 0 {
        if v[right] != -1 {
            let length = *h.get(&v[right]).unwrap();

            if let Some((lb, rb)) = find_span(&v[0..right], length) {
                let (span_slice, file_slice) = v.split_at_mut(right - length + 1);
                span_slice[lb..=rb].swap_with_slice(&mut file_slice[..length]);
            }
            right -= length - 1
        }
        right -= if right > 0 { 1 } else { 0 }
    }
}

fn main() {
    let (mut file_system, counter) = include_bytes!("../input.txt").iter().enumerate().fold(
        (Vec::new(), HashMap::new()),
        |(mut v, mut h), (index, &num)| {
            if let Some(count) = atoi::ascii_to_digit::<usize>(num) {
                if index % 2 == 0 {
                    v.extend(vec![index.div(2) as isize; count]);
                    h.insert(index.div(2) as isize, count);
                } else {
                    v.extend(vec![-1; count])
                }
            }
            (v, h)
        },
    );

    compact(&mut file_system, &counter);
    println!(
        "{:?}",
        file_system
            .iter()
            .enumerate()
            .fold(0, |mut acc, (index, &file_id)| {
                if file_id < 0 {
                    return acc;
                }
                acc += index.mul(file_id as usize);
                acc
            })
    )
}
