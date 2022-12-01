use std::io::{BufRead, BufReader, ErrorKind};
use std::io::Error;
use std::fs::File;
use std::collections::BinaryHeap;

fn read_input(filename: &str) -> Result<Vec<String>, Error> {
    let f = File::open(filename).unwrap();
    let f = BufReader::new(f);
    f.lines().map(|l| l.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e)))).collect()
}

fn parse_input(input: Vec<String>) -> Vec<Vec<u64>> {
    let mut cur_vec: Option<Vec<u64>> = None;
    let mut ret_vec: Vec<Vec<u64>> = vec![];
    for (i, v) in input.into_iter().enumerate() {
        match v.is_empty() {
            false => cur_vec.get_or_insert(Vec::new()).push(v.parse::<u64>().unwrap()),
            true => ret_vec.push(cur_vec.take().unwrap())
        }
    }
    ret_vec.push(cur_vec.take().unwrap());
    ret_vec
}

fn get_total(input: &Vec<Vec<u64>>) -> Vec<u64> {
    input.iter().map(|v| v.iter().sum()).collect()
}

fn get_max_3(input: &Vec<u64>) -> u64 {
    let mut heap: BinaryHeap<i64> = BinaryHeap::new();
    for val in input.iter() {
        let nval = *val as i64;
        if heap.len() < 3 {
            heap.push(-1 * nval);
        }
        else {
            if -1 * nval < *heap.peek().unwrap() {
                heap.pop();
                heap.push(-1 * nval);
            }
        }
    }
    let mut sum = 0;
    while !heap.is_empty() {
        sum += (heap.pop().unwrap() * -1) as u64;
    }
    sum
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        use super::*;
        let input: Vec<Vec<u64>> = parse_input(read_input("in.test").unwrap());
        let sums = get_total(&input);
        println!("test: {}", sums.iter().max().unwrap());
        println!("test 2: {}", get_max_3(&sums));
    }

    #[test]
    fn part() {
        use super::*;
        use std::time::Instant;
        let now = Instant::now();
        let input: Vec<Vec<u64>> = parse_input(read_input("in.1").unwrap());
        let sums = get_total(&input);
        let part1_res = sums.iter().max().unwrap();
        let part2_res = get_max_3(&sums);
        let elapsed = now.elapsed();
        println!("part 1: {}", part1_res);
        println!("test 2: {}", part2_res);
        println!("Elapsed: {:.2?}", elapsed);
    }
}
