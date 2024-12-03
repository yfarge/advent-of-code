fn is_safe(report: &Vec<isize>) -> bool {
    for i in 0..report.len() {
        let nums = report[0..i]
            .iter()
            .chain(&report[i + 1..])
            .collect::<Vec<_>>();

        let increasing = nums.windows(2).all(|pair| {
            let diff = pair[1] - pair[0];
            pair[0] < pair[1] && diff >= 1 && diff <= 3
        });

        let decreasing = nums.windows(2).all(|pair| {
            let diff = pair[0] - pair[1];
            pair[0] > pair[1] && diff >= 1 && diff <= 3
        });

        if decreasing || increasing {
            return true;
        }
    }
    false
}

fn main() {
    println!(
        "{:#?}",
        include_str!("../input.txt")
            .lines()
            .map(|line| line
                .split_whitespace()
                .filter_map(|level| level.parse::<isize>().ok())
                .collect::<Vec<isize>>())
            .filter(is_safe)
            .count()
    )
}
