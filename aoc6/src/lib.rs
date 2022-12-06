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
    let get_ones = |val: &[u8]| -> usize {
        let mut accum: usize = 0;
        for c in val.iter() {
            accum ^= (1 << (*c - 97u8)) as usize;
        }
        accum.count_ones() as usize
    };

    for ex in inp.iter() {
        for (i, val) in ex.as_bytes().windows(num_distinct).enumerate() {
            // Slowest -> Hash the slice and compare lengths - 1ms in release
            //let set: HashSet<u8> = val.iter().cloned().collect();
            //if set.len() == num_distinct {
            //    res.push(i + num_distinct);
            //    break;
            //}

            // Use Bitmap -> compare number of true values with num_distinct -> 500us in release
            //let mut bitmap = vec![false; 26];
            //let count = bitmap.iter().filter(|v| **v).count();
            //if count == num_distinct {
            //    res.push(i + num_distinct);
            //    break;
            //}

            // Idea from /r/adventofcode -> Use XoR -> 100us in release
            if get_ones(&val) == num_distinct {
                res.push(i + num_distinct);
                break;
            }

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
