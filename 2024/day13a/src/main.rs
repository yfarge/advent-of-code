fn main() {
    println!(
        "{:#?}",
        include_str!("../input.txt")
            .split("\n\n")
            .filter_map(|machine| {
                machine
                    .lines()
                    .filter_map(|line| {
                        line.split_once(":").and_then(|(_, values)| {
                            values.split_once(",").and_then(|(x_split, y_split)| {
                                let x = x_split.get(3..)?.parse::<isize>().ok()?;
                                let y = y_split.get(3..)?.parse::<isize>().ok()?;
                                Some((x, y))
                            })
                        })
                    })
                    .collect::<Vec<_>>()
                    .try_into()
                    .ok()
            })
            .map(|[a, b, p]: [(isize, isize); 3]| {
                let d = a.0 * b.1 - b.0 * a.1;
                let (dx, dy) = (p.0 * b.1 - b.0 * p.1, a.0 * p.1 - p.0 * a.1);
                let (x, y) = (dx / d, dy / d);

                if (a.0 * x + b.0 * y, a.1 * x + b.1 * y) == p {
                    return x * 3 + y;
                }
                0
            })
            .sum::<isize>()
    );
}
