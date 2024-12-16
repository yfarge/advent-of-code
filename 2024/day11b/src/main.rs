use std::{collections::HashMap, ops::Mul};

fn split(a: usize) -> (usize, usize) {
    let offset = 10usize.pow((a.ilog10() + 1) / 2);
    let l = a / offset;
    let r = a - l.mul(offset);
    (l, r)
}

fn is_even_digits(a: usize) -> bool {
    a.ilog10() % 2 != 0
}

fn blink(v: usize, n: usize) -> usize {
    let mut memo = HashMap::new();

    fn dfs(v: usize, n: usize, h: &mut HashMap<(usize, usize), usize>) -> usize {
        if n == 0 {
            return 0;
        }

        if h.contains_key(&(v, n)) {
            return h[&(v, n)];
        }

        let count;
        if v == 0 {
            count = dfs(1, n - 1, h);
        } else if is_even_digits(v) {
            let (l, r) = split(v);
            count = 1 + dfs(l, n - 1, h) + dfs(r, n - 1, h);
        } else {
            count = dfs(v.mul(2024), n - 1, h);
        }

        h.insert((v, n), count);
        return count;
    }

    dfs(v, n, &mut memo)
}

fn main() {
    println!(
        "{}",
        include_bytes!("../input.txt")
            .split(|&b| b == b' ')
            .filter(|line| !line.is_empty())
            .map(|b| atoi::atoi::<usize>(b).unwrap())
            .map(|stone| 1 + blink(stone, 75))
            .sum::<usize>()
    );
}
