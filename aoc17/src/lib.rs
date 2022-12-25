use std::io::{BufRead, BufReader, ErrorKind};
use std::io::Error;
use std::fs::File;
use std::collections::{HashMap, HashSet};
use std::cmp::{min, max};
use std::iter::Cycle;

use crate::rock::{Rock, Point};

pub mod rock;
pub mod flat;
pub mod straight;
pub mod square;
pub mod invl;
pub mod plus;

use flat::Flat;
use straight::Straight;
use square::Square;
use invl::InvL;
use plus::Plus;

fn read_input(filename: &str) -> Result<Vec<String>, Error> {
    let f = File::open(filename).unwrap();
    let f = BufReader::new(f);
    f.lines().map(|l| l.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e)))).collect()
}

#[derive(Debug, Clone)]
enum Jet {
    Left,
    Right
}

fn parse_input(inp: Vec<String>) -> Vec<Jet> {
    inp[0].chars().map(|c| match c {
        '<' => Jet::Left,
        '>' => Jet::Right,
        _ => panic!("Unknown char")
    }).collect()
}


/// Get the next piece of rock to fall -> given the last row containing rock
/// Note -> the row will be that of the highest point of any piece in the pile
pub fn spawn(index: usize, cur_high_point: usize) -> Box<dyn Rock> {
    match index % 5 {
        0 => Box::new(Flat::new((cur_high_point + 3, 2))),
        1 => Box::new(Plus::new((cur_high_point + 4, 3))),
        2 => Box::new(InvL::new((cur_high_point + 3, 2))),
        3 => Box::new(Straight::new((cur_high_point + 3, 2))),
        4 => Box::new(Square::new((cur_high_point + 3, 2))),
        _ => panic!("Not possible"),
    }
}

struct Session<I: Iterator<Item = Jet>> {
    jet_stream: I,
    index: usize,
    rocks: HashSet<Point>,
    max_height: usize
}

impl<I: Iterator<Item = Jet>> Session<I> {
    pub fn new(stream: I) -> Self {
        Session {
            index: 0,
            rocks: HashSet::new(),
            max_height: 0,
            jet_stream: stream
        }
    }

    fn corner_check(&self, jet: &Jet, rock: &Box<dyn Rock>) -> bool {
        match jet {
            &Jet::Left => {
                let le = rock.get_left_endpoints();
                for p in &le {
                    if p.0 == 0 {
                        return false;
                    }
                }
                for p in &le {
                    if self.rocks.contains(&(p.0 - 1, p.1)) {
                        return false;
                    }
                }
            },
            &Jet::Right => {
                let re = rock.get_right_endpoints();
                for p in &re {
                    if p.0 == 7 {
                        return false;
                    }
                }
                for p in &re {
                    if self.rocks.contains(&(p.0 + 1, p.1)) {
                        return false;
                    }
                }
            }
        };
        true
    }

    fn bottom_check(&self, rock: &Box<dyn Rock>) -> bool {
        let be = rock.get_bottom_endpoints();
        for p in &be {
            if p.1 == 0 {
                return false;
            }
        }
        for p in &be {
            if self.rocks.contains(&(p.0, p.1 - 1)) {
                return false;
            }
        }
        true
    }

    fn iteration(&mut self) {
        let mut rock = spawn(self.index, self.max_height);
        loop {
            let jet = self.jet_stream.next().unwrap();
            if self.corner_check(&jet, &rock) {
                match jet {
                    Jet::Left => rock.move_left(),
                    Jet::Right => rock.move_right(),
                };
            }
            match self.bottom_check(&rock) {
                true => rock.move_down(),
                false => break,
            };
        }
        self.max_height = max(self.max_height, rock.get_highest_point().1);
        for p in rock.get_all_points() {
            self.rocks.insert(p);
        }
    }

    pub fn run_simulation(&mut self, upto: usize) -> usize {
        while self.index <= upto {
            self.iteration();
        }
        self.max_height
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let inp = parse_input(read_input("in.test").unwrap());
        let mut session = Session::new(inp.into_iter().cycle());
    }
}
