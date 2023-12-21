use std::collections::HashSet;

fn main() {
    let mut nums: Vec<usize> = vec![];
    let mut winning_nums: HashSet<usize> = HashSet::new();
    let mut card_count: [usize; 208] = [1; 208];

    println!(
        "{:#?}",
        include_str!("../input.txt")
            .lines()
            .filter_map(|line| line.split(":").last())
            .enumerate()
            .map(|(i, line)| {
                nums.clear();
                winning_nums.clear();

                let mut nw = line.split("|").map(|s| {
                    s.trim()
                        .split_whitespace()
                        .filter_map(|num| num.parse::<usize>().ok())
                });

                nums.extend(nw.next().unwrap());
                winning_nums.extend(nw.next().unwrap());

                let matches = nums.iter().filter(|n| winning_nums.contains(n)).count();

                (i + 1..=i + matches).for_each(|pos| card_count[pos] += 1 * card_count[i]);
                card_count[i]
            })
            .sum::<usize>()
    );
}
