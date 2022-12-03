use std::io::{BufRead, BufReader, ErrorKind};
use std::io::Error;
use std::fs::File;
use std::collections::HashSet;

fn read_input(filename: &str) -> Result<Vec<String>, Error> {
    let f = File::open(filename).unwrap();
    let f = BufReader::new(f);
    f.lines().map(|l| l.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e)))).collect()
}

fn parse_input(input: Vec<String>) -> Vec<Vec<char>> {
    input.iter().map(|rucksack| rucksack.chars().collect()).collect()
}

fn get_priority(c: char) -> u32 {
    match c.is_lowercase() {
        true => c as u32 - 96,
        false => c as u32 -65 + 27
    }
}

fn get_hashed_set(round: &[char]) -> HashSet<char> {
    round.iter().cloned().collect()
}

fn get_priorities(rounds: &Vec<Vec<char>>) -> u32 {
    rounds.iter().map(|round| {
        let length = round.len();
        let (left, right) = round.split_at(length/2);
        let (first_batch, second_batch) = (get_hashed_set(left), get_hashed_set(right));
        get_priority(*first_batch.intersection(&second_batch).next().unwrap())
    }).sum()
}

fn get_badge_priorities(rounds: &Vec<Vec<char>>) -> u32 {
    rounds.chunks(3)
        .map(|group| {
            let mut sets: Vec<HashSet<char>> = group.iter().map(|val| get_hashed_set(val)).collect();
            let (intersection, others) = sets.split_at_mut(1);
            let intersection = &mut intersection[0];
            for other in others {
                intersection.retain(|e| other.contains(e));
            }
            get_priority(*intersection.iter().next().unwrap())
        }).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = parse_input(read_input("in.test").unwrap());
        let part1 = get_priorities(&input);
        let part2 = get_badge_priorities(&input);
        println!("test 1: {}", part1);
        println!("test 2: {}", part2);
    }

    #[test]
    fn actual() {
        use std::time::Instant;
        let now = Instant::now();
        let input = parse_input(read_input("in.1").unwrap());
        let part1 = get_priorities(&input);
        let part2 = get_badge_priorities(&input);
        let elapsed = now.elapsed();
        println!("Part 1: {}", part1);
        println!("test 2: {}", part2);
        println!("Elapsed: {:.2?}", elapsed);
    }
}
