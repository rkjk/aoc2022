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

/// x-coordinate and distance travelled along y-coordinate
/// from initial position of rock to final positon
/// Measured using the pivot (whatever point is stored in the structure)
type FinalPos = (usize, usize);

/// Map of rock to set of FinalPos for each 
/// Will be used to check for cycles
type History = HashMap<usize, Vec<FinalPos>>;

fn parse_input(inp: Vec<String>) -> Vec<Jet> {
    inp[0].chars().map(|c| match c {
        '<' => Jet::Left,
        '>' => Jet::Right,
        _ => panic!("Unknown char")
    }).collect()
}


/// Get the next piece of rock to fall -> given the last row containing rock
/// Note -> the row will be that of the highest point of any piece in the pile
pub fn spawn(index: usize, cur_high_point: Option<usize>) -> Box<dyn Rock> {
    let high_point = match cur_high_point {
        None => 3,
        Some(v) => v + 4
    };
    match index % 5 {
        0 => Box::new(Flat::new((2, high_point))),
        1 => Box::new(Plus::new((3, high_point + 1))),
        2 => Box::new(InvL::new((2, high_point))),
        3 => Box::new(Straight::new((2, high_point))),
        4 => Box::new(Square::new((2, high_point))),
        _ => panic!("Not possible"),
    }
}

struct Session<I: Iterator<Item = Jet>> {
    jet_stream: I,
    index: usize,
    history: History,
    max_height_history: Vec<usize>,
    rocks: HashSet<Point>,
    max_height: Option<usize>
}

impl<I: Iterator<Item = Jet>> Session<I> {
    pub fn new(stream: I) -> Self {
        Session {
            index: 0,
            rocks: HashSet::new(),
            max_height: None,
            jet_stream: stream,
            history: HashMap::new(),
            max_height_history: Vec::new()
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
                    if p.0 == 6 {
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
        let init_pivot = rock.get_pivot();
        //println!("Index: {}, Spawned rock: {:?}", self.index, rock);
        loop {
            let jet = self.jet_stream.next().unwrap();
            if self.corner_check(&jet, &rock) {
                match jet {
                    Jet::Left => {
                        rock.move_left();
                        //println!("Moved Left: {:?}", rock);
                    },
                    Jet::Right => {
                        rock.move_right();
                        //println!("Moved right: {:?}", rock);
                    },
                };
            }
            match self.bottom_check(&rock) {
                true => {
                    rock.move_down();
                    //println!("Moved down: {:?}", rock);
                },
                false => break,
            };
        }
        let cur_pivot = rock.get_pivot();
        let v = rock.get_highest_point().1;
        if self.max_height.is_none() {
            self.max_height = Some(v);
        } else {
            let tmp = self.max_height.unwrap();
            if v > tmp {
                self.max_height = Some(v);
            }
        }
        for p in rock.get_all_points() {
            self.rocks.insert(p);
        }
        self.history.entry(self.index % 5)
        .and_modify(|val1| {
            let ins_val = (cur_pivot.0, init_pivot.1 - cur_pivot.1);
            let pos = val1.iter().position(|&q| q == ins_val);
            if let Some(x) = pos {
                if self.index % 5 == 3 {
                    //println!("Cur Index: {}, Act Index: {} Final Pos: {:?}, Height: {}",self.index, x, ins_val, self.max_height.unwrap());
                }
            }
            val1.push(ins_val);
        })
        .or_insert(Vec::new());
        self.max_height_history.push(self.max_height.unwrap());
        //println!("Index: {} Final Rock Pos: {:?}", self.index, rock);
        self.index += 1;
    }

    pub fn run_simulation(&mut self, upto: usize) -> usize {
        while self.index < upto {
            self.iteration();
        }
        
        for k in 1760..1761 {
            let mut flag = true;
            let mut height_diff = 0;
            for j in 1..self.max_height_history.len() - 2 * k {
                if self.max_height_history[j + k] - self.max_height_history[j] == self.max_height_history[j + 2 * k] - self.max_height_history[j + k] {
                    height_diff = self.max_height_history[j + k] - self.max_height_history[j];
                    let hd_part = self.max_height_history[j + 1476] - self.max_height_history[j];
                    let hd_part2 = self.max_height_history[j + 1738] - self.max_height_history[j];
                    let hd_part3 = self.max_height_history[j + 34] - self.max_height_history[j];
                    let hd_part4 = self.max_height_history[j + 35] - self.max_height_history[j];
                    let hd_part5 = self.max_height_history[j + 36] - self.max_height_history[j];
                    println!("j: {}, height_diff: {}", j, height_diff);
                    println!("hd_part: {}, hd_part2: {}", hd_part, hd_part2);
                    println!("hd_part3: {}, hd_part4: {}, hd_part5: {}", hd_part3, hd_part4, hd_part5);
                }

            }
        }

        self.max_height.unwrap() + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Period is 35.
    /// Height addition every period is 53
    /// Therefore -> 2022 % 35 = 27 and 2022 // 35 = 57 -> smulation(27) + 57 * 53 = 3068
    /// For 10^12 -> 10^12 % 35 = 15 and 10^12 // 35 = 28571428571 -> simulation(15) + 28571428571 * 53 = 1514285714288
    //#[test]
    fn it_works() {
        let inp = parse_input(read_input("in.test").unwrap());
        println!("Length of jet stream: {}", inp.len());
        let mut session = Session::new(inp.into_iter().cycle());
        let part1 = session.run_simulation(15);
        println!("Test 1: {}", part1);
    }

    /// Period is 1760 starting at 285
    /// Height addition every period is 2737
    /// Height addition in first 284 is 465
    /// Therefore -> 465 + (10**12 % 1760) * 2737 + Height addition in first 35/36 (which is the remainder at the end) = 1555113636385
    #[test]
    fn actual() {
        let inp = parse_input(read_input("in.1").unwrap());
        println!("Length of jet stream: {}", inp.len());
        let mut session = Session::new(inp.into_iter().cycle());
        let part1 = session.run_simulation(10000);
        println!("Part 1: {}", part1);
    }
}
