use std::io::{BufRead, BufReader, ErrorKind};
use std::io::Error;
use std::fs::File;

fn read_input(filename: &str) -> Result<Vec<String>, Error> {
    let f = File::open(filename).unwrap();
    let f = BufReader::new(f);
    f.lines().map(|l| l.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e)))).collect()
}

#[derive(Debug)]
struct Instr {
    quantity: usize,
    from: usize,
    to: usize,
}

impl Instr {
    pub fn new(quantity: usize, from: usize, to: usize) -> Self {
        Instr {
            quantity: quantity,
            from: from - 1,
            to: to - 1
        }
    }
}

#[derive(Debug)]
struct Ship {
    crates: Vec<Vec<u8>>,
    instructions: Vec<Instr>
}

impl Ship {
    pub fn new(stacks: Vec<&str>, instructions: Vec<Instr>) -> Self {
        Ship {
            crates: stacks.into_iter().map(|stack| stack.as_bytes().to_vec()).collect(),
            instructions: instructions
        }
    }

    pub fn simulate_part1(&self) -> String {
        let mut crates_copy: Vec<Vec<u8>> = self.crates.iter().cloned().collect();
        for instr in self.instructions.iter() {
            for _ in 0..instr.quantity {
                let val = crates_copy[instr.from].pop().unwrap();
                crates_copy[instr.to].push(val);
            }
        }
        Ship::get_as_string(&crates_copy)
    }

    pub fn simulate_part2(&mut self) -> String {
        for instr in self.instructions.iter() {
            let ind = self.crates[instr.from].len() - instr.quantity;
            let mut tmp = self.crates[instr.from].split_off(ind);
            self.crates[instr.to].append(&mut tmp);
        }
        Ship::get_as_string(&self.crates)
    }

    fn get_as_string(crates: &Vec<Vec<u8>>) -> String {
        String::from_utf8(crates.iter().map(|v| v[v.len() - 1]).collect()).unwrap()
    }
}

fn parse_input(input: Vec<String>) -> Vec<Instr> {
    let mut res = Vec::new();
    let mut flag = false;
    for val in input.into_iter() {
        if !flag && !val.starts_with("m") {
            continue;
        }
        flag = true;
        let tmp: Vec<&str> = val.split(" ").collect();
        res.push(Instr::new(
            tmp[1].parse::<usize>().unwrap(),
            tmp[3].parse::<usize>().unwrap(),
            tmp[5].parse::<usize>().unwrap()));
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let inp = parse_input(read_input("in.test").unwrap());
        let crates = vec!["ZN", "MCD", "P"];
        let mut ship = Ship::new(crates, inp);
        let part1 = ship.simulate_part1();
        let part2 = ship.simulate_part2();
        println!("Part 1: {:?}", part1);
        println!("Part 2: {:?}", part2);
    }

    #[test]
    fn actual() {
        use std::time::Instant;
        let now = Instant::now();
        let inp = parse_input(read_input("in.1").unwrap());
        let crates = vec![
            "RSLFQ",
            "NZQGPT",
            "SMQB",
            "TGZJHCBQ",
            "PHMBNFS",
            "PCQNSLVG",
            "WCF",
            "QHGZWVPM",
            "GZDLCNR"
        ];
        let mut ship = Ship::new(crates, inp);
        let part1 = ship.simulate_part1();
        let part2 = ship.simulate_part2();
        let elapsed = now.elapsed();
        println!("Part 1: {:?}", part1);
        println!("Part 2: {:?}", part2);
        println!("Elapsed: {:.2?}", elapsed);
    }
}
