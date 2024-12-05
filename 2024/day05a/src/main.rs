use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("../input.txt");
    let (rules, pages) = input.split_once("\n\n").unwrap();
    let graph: HashMap<usize, HashSet<usize>> =
        rules.lines().fold(HashMap::new(), |mut acc, rule| {
            let (u, v) = rule.split_once("|").unwrap();
            acc.entry(u.parse().unwrap())
                .or_default()
                .insert(v.parse().unwrap());
            acc
        });

    println!(
        "{:#?}",
        pages
            .lines()
            .map(|line| line
                .split(",")
                .map(|page| page.parse::<usize>().unwrap())
                .collect::<Vec<_>>())
            .filter(|pages| {
                for (i, page) in pages.iter().enumerate() {
                    if let Some(dependencies) = graph.get(&page) {
                        if pages[0..i]
                            .iter()
                            .any(|page| dependencies.get(&page).is_some())
                        {
                            return false;
                        }
                    }
                }
                true
            })
            .map(|pages| pages[pages.len() / 2])
            .sum::<usize>()
    );
}
