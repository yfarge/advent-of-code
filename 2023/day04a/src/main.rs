use std::collections::HashSet;

fn main() {
    let mut nums: Vec<usize> = vec![];
    let mut winning_nums: HashSet<usize> = HashSet::new();

    println!(
        "{:#?}",
        include_str!("../input.txt")
            .lines()
            .filter_map(|line| line.split(":").last())
            .map(|line| {
                nums.clear();
                winning_nums.clear();
                let mut nw = line.split("|").map(|s| {
                    s.trim()
                        .split_whitespace()
                        .map(|num| num.parse::<usize>().unwrap())
                });
                nums.extend(nw.next().unwrap());
                winning_nums.extend(nw.next().unwrap());

                let count = nums.iter().filter(|n| winning_nums.contains(n)).count() as u32;
                return 2usize.pow(count).saturating_div(2);
            })
            .sum::<usize>()
    );
}
