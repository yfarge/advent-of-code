fn map_digit_to_numeric_char(digit: &str) -> Option<char> {
    if digit.contains("one") {
        return Some('1');
    } else if digit.contains("two") {
        return Some('2');
    } else if digit.contains("three") {
        return Some('3');
    } else if digit.contains("four") {
        return Some('4');
    } else if digit.contains("five") {
        return Some('5');
    } else if digit.contains("six") {
        return Some('6');
    } else if digit.contains("seven") {
        return Some('7');
    } else if digit.contains("eight") {
        return Some('8');
    } else if digit.contains("nine") {
        return Some('9');
    } else {
        return None;
    }
}

fn get_value(s: &str) -> u32 {
    let mut result = String::with_capacity(2);
    let mut digit = String::with_capacity(5);

    for c in s.chars() {
        if let Some(numeric_char) = map_digit_to_numeric_char(&digit) {
            result.push(numeric_char);
            digit.clear();
            break;
        }

        if c.is_numeric() {
            result.push(c);
            break;
        }

        digit.push(c);
    }

    for c in s.chars().rev() {
        if let Some(numeric_char) =
            map_digit_to_numeric_char(&digit.chars().rev().collect::<String>())
        {
            result.push(numeric_char);
            digit.clear();
            break;
        }

        if c.is_numeric() {
            result.push(c);
            break;
        }

        digit.push(c);
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
