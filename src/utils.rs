pub trait IIter: Iterator {
    fn prev(&mut self) -> Option<Self::Item>;
}

pub struct Iter<'a, Item> where Item: 'a {
    index: Option<usize>,
    vector: &'a Vec<Item>,
}

impl<'a, Item> Iter<'a, Item> {
    pub fn new(vector: &'a Vec<Item>) -> Iter<'a, Item> {
        Iter { index: None, vector }
    }
}

impl<'a, Item> Iterator for Iter<'a, Item> {
    type Item = &'a Item;

    fn next(&mut self) -> Option<&'a Item> {
        let index =
            match self.index {
                Some(i) => i + 1,
                None => 0
            };

        self.index = Some(index);
        self.vector.get(index)
    }
}

impl<'a, Item> IIter for Iter<'a, Item> {
    fn prev(&mut self) -> Option<&'a Item> {
        let index =
            match self.index {
                Some(0) | None => return None,
                Some(i) => i - 1
            };

        self.index = Some(index);
        self.vector.get(index)
    }
}