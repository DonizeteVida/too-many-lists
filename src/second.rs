#[derive(Default, Debug)]
pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
}

impl<T> LinkedList<T> {
    pub fn push(&mut self, item: T) {
        let node = Node {
            item,
            next: self.head.take(),
        };
        self.head = Some(Box::new(node));
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.item
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.item)
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.item)
    }

    pub fn into_iter(self) -> Iter<T> {
        Iter(self)
    }

    pub fn iter(&self) -> IntoIter<'_, T> {
        IntoIter(self.head.as_deref())
    }
}

pub struct Iter<T>(LinkedList<T>);

impl<T> Iterator for Iter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

pub struct IntoIter<'a, T>(Option<&'a Node<T>>);

impl<'a, T> Iterator for IntoIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.map(|node| {
            self.0 = node.next.as_deref();
            &node.item
        })
    }
}

#[derive(Debug)]
struct Node<T> {
    item: T,
    next: Option<Box<Node<T>>>,
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

    #[test]
    fn can_add_and_into_iter() {
        let mut list = LinkedList::<u32>::default();
        list.push(1);
        list.push(2);
        list.push(3);
        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn can_add_and_iter() {
        let mut list = LinkedList::<u32>::default();
        list.push(1);
        list.push(2);
        list.push(3);
        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), None);
    }
}
