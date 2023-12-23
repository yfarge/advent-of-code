use std::ops::Div;

fn calculate_possible_wins(time: f64, record: f64) -> usize {
    let b = time.div(2f64).powi(2);
    let operand = (record + 1f64 - b).abs();
    let min = (time.div(2f64) - f64::sqrt(operand)).ceil();
    let max = (time.div(2f64) + f64::sqrt(operand)).floor();
    (max - min + 1.0) as usize
}

fn main() {
    let td = include_str!("../input.txt")
        .lines()
        .filter_map(|line| line.split(":").last())
        .map(|nums| {
            nums.chars()
                .filter(|c| !c.is_whitespace())
                .collect::<String>()
                .split_whitespace()
                .filter_map(|num| num.parse::<f64>().ok())
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

