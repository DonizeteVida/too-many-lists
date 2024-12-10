#[derive(Default, Debug)]
pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
}

impl<T> LinkedList<T> {
    pub fn push(&mut self, item: T) {
        let prev = std::mem::replace(&mut self.head, None);
        self.head = Some(Box::new(Node::new(item, prev)));
    }

    pub fn pop(&mut self) -> Option<T> {
        match std::mem::replace(&mut self.head, None) {
            None => None,
            Some(item) => {
                self.head = item.next;
                Some(item.value)
            }
        }
    }
}

#[derive(Debug)]
struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn new(value: T, next: Option<Box<Node<T>>>) -> Self {
        Self { value, next }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_add_and_pop() {
        let mut list = LinkedList::<u32>::default();
        list.push(1);
        list.push(2);
        list.push(3);
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }
}
