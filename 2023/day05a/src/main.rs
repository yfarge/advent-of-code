fn source_to_destination(source: usize, mapping: &Vec<(usize, usize, usize)>) -> usize {
    for (d, s, l) in mapping {
        if source.ge(&s) && source.lt(&(s + l)) {
            return source - s + d;
        }
    }

    return source;
}

fn seed_to_location(seed: usize, mappings: &Vec<Vec<(usize, usize, usize)>>) -> usize {
    let mut result = seed;
    for map in mappings {
        result = source_to_destination(result, map);
    }
    return result;
}

fn main() {
    let mut lines = include_str!("../input.txt").split("\n\n");
    let seeds: Vec<usize> = lines
        .next()
        .unwrap()
        .split(":")
        .last()
        .unwrap()
        .split_whitespace()
        .map(|num| num.parse().unwrap())
        .collect();

    let maps = lines
        .map(|map| {
            map.split(":")
                .last()
                .unwrap()
                .split("\n")
                .filter(|x| !x.is_empty())
                .map(|range| {
                    let mut r = range
                        .split_whitespace()
                        .filter_map(|num| num.parse::<usize>().ok());
                    (r.next().unwrap(), r.next().unwrap(), r.next().unwrap())
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    println!(
        "{:#?}",
        seeds
            .iter()
            .map(|seed| seed_to_location(*seed, &maps))
            .min()
            .unwrap()
    )
}
