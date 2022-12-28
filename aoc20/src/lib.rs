use std::io::{BufRead, BufReader, ErrorKind};
use std::io::Error;
use std::fs::File;
use std::collections::HashMap;
use std::cmp::{min, max};

fn read_input(filename: &str) -> Result<Vec<String>, Error> {
    let f = File::open(filename).unwrap();
    let f = BufReader::new(f);
    f.lines().map(|l| l.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e)))).collect()
}

/// To handle duplicates, use the value and the original index in the array
type Key = (i64, usize);

fn parse_input(inp: Vec<String>) -> Vec<Key> {
    inp.into_iter().enumerate().map(|(i, v)| (v.parse::<i64>().unwrap(), i)).collect()
}

struct Session {
    arr: Vec<Key>,
    map: HashMap<Key, usize>,
    new_arr: Vec<Key>,
    zero_ind: Key,
}

impl Session {
    pub fn new(inp: Vec<Key>) -> Self {
        let mut map = HashMap::new();
        let mut zero_ind = (0, 0);
        for (i, k) in inp.iter().enumerate() {
            map.insert(*k, i);
            if k.0 == 0 {
                zero_ind = *k;
            }
        }
        Session {
            arr: inp.clone(),
            map: map,
            new_arr: inp,
            zero_ind: zero_ind
        }
    }

    /// Trying to replicate what rem_euclid gives for free
    fn do_one(&mut self, index: usize) {
        let (x, _) = self.arr[index];
        let cur_index = *self.map.get(&(x, index)).unwrap();
        //println!("Val: {}, cur_index: {}", x, cur_index);
        if x == 0 {
            return;
        }
        let sign = match x < 0 {
            true => -1,
            false => 1
        };
        let x = (x.abs() as usize % self.arr.len()) as i64 * sign ;
        if x > 0 {
            let xu = x as usize;
            if xu + cur_index < self.arr.len() - 1 {
                for i in cur_index + 1..cur_index + xu + 1 {
                    let val = self.new_arr[i];
                    self.map.insert(val, i - 1);
                    self.new_arr[i - 1] = val;
                }
                self.map.insert((x, index), xu + cur_index);
                self.new_arr[xu + cur_index] = (x, index);
            } else {
                let new_ind = (xu + cur_index + 1) % self.arr.len();
                let old_vals = Vec::from_iter(self.new_arr[new_ind..cur_index].iter().cloned());
                for i in new_ind..cur_index {
                    let val = old_vals[i - new_ind];
                    self.map.insert(val, i + 1);
                    self.new_arr[i + 1] = val;
                }
                self.map.insert((x, index), new_ind);
                self.new_arr[new_ind] = (x, index);
            }
        } else {
            let xu = x.abs() as usize;
            if cur_index > xu {
                let begin = (cur_index as i64 + x) as usize;
                let end = cur_index;
                let old_vals = Vec::from_iter(self.new_arr[begin..end].iter().cloned());
                for i in begin..end {
                    let val = old_vals[i - begin];
                    self.map.insert(val, i + 1);
                    self.new_arr[i + 1] = val;
                }
                self.map.insert((x, index), begin);
                self.new_arr[begin] = (x, index);
            } else {
                let new_ind = (((x + cur_index as i64).rem_euclid(self.arr.len() as i64)) - 1).rem_euclid(self.arr.len() as i64);
                //println!("New index: {}", new_ind);
                let new_ind = new_ind as usize;
                for i in cur_index + 1..new_ind+1 {
                    let val = self.new_arr[i];
                    self.map.insert(val, i - 1);
                    self.new_arr[i - 1] = val;
                }
                self.map.insert((x, index), new_ind);
                self.new_arr[new_ind] = (x, index);
            }
        }
    }

    /// Read this one online
    fn do_one_easy(&mut self, original_index: usize) {
        let index = self.new_arr.iter().position(|x| x.1 == original_index).unwrap();
        let x = self.new_arr[index].0;
        //println!("Val: {}, cur_index: {}", x, cur_index);
        if x == 0 {
            return;
        }
        let new_ind = index as i64 + x;
        let new_ind = new_ind.rem_euclid(self.new_arr.len() as i64 - 1);

        let tmp = self.new_arr.remove(index);
        self.new_arr.insert(new_ind as usize, tmp);
    }

    pub fn part_one(&mut self, rounds: usize) -> i64 {
        //println!("Init arr: {:?}", self.new_arr);
        for _ in 0..rounds {
            for i in 0..self.arr.len() {
                self.do_one_easy(i);
                //println!("new_arr: {:?}", self.new_arr);
            }
        }   
        let zero_ind = self.new_arr.iter().position(|x| x.0 == 0).unwrap();
        let thou = (zero_ind + 1000) % self.arr.len();
        let thou2 = (zero_ind + 2000) % self.arr.len();
        let thou3 = (zero_ind + 3000) % self.arr.len();
        //println!("1: {:?}, 2: {:?}, 3: {:?}", self.new_arr[thou], self.new_arr[thou2], self.new_arr[thou3]);
        self.new_arr[thou].0 + self.new_arr[thou2].0 + self.new_arr[thou3].0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let inp = parse_input(read_input("in.test").unwrap());
        let inp_2 = inp.iter().map(|&(k1, k2)| (k1 * 811589153, k2)).collect();
        let mut session = Session::new(inp);
        let part1 = session.part_one(1);
        let mut session2 = Session::new(inp_2);
        let part2 = session2.part_one(10);
        println!("Test 1: {}", part1);
        println!("Test 2: {}", part2);
    }

    #[test]
    fn actual() {
        let inp = parse_input(read_input("in.1").unwrap());
        let inp_2 = inp.iter().map(|&(k1, k2)| (k1 * 811589153, k2)).collect();
        let mut session = Session::new(inp);
        let part1 = session.part_one(1);
        let mut session2 = Session::new(inp_2);
        let part2 = session2.part_one(10);
        println!("Part 1: {}", part1);
        println!("Part 2: {}", part2);
    }
}
