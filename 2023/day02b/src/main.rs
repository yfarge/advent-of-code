// red, green, blue
#[derive(Debug)]
struct Round(u32, u32, u32);

#[derive(Debug)]
struct Game {
    rounds: Vec<Round>,
}

impl Game {
    fn get_max(&self) -> Vec<u32> {
        let mut max_red: u32 = 0;
        let mut max_green: u32 = 0;
        let mut max_blue: u32 = 0;
        match self.rounds.iter().map(|round| round.0).max() {
            Some(v) => max_red = v,
            None => (),
        };
        match self.rounds.iter().map(|round| round.1).max() {
            Some(v) => max_green = v,
            None => (),
        };
        match self.rounds.iter().map(|round| round.2).max() {
            Some(v) => max_blue = v,
            None => (),
        };
        return vec![max_red, max_green, max_blue];
    }
}

fn build_rounds(round_data: Vec<Vec<Vec<&str>>>) -> Vec<Round> {
    let mut red: u32;
    let mut green: u32;
    let mut blue: u32;
    let mut rounds: Vec<Round> = Vec::new();
    for round in round_data {
        for cube in round {
            red = 0;
            green = 0;
            blue = 0;
            let (amount, color) = (cube[0], cube[1]);
            match color {
                "red" => red = amount.parse().unwrap(),
                "blue" => blue = amount.parse().unwrap(),
                "green" => green = amount.parse().unwrap(),
                _ => (),
            }
            rounds.push(Round(red, green, blue))
        }
    }
    return rounds;
}

fn process_line(line: &str) -> Game {
    let game_rounds = line.split(":").collect::<Vec<_>>();
    let round_data: Vec<_> = game_rounds[1]
        .split(";")
        .map(|round| {
            round
                .trim()
                .split(",")
                .map(|cube| cube.trim().split(" ").collect::<Vec<_>>())
                .collect::<Vec<_>>()
        })
        .collect();

    return Game {
        rounds: build_rounds(round_data),
    };
}

fn main() {
    println!(
        "{:#?}",
        include_str!("../input.txt")
            .lines()
            .map(|line| process_line(line)
                .get_max()
                .iter()
                .fold(1, |acc, x| acc * x))
            .sum::<u32>()
    );
}

