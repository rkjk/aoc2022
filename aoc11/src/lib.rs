pub mod example;
pub mod monkey;
pub mod actual;

use monkey::{Monkey, Item};

/// Part 1 is straightforward implement as in the question.
/// Part 2 is the Yearly Chinese Remainder Theorem question.
/// Basically every monkey has a check to determine which monkey to throw to.
/// This check is to see if divisible by a number. So, if we take the LCM of these numbers
/// and take modulo of worry after applying the new  = fn(old) worry function. The result is unchanged.
/// Not doing so would result in integer overflow 
struct Session {
    monkeys: Vec<Monkey>,
    counts: Vec<usize>,
    lcm: Item
}

impl Session {
    pub fn new(monkeys: Vec<Monkey>, lcm: Item) -> Self {
        let size = monkeys.len();
        Session {
            monkeys: monkeys,
            counts: vec![0; size],
            lcm: lcm
        }
    }

    fn run_turn(&mut self, index: usize, should_worry: bool) {
        let size = self.monkeys.len();
        let monkey_ptr: *mut Monkey = &mut self.monkeys[index];
        unsafe {
            let monkey_ref = monkey_ptr.as_mut().unwrap();
            match monkey_ref.is_empty() {
                true => return,
                false => {
                    self.counts[index] += monkey_ref.items.len();
                    while !monkey_ref.items.is_empty() {
                        let v = monkey_ref.items.pop_front().unwrap();
                        let worry = (monkey_ref.worry_fn)(v);
                        let worry = match should_worry {
                            true => worry % self.lcm,
                            false => worry / 3
                        };
                        let destination = (monkey_ref.test_fn)(worry);
                        if destination < 0 || destination > size - 1 {
                            panic!("destination exceeds size: {}", destination);
                        }
                        let dest_ptr: *mut Monkey = &mut self.monkeys[destination];
                        dest_ptr.as_mut().unwrap().items.push_back(worry);
                    }
                }
            }
        }
    }

    pub fn run_round(&mut self, should_worry: bool) {
        let size = self.monkeys.len();
        for i in 0..size {
            self.run_turn(i, should_worry);
        }
    }

    pub fn test(&mut self) {
        self.run_round(false);
        for i in 0..self.monkeys.len() {
            self.monkeys[i].print();
        }
        println!("Counts: {:?}", self.counts);
    }

    pub fn run_rounds(&mut self, num_rounds: usize, should_worry: bool) -> usize {
        for i in 0..num_rounds {
            self.run_round(should_worry);
        }
        let mut counts = self.counts.clone();
        counts.sort();
        counts[counts.len() -1] * counts[counts.len() - 2]
    }
}

#[cfg(test)]
mod tests {
    use crate::example::LCM_OF_MODS;

    use super::*;

    #[test]
    fn it_works() {
        use example::*;
        let inp = get_example_monkeys();
        let mut session = Session::new(inp, 0);
        let part1 = session.run_rounds(20, false);
        let mut session2 = Session::new(get_example_monkeys(), LCM_OF_MODS);
        let part2 = session2.run_rounds(10000, true);
        println!("Test 1: {}", part1);
        println!("Test 2: {}", part2);
    }

    #[test]
    fn actual() {
        use std::time::Instant;
        let now = Instant::now();
        use actual::*;
        let inp = get_actual_monkeys();
        let mut session = Session::new(inp, 0);
        let part1 = session.run_rounds(20, false);
        let mut session2 = Session::new(get_actual_monkeys(), LCM_OF_MODS);
        let part2 = session2.run_rounds(10000, true);
        let elapsed = now.elapsed();
        println!("Part 1: {}", part1);
        println!("Part 2: {}", part2);
        println!("Elapsed: {:.2?}", elapsed);
    }
}
