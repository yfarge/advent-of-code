fn main() {
    let bytes = include_bytes!("../input.txt");
    let width = bytes.iter().position(|x| *x == b'\n').unwrap() as isize;

    println!(
        "{:#?}",
        // step 1: get the starting index of each engine part
        (0..bytes.len() - 2)
            .filter(|i| {
                bytes[*i].is_ascii_digit()
                    && !bytes
                        .get(i.wrapping_sub(1))
                        .map_or(false, u8::is_ascii_digit)
            })
            // step 2: get the engine number and it's length
            .map(|start| {
                let len = (start + 1..start + 4)
                    .position(|next| !bytes[next].is_ascii_digit())
                    .unwrap()
                    + 1;

                let engine_part: usize = atoi::atoi(&bytes[start..start + len]).unwrap();

                return (start, len as _, engine_part);
            })
            // step 3: filter out engine parts not touching a symbol
            .filter(|(start, len, _engine_part)| {
                (-width - 2..-width + *len)
                    .chain([-1, *len])
                    .chain(width..width + *len + 2)
                    .any(|offset| {
                        bytes
                            .get((*start as isize + offset) as usize)
                            .map_or(false, |b| !(*b == b'.') && b.is_ascii_punctuation())
                    })
            })
            // step 4: create a map of engine parts
            .map(|(_start, _len, engine_part)| engine_part)
            // step 5: sum engine parts
            .sum::<usize>()
    );
}
