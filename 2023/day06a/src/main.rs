use std::{ops::Div, usize};

fn calculate_possible_wins(time: f32, record: f32) -> usize {
    let b = time.div(2f32).powi(2);
    let operand = (record + 1f32 - b).abs();
    let min = (time.div(2f32) - f32::sqrt(operand)).ceil();
    let max = (time.div(2f32) + f32::sqrt(operand)).floor();
    (max - min + 1.0) as usize
}

fn main() {
    let td = include_str!("../input.txt")
        .lines()
        .filter_map(|line| line.split(":").last())
        .map(|nums| {
            nums.split_whitespace()
                .filter_map(|num| num.parse::<f32>().ok())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    println!(
        "{:#?}",
        (0..td[0].len())
            .map(|race| calculate_possible_wins(td[0][race], td[1][race]))
            .product::<usize>()
    )
}
