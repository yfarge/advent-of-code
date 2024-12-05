fn main() {
    let grid: Vec<Vec<&str>> = include_str!("../input.txt")
        .lines()
        .map(|line| line.split("").collect())
        .collect();

    println!(
        "{:#?}",
        (0..grid.len() as isize)
            .flat_map(|r| (0..grid[0].len() as isize).map(move |c| (r, c)))
            .map(|(r, c)| {
                [
                    (r + 1, c + 1), // Center
                    (r, c),         // NW
                    (r + 2, c),     // SW
                    (r, c + 2),     // NE
                    (r + 2, c + 2), // SE
                ]
            })
            .filter(|offsets| {
                let mut path = offsets.iter().map(|(r, c)| {
                    grid.get(*r as usize)
                        .and_then(|line| line.get(*c as usize).copied())
                        .unwrap_or_default()
                });

                if path.next().is_none_or(|c| c != "A") {
                    return false;
                };

                let path = path.collect::<String>();
                path == "MSMS" || path == "MMSS" || path == "SSMM" || path == "SMSM"
            })
            .count()
    );
}
