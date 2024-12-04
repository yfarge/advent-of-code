use regex::Regex;

fn main() {
    let re = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)|(do\(\))|(don't\(\))").unwrap();
    let mut enabled = true;

    println!(
        "{:#?}",
        re.captures_iter(include_str!("../input.txt"))
            .filter(|captures| {
                if captures.get(0).unwrap().as_str() == "do()" {
                    enabled = true;
                    return false;
                } else if captures.get(0).unwrap().as_str() == "don't()" {
                    enabled = false;
                }
                enabled
            })
            .map(
                |captures| captures.get(1).unwrap().as_str().parse::<isize>().unwrap()
                    * captures.get(2).unwrap().as_str().parse::<isize>().unwrap()
            )
            .sum::<isize>()
    );
}
