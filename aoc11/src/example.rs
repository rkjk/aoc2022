use crate::monkey::{Monkey, Item};

use std::collections::VecDeque;

pub fn get_example_monkeys() -> Vec<Monkey> {
    vec![
        Monkey::new(
            VecDeque::from([79, 98]), 
            Box::new(|old: Item| -> Item { old * 19 }),
            Box::new(|val: Item| -> usize { 
                match val % 23 == 0 {
                    true => 2,
                    false => 3
                }
            })
        ),
        Monkey::new(
            VecDeque::from([54, 65, 75, 74]), 
            Box::new(|old: Item| -> Item { old + 6 }),
            Box::new(|val: Item| -> usize { 
                match val % 19 == 0 {
                    true => 2,
                    false => 0
                }
            })
        ),
        Monkey::new(
            VecDeque::from([79, 60, 97]), 
            Box::new(|old: Item| -> Item { old * old }),
            Box::new(|val: Item| -> usize { 
                match val % 13 == 0 {
                    true => 1,
                    false => 3
                }
            })
        ),
        Monkey::new(
            VecDeque::from([74]), 
            Box::new(|old: Item| -> Item { old + 3 }),
            Box::new(|val: Item| -> usize { 
                match val % 17 == 0 {
                    true => 0,
                    false => 1
                }
            })
        ),
    ]
}