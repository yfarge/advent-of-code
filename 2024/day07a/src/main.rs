fn is_valid(equation: &(usize, Vec<usize>)) -> bool {
    let (target, operands) = equation;
    fn backtrack(target: usize, operands: &[usize], i: usize, total: usize) -> bool {
        if total > target {
            return false;
        }

        if i + 1 == operands.len() {
            return total == target;
        }

        backtrack(target, operands, i + 1, total + operands[i + 1])
            || backtrack(target, operands, i + 1, total * operands[i + 1])
    }

    backtrack(*target, operands, 0, operands[0])
}

fn main() {
    println!(
        "{:#?}",
        include_bytes!("../input.txt")
            .split(|&b| b == b'\n')
            .filter_map(|line| {
                let mut split = line.split(|&b| b == b':');
                let target = atoi::atoi::<usize>(split.next().unwrap())?;
                let operands = split
                    .next()?
                    .split(|&b| b == b' ')
                    .filter_map(|operand| atoi::atoi::<usize>(operand))
                    .collect::<Vec<_>>();
                Some((target, operands))
            })
            .filter(is_valid)
            .map(|equation| equation.0)
            .sum::<usize>()
    );
}
