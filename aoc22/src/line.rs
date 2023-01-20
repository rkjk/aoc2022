struct Tmp {
    first: Option<usize>,
    last: Option<usize>,
}

impl Tmp {
    pub fn new() -> Self {
        Tmp {
            first: None,
            last: None,
        }
    }

    pub fn fill(&mut self, val: usize) {
        if self.first.is_none() {
            self.first = Some(val);
            
        }
        self.last = Some(val);
    }
}

struct Line {
    pub first: usize,
    pub last: usize,
}