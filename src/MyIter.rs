
/// A customized iterator for KvPair
/// NOTE: Yet unimplemented. Just ignore this file.
struct MyIter<I> {
    iter: I,
    index: usize,
}

impl<I> Iterator for MyIter<I> where I: Iterator {
    type Item = (usize, I::Item);

    fn next(&mut self) -> Option<(usize, I::Item)> {
        match self.iter.next() {
            None => None,
            Some(v) => {
                let index = self.index;
                self.index += 1;
                Some((index, v))
            }
        }
    }
}

impl<I> MyIter<I> {
    fn new(iter: I, items: u32) -> MyIter<I> {
        MyIter { iter: iter, index: 0 }
    }
}
