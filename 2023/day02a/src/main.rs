use std::u32;

// red, green, blue
#[derive(Debug)]
struct Round(u32, u32, u32);

#[derive(Debug)]
struct Game {
    id: u32,
    rounds: Vec<Round>,
}

impl Game {
    fn get_max_red(&self) -> u32 {
        match self.rounds.iter().map(|round| round.0).max() {
            Some(v) => v,
            None => 0,
        }
    }

    fn get_max_green(&self) -> u32 {
        match self.rounds.iter().map(|round| round.1).max() {
            Some(v) => v,
            None => 0,
        }
    }

    fn get_max_blue(&self) -> u32 {
        match self.rounds.iter().map(|round| round.2).max() {
            Some(v) => v,
            None => 0,
        }
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
    let id = game_rounds[0].split(" ").collect::<Vec<_>>()[1]
        .parse()
        .unwrap();
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
        id,
        rounds: build_rounds(round_data),
    };
}

fn main() {
    println!(
        "{:#?}",
        include_str!("../input.txt")
            .lines()
            .map(|line| process_line(line))
            .filter(|game| game.get_max_red() <= 12
                && game.get_max_green() <= 13
                && game.get_max_blue() <= 14)
            .map(|game| game.id)
            .sum::<u32>()
    );
}
