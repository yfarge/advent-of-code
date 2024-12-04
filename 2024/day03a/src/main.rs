use regex::Regex;

fn main() {
    let re = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();
    println!(
        "{:#?}",
        re.captures_iter(include_str!("../input.txt"))
            .map(
                |captures| captures.get(1).unwrap().as_str().parse::<isize>().unwrap()
                    * captures.get(2).unwrap().as_str().parse::<isize>().unwrap()
            )
            .sum::<isize>()
    );
}
