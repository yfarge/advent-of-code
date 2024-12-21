use std::ops::{Div, Mul};

fn calculate_possible_wins(time: f64, record: f64) -> usize {
    let b = time.div(2f64).powi(2);
    let operand = (record + 1f64 - b).abs();
    let min = (time.div(2f64) - f64::sqrt(operand)).ceil();
    let max = (time.div(2f64) + f64::sqrt(operand)).floor();
    (max - min + 1.0) as usize
}

fn main() {
    let td = include_bytes!("../input.txt")
        .split(|&b| b == b'\n')
        .map(|line| {
            line.split(|&b| b == b':')
                .last()
                .unwrap()
                .split(|&b| b.is_ascii_whitespace())
                .fold(0usize, |mut acc, num| {
                    if let Some(num) = atoi::atoi::<usize>(num) {
                        acc = acc.mul(10usize.pow(num.ilog10() + 1)) + num
                    }
                    acc
                })
        })
        .collect::<Vec<_>>();

    println!("{:#?}", calculate_possible_wins(td[0] as f64, td[1] as f64))
}
