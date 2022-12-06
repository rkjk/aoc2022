use std::io::{BufRead, BufReader, ErrorKind};
use std::io::Error;
use std::fs::File;
use std::collections::HashSet;

fn read_input(filename: &str) -> Result<Vec<String>, Error> {
    let f = File::open(filename).unwrap();
    let f = BufReader::new(f);
    f.lines().map(|l| l.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e)))).collect()
}

fn get_first_marker_index(inp: &Vec<String>, num_distinct: usize) -> Vec<usize> {
    let mut res = vec![];
    for ex in inp.iter() {
        for (i, val) in ex.as_bytes().windows(num_distinct).enumerate() {
            let mut bitmap = vec![false; 26];
            for c in val.iter() {
                bitmap[(c - 97u8) as usize] = true;
            }
            let count = bitmap.iter().filter(|v| **v).count();
            if count == num_distinct {
                res.push(i + num_distinct);
                break;
            }
            //let set: HashSet<u8> = val.iter().cloned().collect();
            //if set.len() == num_distinct {
            //    res.push(i + num_distinct);
            //    break;
            //}
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let inp1 = read_input("in.test").unwrap();
        let inp2 = read_input("in.test.2").unwrap();
        let part1 = get_first_marker_index(&inp1, 4);
        let part2 = get_first_marker_index(&inp2, 14);
        println!("Test 1: {:?}", part1);
        println!("Test 2: {:?}", part2);
    }

    #[test]
    fn actual() {
        use std::time::Instant;
        let now = Instant::now();
        let inp = read_input("in.1").unwrap();
        let part1 = get_first_marker_index(&inp, 4);
        let part2 = get_first_marker_index(&inp, 14);
        let elapsed = now.elapsed();
        println!("Part 1: {:?}", part1);
        println!("Part 2: {:?}", part2);
        println!("Elapsed: {:.2?}", elapsed);
    }
}
