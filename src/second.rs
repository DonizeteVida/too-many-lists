#[derive(Default, Debug)]
pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
}

impl<T> LinkedList<T> {
    pub fn push(&mut self, item: T) {
        let node = Node::new(item, self.head.take());
        self.head = Some(Box::new(node));
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.value
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.value)
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.value)
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

    #[test]
    fn can_add_and_peek() {
        let mut list = LinkedList::<u32>::default();
        list.push(1);
        assert_eq!(list.peek(), Some(&1));
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn can_add_and_peek_mut() {
        let mut list = LinkedList::<u32>::default();
        list.push(2);
        list.peek_mut().map(|node| {
            *node *= 41;
        });
        assert_eq!(list.pop(), Some(82));
        assert_eq!(list.pop(), None);
    }
}
