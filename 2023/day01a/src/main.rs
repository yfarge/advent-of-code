pub fn main() {
    println!(
        "{}",
        include_str!("../input.txt")
            .lines()
            .map(|s| s.replace(|c: char| !c.is_numeric(), ""))
            .map(
                |n| format!("{}{}", n.chars().next().unwrap(), n.chars().last().unwrap())
                    .parse::<u32>()
                    .unwrap()
            )
            .sum::<u32>()
    );
}

