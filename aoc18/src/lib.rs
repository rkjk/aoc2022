use std::io::{BufRead, BufReader, ErrorKind};
use std::io::Error;
use std::fs::File;
use std::collections::{HashMap, HashSet};
use std::cmp::{min, max};

type Cube = (i32, i32, i32);

fn read_input(filename: &str) -> Result<Vec<String>, Error> {
    let f = File::open(filename).unwrap();
    let f = BufReader::new(f);
    f.lines().map(|l| l.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e)))).collect()
}

fn parse_input(inp: Vec<String>) -> HashSet<Cube> {
    inp.into_iter().map(|l| {
        let tmp: Vec<i32> = l.trim().split(",").map(|num| num.parse::<i32>().unwrap()).collect();
        (tmp[0], tmp[1], tmp[2])
    }).collect()
}

struct Session {
    cubes: HashSet<Cube>
}

impl Session {
    pub fn new(inp: HashSet<Cube>) -> Self {
        Session {
            cubes: inp
        }
    }

    fn get_neighbours(cube: &Cube) -> Vec<Cube> {
        let &(x, y, z) = cube;
        vec![
            (x + 1, y, z), (x - 1, y, z),
            (x, y + 1, z), (x, y - 1, z),
            (x, y, z + 1), (x, y, z - 1)
        ]
    }

    pub fn count_surfaces(&self) -> usize {
        let mut count = 0;
        for v in self.cubes.iter() {
            let mut init_count = 6;
            for n in Session::get_neighbours(v) {
                if self.cubes.contains(&n) {
                    init_count -= 1;
                }
            }
            count += init_count;
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let inp = parse_input(read_input("in.test").unwrap());
        let session = Session::new(inp);
        let part1 = session.count_surfaces();
        println!("Test 1: {}", part1);
    }

    #[test]
    fn actual() {
        let inp = parse_input(read_input("in.1").unwrap());
        let session = Session::new(inp);
        let part1 = session.count_surfaces();
        println!("Part 1: {}", part1);
    }
}
