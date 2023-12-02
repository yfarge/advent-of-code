fn get_value(s: &str) -> u32 {
    let mut result = String::with_capacity(2);

    for c in s.chars() {
        if c.is_numeric() {
            result.push(c);
            break;
        }
    }

    for c in s.chars().rev() {
        if c.is_numeric() {
            result.push(c);
            break;
        }
    }

    return result.parse().unwrap();
}

fn main() {
    println!(
        "{}",
        include_str!("../input.txt")
            .lines()
            .map(|s| get_value(s))
            .sum::<u32>()
    );
}
