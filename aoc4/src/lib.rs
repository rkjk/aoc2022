use std::io::{BufRead, BufReader, ErrorKind};
use std::io::Error;
use std::fs::File;
use std::cmp::{min, max};

fn read_input(filename: &str) -> Result<Vec<String>, Error> {
    let f = File::open(filename).unwrap();
    let f = BufReader::new(f);
    f.lines().map(|l| l.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e)))).collect()
}

type Group = Vec<(u32, u32)>;

fn parse_input(input: Vec<String>) -> Vec<Group> {
    input.into_iter()
        .map(|s| s.split(",")
            .map(|one| {
                let v = one.split("-").map(|v| v.parse::<u32>().unwrap()).collect::<Vec<u32>>();
                (v[0], v[1])
                })
            .collect::<Group>())
        .collect()
}

fn check_subsumed(inp: &Vec<Group>) -> usize {
    inp.iter().filter(|val| 
        val[0].0 >= val[1].0 && val[0].1 <= val[1].1 ||
        val[1].0 >= val[0].0 && val[1].1 <= val[0].1
    ).count()
}

fn check_overlap(inp: &Vec<Group>) -> usize {
    inp.iter().filter(|val| 
        min(val[0].1, val[1].1) >= max(val[0].0, val[1].0))
    .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let inp = parse_input(read_input("in.test").unwrap());
        let part1 = check_subsumed(&inp);
        let part2 = check_overlap(&inp);
        println!("test 1: {}", part1);
        println!("test 2: {}", part2);
    }

    #[test]
    fn actual() {
        use std::time::Instant;
        let now = Instant::now();
        let inp = parse_input(read_input("in.1").unwrap());
        let part1 = check_subsumed(&inp);
        let part2 = check_overlap(&inp);
        let elapsed = now.elapsed();
        println!("Part 1: {}", part1);
        println!("Part 2: {}", part2);
        println!("Elapsed: {:.2?}", elapsed);
    }
}
