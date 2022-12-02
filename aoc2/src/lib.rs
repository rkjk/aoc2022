use std::io::{BufRead, BufReader, ErrorKind};
use std::io::Error;
use std::fs::File;
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Copy)]
pub enum RPS {
    Rock,
    Paper,
    Scissor,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum PlaceHolder {
    X,
    Y,
    Z
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Outcome {
    Win,
    Lose,
    Draw
}

type Round = (RPS, PlaceHolder);

impl FromStr for RPS {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(RPS::Rock),
            "B" => Ok(RPS::Paper),
            "C" => Ok(RPS::Scissor),
            _ => Err(()),
        }
    }
}

impl FromStr for PlaceHolder {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(PlaceHolder::X),
            "Y" => Ok(PlaceHolder::Y),
            "Z" => Ok(PlaceHolder::Z),
            _ => Err(())
        }
    }
}

fn read_input(filename: &str) -> Result<Vec<String>, Error> {
    let f = File::open(filename).unwrap();
    let f = BufReader::new(f);
    f.lines().map(|l| l.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e)))).collect()
}

fn parse_input(input: Vec<String>) -> Vec<Round> {
    input.into_iter().map(|v| {
        let vals: Vec<&str> = v.trim().split(" ").collect();
        (RPS::from_str(vals[0]).unwrap(), PlaceHolder::from_str(vals[1]).unwrap())
    })
    .collect()
}

fn get_points_for_piece() -> HashMap<RPS, u64> {
    [(RPS::Rock, 1), (RPS::Paper, 2), (RPS::Scissor, 3)].iter().cloned().collect()
}

fn get_points_for_outcome() -> HashMap<Outcome, u64> {
    [(Outcome::Win, 6), (Outcome::Lose, 0), (Outcome::Draw, 3)].iter().cloned().collect()
}

fn play_round(
    pA: &RPS,
    pB: &RPS,
    points_for_outcome: &HashMap<Outcome, u64>,
    points_for_piece: &HashMap<RPS, u64>) -> u64 
{
    let mut score = 0;
    score += points_for_piece.get(pB).unwrap();
    let outcome = match pA == pB {
        true => Outcome::Draw,
        false => {
            let mut out;
            if *pB == RPS::Scissor && *pA == RPS::Paper
                || *pB == RPS::Rock && *pA == RPS::Scissor
                || *pB == RPS::Paper && *pA == RPS::Rock
            {
                out = Outcome::Win
            }
            else {
                out = Outcome::Lose;
            }
            out
        }
    };
    score += points_for_outcome.get(&outcome).unwrap();
    //println!("pA: {:?}, pB: {:?}, score: {}", pA, pB, score);
    score
}

fn play(input: &Vec<Round>) -> u64 {
    let options = vec![
        (RPS::Rock, RPS::Paper, RPS::Scissor),
        //(RPS::Rock, RPS::Scissor, RPS::Paper),
        //(RPS::Paper, RPS::Rock, RPS::Scissor),
        //(RPS::Paper, RPS::Scissor, RPS::Rock),
        //(RPS::Scissor, RPS::Rock, RPS::Paper),
        //(RPS::Scissor, RPS::Paper, RPS::Rock)
    ];
    let points_for_piece = get_points_for_piece();
    let points_for_outcome = get_points_for_outcome();
    let mut results: Vec<u64> = options.iter()
        .map(|option| input.iter().map(|round| {
                let pB = match round.1 {
                    PlaceHolder::X => option.0,
                    PlaceHolder::Y => option.1,
                    PlaceHolder::Z => option.2
                };
                play_round(&round.0, &pB, &points_for_outcome, &points_for_piece)
            }).sum()
        ).collect();
    *results.iter().max().unwrap()
}

fn get_piece_for_outcome(pA: &RPS, outcome: Outcome) -> &RPS {
    match outcome {
        Outcome::Draw => pA,
        Outcome::Win => {
            match pA {
                &RPS::Rock => &RPS::Paper,
                &RPS::Paper => &RPS::Scissor,
                &RPS::Scissor => &RPS::Rock
            }
        },
        Outcome::Lose => {
            match pA {
                &RPS::Rock => &RPS::Scissor,
                &RPS::Paper => &RPS::Rock,
                &RPS::Scissor => &RPS::Paper
            }
        }
    }
}

fn play_2(input: &Vec<Round>) -> u64 {
    let get_outcome = |placeholder: &PlaceHolder| -> Outcome {
        match placeholder {
            &PlaceHolder::X => Outcome::Lose,
            &PlaceHolder::Y => Outcome::Draw,
            &PlaceHolder::Z => Outcome::Win
        }
    };
    let points_for_piece = get_points_for_piece();
    let points_for_outcome = get_points_for_outcome();
    input.iter().map(|round| {
        let mut score = 0;
        let outcome = get_outcome(&round.1);
        score += points_for_outcome.get(&outcome).unwrap();
        score += points_for_piece.get(get_piece_for_outcome(&round.0, outcome)).unwrap();
        score
    }).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = parse_input(read_input("in.test").unwrap());
        println!("Test 1: {}", play(&input));
        println!("Test 2: {}", play_2(&input));
    }

    #[test]
    fn actual() {
        use std::time::Instant;
        let now = Instant::now();
        let input = parse_input(read_input("in.1").unwrap());
        let part1 = play(&input);
        let part2 = play_2(&input);
        let elapsed = now.elapsed();
        println!("Part 1: {}", part1);
        println!("Part 2: {}", part2);
        println!("Elapsed: {:.2?}", elapsed);
    }
}
