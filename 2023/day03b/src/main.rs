fn main() {
    let bytes = include_bytes!("../input.txt");
    let width = bytes.iter().position(|b| *b == b'\n').unwrap() as isize;
    let mut part_indices: Vec<usize> = vec![];

    println!(
        "{:#?}",
        (0..bytes.len() - 2)
            // find the indices of all the potential gear indices (gi) *
            .filter(|index| { bytes[*index] == b'*' })
            .filter_map(|i| {
                // clear the previous iteration's indices
                part_indices.clear();
                // find all the starting index of all the associated engine parts
                part_indices.extend(
                    (-width - 2..=-width)
                        .chain([-1, 1])
                        .chain(width..=width + 2)
                        .map(|pos| (i as isize + pos) as usize)
                        .filter(|pos| bytes[*pos].is_ascii_digit())
                        .flat_map(|pos| {
                            (pos.saturating_sub(2)..=pos)
                                .rev()
                                .take_while(|i| bytes[*i].is_ascii_digit())
                                .last()
                        }),
                );
                // remove duplicate entries
                part_indices.dedup();
                // calculate the gear ratio
                (part_indices.len() == 2).then(|| {
                    part_indices
                        .iter()
                        .map(|start| {
                            let len = (start + 1..start + 4)
                                .position(|next| !bytes[next].is_ascii_digit())
                                .unwrap()
                                + 1;
                            atoi::atoi::<usize>(&bytes[*start..*start + len]).unwrap()
                        })
                        .product::<usize>()
                })
            })
            .sum::<usize>()
    )
}
