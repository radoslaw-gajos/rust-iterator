struct SimpleIterator<T: Clone> {
    list: Vec<T>,
    current: usize,
}

impl<T: Clone> SimpleIterator<T> {
    fn new(list: Vec<T>) -> Self {
        Self {
            list,
            current: 0,
        }
    }
}

impl<T: Clone> Iterator for SimpleIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.current >= self.list.len() {
            return None
        }
        Some(self.list[{
            let index = self.current;
            self.current += 1;
            index
        }].clone())
    }
}

fn main() {
    let iterator = SimpleIterator::new(vec!(1,2,3,4,5));

    for item in iterator {
        println!("{item}");
    }
}
