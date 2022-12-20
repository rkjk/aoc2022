use std::io::{BufRead, BufReader, ErrorKind};
use std::io::Error;
use std::fs::File;
use std::collections::HashSet;

fn read_input(filename: &str) -> Result<Vec<String>, Error> {
    let f = File::open(filename).unwrap();
    let f = BufReader::new(f);
    f.lines().map(|l| l.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e)))).collect()
}

type Point = i32;
type Dist = u32;
type Coord = (Point, Point);

#[derive(Debug, Copy, Clone)]
struct Pair {
    sensor: Coord,
    closest_beacon: Coord,
    manhattan_dist: Dist
}

impl Pair {
    pub fn new(sen: Coord, beac: Coord) -> Self {
        Pair {
            sensor: sen,
            closest_beacon: beac,
            manhattan_dist: Pair::manhattan_dist(&sen, &beac)
        }
    }

    fn manhattan_dist(sen: &Coord, beac: &Coord) -> Dist {
        ((sen.0 - beac.0).abs() + (sen.1 - beac.1).abs()) as Dist
    }

    pub fn dist(&self) -> Dist {
        self.manhattan_dist
    }

    pub fn get_y_dist(&self, y: &Point) -> Dist {
        (self.sensor.1 - y).abs() as Dist
    }

    pub fn get_sensor(&self) -> &Coord {
        &self.sensor
    }

    pub fn get_closest_beacon(&self) -> &Coord {
        &self.closest_beacon
    }
}

struct Session {
    pairs: Vec<Pair>,
}

impl Session {
    pub fn new(inp: Vec<Pair>) -> Self {
        Session {
            pairs: inp,
        }
    }

    pub fn count_impossibles(&self, y: Point) -> usize {
        let mut set = HashSet::new();
        let mut res = 0;
        for pair in self.pairs.iter() {
            let cost = pair.get_y_dist(&y);
            // The row is too far away
            let (sensor, dist, beacon) = (pair.get_sensor(), pair.dist(), pair.get_closest_beacon());
            //println!("sensor: {:?}, man_d: {}, cost: {}", sensor, pair.dist(), cost);
            if cost > pair.dist() {
                continue;
            }
            let remain = dist - cost;
            for x in (sensor.0 - remain as Point)..(sensor.0 + remain as Point + 1) {
                set.insert((x, y));
            }
            //let mut add = (2 * remain + 1) as usize;
            if pair.get_closest_beacon().1  == y {
                set.remove(beacon);
            }
            //println!("Added: {}", add);
        }
        set.len()
    }
}

/// Return vector of sensor-closest-beacon pairs
fn parse_input(inp: Vec<String>) -> Vec<Pair> {
    let parse = |s: &str, remove_last: bool| -> i32 {
        let new_s = match remove_last {
            false => s,
            true => {
                let mut chars = s.chars();
                chars.next_back();
                chars.as_str()
            }
        };
        let arr: Vec<&str> = new_s.split("=").collect();
        arr[arr.len() - 1].parse::<i32>().unwrap()
    };
    inp.into_iter().map(|line| {
        let tmp: Vec<&str> = line.split_whitespace().collect();
        let sensor = (parse(tmp[2], true), parse(tmp[3], true));
        let beac = (parse(tmp[8], true), parse(tmp[9], false));
        Pair::new(sensor, beac)
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let inp = parse_input(read_input("in.test").unwrap());
        let session = Session::new(inp);
        let part1 = session.count_impossibles(10);
        println!("Test 1: {}", part1);
    }

    #[test]
    fn actual() {
        let inp = parse_input(read_input("in.1").unwrap());
        let session = Session::new(inp);
        let part1 = session.count_impossibles(2000000);
        println!("part 1: {}", part1);
    }
}
