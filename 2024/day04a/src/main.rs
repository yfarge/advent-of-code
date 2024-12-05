fn main() {
    let grid: Vec<Vec<&str>> = include_str!("../input.txt")
        .lines()
        .map(|line| line.split("").collect())
        .collect();

    println!(
        "{:#?}",
        (0..grid.len() as isize)
            .flat_map(|r| (0..grid[0].len() as isize).map(move |c| (r, c)))
            .flat_map(|(r, c)| {
                [
                    [(r, c), (r - 1, c + 1), (r - 2, c + 2), (r - 3, c + 3)],
                    [(r, c), (r, c + 1), (r, c + 2), (r, c + 3)],
                    [(r, c), (r + 1, c + 1), (r + 2, c + 2), (r + 3, c + 3)],
                    [(r, c), (r + 1, c), (r + 2, c), (r + 3, c)],
                ]
            })
            .filter(|offsets| {
                let path = offsets
                    .iter()
                    .map(|(r, c)| {
                        grid.get(*r as usize)
                            .and_then(|line| line.get(*c as usize).copied())
                            .unwrap_or_default()
                    })
                    .collect::<String>();
                path == "XMAS" || path == "SAMX"
            })
            .count()
    );
}
