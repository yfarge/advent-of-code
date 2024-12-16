use ahash::AHashMap;

fn split(a: usize) -> (usize, usize) {
    let offset = 10usize.pow((a.ilog10() + 1) / 2);
    let l = a / offset;
    let r = a - l * offset;
    (l, r)
}

fn blink(v: usize, n: usize, h: &mut AHashMap<(usize, usize), usize>) -> usize {
    if n == 0 {
        return 0;
    }

    if h.contains_key(&(v, n)) {
        return h[&(v, n)];
    }

    let count;
    if v == 0 {
        count = blink(1, n - 1, h);
    } else if v.ilog10() % 2 != 0 {
        let (l, r) = split(v);
        count = 1 + blink(l, n - 1, h) + blink(r, n - 1, h);
    } else {
        count = blink(v * 2024, n - 1, h);
    }

    h.insert((v, n), count);
    return count;
}

fn main() {
    let mut memo = AHashMap::with_capacity(150000);

    println!(
        "{}",
        include_bytes!("../input.txt")
            .split(|&b| b == b' ')
            .filter(|line| !line.is_empty())
            .map(|b| 1 + blink(atoi::atoi::<usize>(b).unwrap(), 25, &mut memo))
            .sum::<usize>()
    );
}
