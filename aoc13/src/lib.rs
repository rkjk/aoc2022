use std::io::{BufRead, BufReader, ErrorKind};
use std::io::Error;
use std::fs::File;
use std::mem::take;
use std::fmt::Debug;
use std::cmp::Ordering;

fn read_input(filename: &str) -> Result<Vec<String>, Error> {
    let f = File::open(filename).unwrap();
    let f = BufReader::new(f);
    f.lines().map(|l| l.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e)))).collect()
}

type Pair = (Node, Node);

#[derive(Debug, Default)]
struct ListNode {
    items: Vec<Box<Node>>
}

impl ListNode {
    pub fn new() -> Self {
        ListNode {
            items: vec![]
        }
    }

    pub fn single(single_item: Box<Node>) -> Self {
        ListNode {
            items: vec![single_item]
        }
    }

    pub fn push(&mut self, node: Box<Node>) {
        self.items.push(node);
    }

    pub fn get(&self, index: usize) -> &Node {
        self.items[index].as_ref()
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn compare(&self, node: &ListNode) -> Ordering {
        let (mut i, mut j) = (0, 0);
        loop {
            if i == self.len() && j == node.len() {
                break;
            }
            if i == self.len() {
                return Ordering::Less;
            }
            if j == node.len() {
                return Ordering::Greater;
            }
            let val = self.get(i).compare(node.get(j));
            if val != Ordering::Equal {
                return val;
            }
            i += 1;
            j += 1;
        }
        Ordering::Equal
    }
}

#[derive(Debug)]
enum Node {
    Number(u8),
    List(ListNode),
}

impl Node {
    pub fn compare(&self, other: &Node) -> Ordering {
        match (self, other) {
            (Node::Number(u), Node::Number(v)) => {
                if v == u {
                    return Ordering::Equal
                } else if u < v {
                    return Ordering::Less
                } else {
                    return Ordering::Greater
                }
            },
            (Node::List(u), Node::List(v)) => {
                return u.compare(v);
            },
            (Node::List(u), Node::Number(v)) => {
                let new_list = ListNode::single(Box::new(Node::Number(*v)));
                return u.compare(&new_list);
            },
            (Node::Number(u), Node::List(v)) => {
                let new_list = ListNode::single(Box::new(Node::Number(*u)));
                return new_list.compare(v);
            }
        }
    }
}

/// Return index of first comma (or end of string), where bracket count is <= 0
fn find_index(slice: &str) -> usize {
    let mut bracket_count = 0;
    for (i, c) in slice.chars().enumerate() {
        if c == ',' && bracket_count == 0 {
            return i;
        }
        if c == '[' {
            bracket_count += 1;
        }
        if c == ']' {
            bracket_count -= 1;
        }
    }
    slice.len()
}

/// Invariant -> inp is a string slice that starts with '[' and ends with ']'
/// So, split by comma -> if it is a number, just add a Number to cur_node
/// If it is a ListNode -> create a new ListNode, and recurse
/// Since we split by comma, the new string slice will respect the invariant
fn parse_packets(cur_node: &mut ListNode, inp: &str) {
    // If empty list, return early
    if inp.len() <= 2 {
        return;
    }
    let mut start_index = 1;
    let end_index = inp.len() - 1;
    while start_index < end_index {
        let cur_slice = &inp[start_index..end_index];
        let ind = find_index(cur_slice);
        let node: &str = &cur_slice[0..ind];
        match node.parse::<u8>() {
            Ok(val) => cur_node.push(Box::new(Node::Number(val))),
            Err(_) => {
                let mut new_node = ListNode::new();
                parse_packets(&mut new_node, &node);
                cur_node.push(Box::new(Node::List(new_node)));
            }
        }
        start_index += ind + 1;
    }
}

fn parse_input(input: Vec<String>) -> Vec<Pair> {
    let mut res = vec![];
    for inp in input {
        if inp.is_empty() {
            continue;
        } else {
            let mut new_node = ListNode::new();
            parse_packets(&mut new_node, &inp);
            res.push(new_node);
        }
    }
    let mut new_res = vec![];
    let mut i = 0;
    while i < res.len() {
        new_res.push((Node::List(take(&mut res[i])), Node::List(take(&mut res[i + 1]))));
        i += 2;
    }
    new_res
}

struct Session {
    pairs: Vec<Pair>
}

impl Session {
    pub fn new(pairs: Vec<Pair>) -> Self {
        Session {
            pairs: pairs
        }
    }

    pub fn sum_right_ordered_indices(&self) -> usize {
        let mut res = 0;
        for i in 0..self.pairs.len() {
            //println!("Pair: {}, LHS: {:?}", i, self.pairs[i].0);
            //println!("Pair: {}, RHS: {:?}", i, self.pairs[i].1);
            match self.pairs[i].0.compare(&self.pairs[i].1) {
                Ordering::Less => res += i + 1,
                _ => (),
            } 
            //println!("Pair: {}, Comparison: {:?}", i + 1, self.pairs[i].0.compare(&self.pairs[i].1));
            //println!("");
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let pairs = parse_input(read_input("in.test").unwrap());
        let session = Session::new(pairs);
        let part1 = session.sum_right_ordered_indices();
        println!("Test 1: {}", part1);
    }

    #[test]
    fn actual() {
        let pairs = parse_input(read_input("in.1").unwrap());
        let session = Session::new(pairs);
        let part1 = session.sum_right_ordered_indices();
        println!("Part 1: {}", part1);
    }
}
