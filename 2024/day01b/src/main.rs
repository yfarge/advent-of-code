use std::{collections::HashMap, ops::Mul};

fn main() {
    let mut list_one: Vec<usize> = Vec::new();
    let mut counter: HashMap<usize, usize> = HashMap::new();

    include_str!("../input.txt").lines().for_each(|line| {
        let mut numbers = line.split_whitespace();
        let a = numbers.next().unwrap().parse::<usize>().unwrap();
        let b = numbers.next().unwrap().parse::<usize>().unwrap();
        list_one.push(a);
        *counter.entry(b).or_insert(0) += 1;
    });

    println!(
        "{:#?}",
        list_one
            .iter()
            .map(|number| number.mul(counter.get(number).unwrap_or(&0)))
            .sum::<usize>()
    );
}
