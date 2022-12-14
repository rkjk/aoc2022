use std::collections::VecDeque;

pub type Item = u64;

const LCM_OF_MODS: u64 = 9699690;

pub struct Monkey {
    pub items: VecDeque<Item>,
    pub worry_fn: Box<dyn Fn(Item) -> Item>,
    pub test_fn: Box<dyn Fn(Item) -> usize>
}

impl Monkey {
    pub fn new(
        items: VecDeque<Item>,
        worry: Box<dyn Fn(Item) -> Item>,
        test: Box<dyn Fn(Item) -> usize>) -> Self {
        Monkey {
            items: items,
            worry_fn: worry,
            test_fn: test
        }
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn print(&self) {
        println!("items {:?}", self.items);
    }

    pub fn size(&self) -> usize {
        self.items.len()
    }
}