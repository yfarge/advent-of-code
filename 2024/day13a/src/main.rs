use ahash::AHashMap;
use std::cmp::min;

fn backtrack(
    a: &(usize, usize),
    b: &(usize, usize),
    target: &(usize, usize),
    memo: &mut AHashMap<(usize, usize), usize>,
) -> usize {
    fn dfs(
        node: &(usize, usize),
        a: &(usize, usize),
        b: &(usize, usize),
        target: &(usize, usize),
        memo: &mut AHashMap<(usize, usize), usize>,
        total: usize,
    ) -> usize {
        if node == target {
            return total;
        }

        if let Some(&total) = memo.get(&node) {
            return total;
        }

        if node.0 > target.0 || node.1 > target.1 {
            return usize::MAX;
        }

        let result = min(
            dfs(&(node.0 + a.0, node.1 + a.1), a, b, target, memo, total + 3),
            dfs(&(node.0 + b.0, node.1 + b.1), a, b, target, memo, total + 1),
        );

        memo.insert(*node, result);

        return result;
    }

    return dfs(&(0, 0), a, b, target, memo, 0);
}

fn main() {
    let machines: Vec<Vec<(usize, usize)>> = include_str!("../input.txt")
        .split("\n\n")
        .map(|machine| {
            machine
                .lines()
                .filter_map(|line| {
                    line.split_once(":").and_then(|(_, values)| {
                        values.split_once(",").and_then(|(x_split, y_split)| {
                            let x = x_split.get(3..)?.parse::<usize>().ok()?;
                            let y = y_split.get(3..)?.parse::<usize>().ok()?;
                            Some((x, y))
                        })
                    })
                })
                .collect()
        })
        .collect();

    let mut memo = AHashMap::with_capacity(110000);

    println!(
        "{:#?}",
        machines.iter().fold(0, |mut acc, machine| {
            memo.clear();
            let tokens = backtrack(&machine[0], &machine[1], &machine[2], &mut memo);
            acc += if tokens == usize::MAX { 0 } else { tokens };
            acc
        })
    );
}
