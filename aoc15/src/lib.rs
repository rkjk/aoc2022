use std::io::{BufRead, BufReader, ErrorKind};
use std::io::Error;
use std::fs::File;
use std::collections::HashSet;
use std::cmp::{min, max};

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
    min_limit: Point,
    max_limit: Point
}

impl Session {
    pub fn new(inp: Vec<Pair>, min_limit: Point, max_limit: Point) -> Self {
        Session {
            pairs: inp,
            min_limit: min_limit,
            max_limit: max_limit
        }
    }

    pub fn count_impossibles(&self, y: Point) -> usize {
        let mut set = HashSet::new();
        let mut res = 0;
        for pair in self.pairs.iter() {
            let cost = pair.get_y_dist(&y);
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

    /// Given a y-coordinate, return the range of x-coordinates on that row
    /// reachable from all sensor
    fn get_distress_helper(&self, y: &Point) -> Vec<Coord> {
        let mut intervals = vec![];
        for pair in self.pairs.iter() {
            let (sensor, dist, beacon) = (pair.get_sensor(), pair.dist(), pair.get_closest_beacon());
            let cost = pair.get_y_dist(&y);
            if cost > pair.dist() {
                continue;
            }
            let remain = dist - cost;
           intervals.push((max(self.min_limit, sensor.0 - remain as Point), min(self.max_limit, sensor.0 + remain as Point)));
        }
        let mut merge_intervals = move || -> Vec<Coord> {
            intervals.sort();
            let mut stack: Vec<Coord> = vec![];
            stack.push(intervals[0]);
            for i in 1..intervals.len() {
                let mut last = stack.pop().unwrap();
                let cur = intervals[i];
                if last.1 >= cur.0 {
                    if last.1 >= cur.1 {
                        stack.push(last);
                        continue;
                    }
                    stack.push((last.0, cur.1));
                    continue
                }
                stack.push(last);
                stack.push(cur);
            }
            stack
        };
        merge_intervals()
    }

    pub fn get_tuning_frequency(&self) -> u64 {
        let (rangel, rangeu) = (self.min_limit, self.max_limit + 1);
        let (mut xval, mut yval): (u64, u64) = (0, 0);
        for y in rangel..rangeu {
            let intervals = self.get_distress_helper(&y);
            //if intervals.len() > 1 {
            //    println!("y: {}, intervals: {:?}", y, intervals);
            //}
            if intervals.len() == 1 {
                continue;
            }
            for i in 0..intervals.len() - 1 {
                if intervals[i + 1].0 - intervals[i].1 == 2 {
                    xval = (intervals[i].1 + 1) as u64;
                    yval = y as u64;
                }
            }
        }
        xval * 4000000 + yval
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
        let session = Session::new(inp, 0, 20);
        let part1 = session.count_impossibles(10);
        let part2 = session.get_tuning_frequency();
        println!("Test 1: {}", part1);
        println!("Test 2: {}", part2);
    }

    #[test]
    fn actual() {
        use std::time::Instant;
        let now = Instant::now();
        let inp = parse_input(read_input("in.1").unwrap());
        let session = Session::new(inp, 0, 4000000);
        let part1 = session.count_impossibles(2000000);
        let part2 = session.get_tuning_frequency();
        let elapsed = now.elapsed();
        println!("part 1: {}", part1);
        println!("Part 2: {}", part2);
        println!("Elapsed: {:?}", elapsed);
    }
}
