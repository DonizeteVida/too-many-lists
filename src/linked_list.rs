pub fn hello_world() {
    println!("Hello WOrld");
}

#[derive(Default, Debug)]
pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
}

impl<T> LinkedList<T> {
    pub fn push(&mut self, item: T) {
        let prev = std::mem::replace(&mut self.head, None);
        self.head = Some(Box::new(Node::new(item, prev)));
    }
}

#[derive(Debug)]
struct Node<T> {
    item: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn new(item: T, next: Option<Box<Node<T>>>) -> Self {
        Self { item, next }
    }
}
