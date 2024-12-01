fn main() {
    let mut list_one: Vec<isize> = Vec::new();
    let mut list_two: Vec<isize> = Vec::new();

    include_str!("../input.txt").lines().for_each(|line| {
        let mut location_ids = line.split_whitespace();
        list_one.push(location_ids.next().unwrap().parse::<isize>().unwrap());
        list_two.push(location_ids.next().unwrap().parse::<isize>().unwrap());
    });

    list_one.sort();
    list_two.sort();

    println!(
        "{:#?}",
        list_one
            .iter()
            .zip(list_two.iter())
            .map(|(&a, &b)| (a - b).abs())
            .sum::<isize>()
    )
}
