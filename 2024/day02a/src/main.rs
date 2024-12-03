fn main() {
    println!(
        "{:#?}",
        include_str!("../input")
            .lines()
            .map(|line| line
                .split_whitespace()
                .filter_map(|level| level.parse::<isize>().ok())
                .collect::<Vec<_>>())
            .filter(|report| report
                .windows(2)
                .all(|pair| pair[0] < pair[1] || report.windows(2).all(|pair| pair[0] > pair[1])))
            .filter(|report| report.windows(2).all(|pair| {
                let diff = (pair[0] - pair[1]).abs();
                diff >= 1 && diff <= 3
            }))
            .count()
    );
}
