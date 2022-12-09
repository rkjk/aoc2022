use std::io::{BufRead, BufReader, ErrorKind};
use std::io::Error;
use std::fs::File;
use std::collections::HashSet;

fn read_input(filename: &str) -> Result<Vec<String>, Error> {
    let f = File::open(filename).unwrap();
    let f = BufReader::new(f);
    f.lines().map(|l| l.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e)))).collect()
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    R,
    L,
    U,
    D
}

type Instr = (Direction, i32);

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct Pos {
    pub i: i32,
    pub j: i32
}

impl Pos {
    pub fn new_origin() -> Self {
        Pos {
            i: 0,
            j: 0
        }
    }

    pub fn new(i: i32, j: i32) -> Self {
        Pos {
            i: i,
            j: j
        }
    }
}

fn parse_input(inp: Vec<String>) -> Vec<Instr> {
    inp.into_iter().map(|line| {
        let l: Vec<&str> = line.split(" ").collect();
        let dir = match l[0] {
            "R" => Direction::R,
            "L" => Direction::L,
            "U" => Direction::U,
            "D" => Direction::D,
            _ => panic!("Unexpected")
        };
        (dir, l[1].parse::<i32>().unwrap())
    }).collect()
}

struct Session {
    h: Pos,
    t: Pos,
    seen_pos: HashSet<Pos>
}

impl Session {
    pub fn new() -> Self {
        Session {
            h: Pos::new_origin(),
            t: Pos::new_origin(),
            seen_pos: HashSet::new()
        }
    }

    pub fn run_through(&mut self, instructions: &Vec<Instr>) {
        self.seen_pos.insert(self.h);
        let mut c = 0;
        for instr in instructions {
            c += 1;
            //if c == 20 {
            //    break;
            //}
            let (dir, magn) = (instr.0, instr.1);
            let (h, t) = (self.h, self.t);
            //println!("iter: {} h: {:?} t: {:?}", c, h, t,);
            //println!("dir: {:?}, magn: {:?}", dir, magn);
            match dir {
                Direction::R => {
                    let h_n = Pos::new(h.i + magn, h.j);
                    if h.i == t.i && magn == 1 || h.i == t.i - 1 && (magn == 1 || magn == 2) {
                        self.h = h_n;
                        continue;
                    }
                    let t_n = Pos::new(h.i + magn - 1, h.j);
                    let start_idx = t.i + 1;
                    let end_idx = h.i + magn - 1;
                    for r in start_idx..end_idx+1 {
                        self.seen_pos.insert(Pos::new(r, h.j));
                    }
                    self.h = h_n;
                    self.t = t_n;
                },
                Direction::L => {
                    let h_n = Pos::new(h.i - magn, h.j);
                    if h.i == t.i && magn == 1 || h.i == t.i + 1 && (magn == 1 || magn == 2) {
                        self.h = h_n;
                        continue;
                    }
                    let t_n = Pos::new(h.i - magn + 1, h.j);
                    let end_idx = t.i - 1;
                    let start_idx = h.i - magn + 1;
                    for r in start_idx..end_idx+1 {
                        self.seen_pos.insert(Pos::new(r, h.j));
                    }
                    self.h = h_n;
                    self.t = t_n;
                },
                Direction::U => {
                    let h_n = Pos::new(h.i, h.j + magn);
                    if h.j == t.j - 1 && (magn == 1 || magn == 2) || h.j == t.j && magn == 1 {
                        self.h = h_n;
                        continue;
                    }
                    let t_n = Pos::new(h.i, h.j + magn - 1);
                    let start_idx = t.j + 1;
                    let end_idx = h.j + magn - 1;
                    for r in start_idx..end_idx+1 {
                        self.seen_pos.insert(Pos::new(h.i, r));
                    }
                    self.h = h_n;
                    self.t = t_n;
                },
                Direction::D => {
                    let h_n = Pos::new(h.i, h.j - magn);
                    if h.j == t.j + 1 && (magn == 1 || magn == 2) || h.j == t.j && magn == 1 {
                        self.h = h_n;
                        continue;
                    }
                    let t_n = Pos::new(h.i, h.j - magn + 1);
                    let end_idx = t.j - 1;
                    let start_idx = h.j - magn + 1;
                    for r in start_idx..end_idx+1 {
                        self.seen_pos.insert(Pos::new(h.i, r));
                    }
                    self.h = h_n;
                    self.t = t_n;
                },
            };
        }
    }

    pub fn seen_count(&self) -> usize {
        self.seen_pos.len()
    }

    pub fn print(&self) {
        println!("Current position: {:?} {:?}", self.h, self.t);
        println!("Seen: {:?}", self.seen_pos);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let inp = parse_input(read_input("in.test").unwrap());
        let mut session = Session::new();
        session.run_through(&inp);
        let part1 = session.seen_count();
        //session.print();
        println!("Test 1: {}", part1);
    }

    #[test]
    fn actual() {
        let inp = parse_input(read_input("in.1").unwrap());
        let mut session = Session::new();
        session.run_through(&inp);
        let part1 = session.seen_count();
        //session.print();
        println!("Actual 1: {}", part1);
    }
}
