use crate::value::Value;

#[derive(Debug)]
pub struct List {
    pub contents: Box<[Value]>,
}

pub struct ListBuilder {
    contents: Vec<Value>,
}

impl List {
    pub fn build() -> ListBuilder { ListBuilder::new() }
}

impl ListBuilder {
    pub fn new() -> Self { Self { contents: Vec::new() } }

    pub fn with(&mut self, item: Value) {
        self.contents.push(item);
    }

    pub fn build(self) -> List {
        List { contents: self.contents.into_boxed_slice() }
    }
}