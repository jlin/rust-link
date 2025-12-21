use std::mem;

pub struct List<T> {
    head: Link<T>,
}

pub struct IntoIter<T>(List<T>);

pub struct Iter<'a, T>{
    next: Option<&'a Node<T>>,
}
// type alias!!
type Link<T> = Option<Box<Node<T>>>;


struct Node<T> {
    elem: T,
    next: Link<T>,
}

// don't actually need lifetimes since elision works
// the '_ hides the elision explicitly so the warnings go away
impl<T> List<T>{

    pub fn iter(&self) -> Iter<'_, T> {
        Iter { next: self.head.as_deref()}
    }
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn into_iter(self) -> IntoIter<T>{
        IntoIter(self)
    }

    pub fn push(&mut self, elem: T){
        let new_node = Box::new(Node {
            elem: elem,
            next: self.head.take(),
        });
        self.head = Some(new_node);
    }

    // use wacky map instead of match option { None => None, Some(x) => Some(y) }
    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| {
            &node.elem
        })
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| {
            &mut node.elem
        })
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        // access fields of tuble struct numerically
        self.0.pop()
    }
}

impl<'a, T> Iterator for Iter<'a, T>{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.elem
        })
    }
}


// crazy drop to avoid recursion
impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        // while let == do this until pattern doesn't match
        while let Some(mut boxed_node)= cur_link {
            cur_link = mem::replace(&mut boxed_node.next, None);
            // boxed_node goes out of scope, gets dropped
            // next is replaced with empty, so doesn't need recursion.
        }
    }

}

#[cfg(test)]
mod test {
    use super::List;
    #[test]
    fn basics() {
        let mut list = List::new();

        // check empty list behaves correctly
        assert_eq!(list.pop(), None);

        // populate list
        list.push(1);
        list.push(2);
        list.push(3);

        // check normal removal

        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        // push more stuff

        list.push(4);
        list.push(5);

        // check normal removal

        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        // check exhaustion

        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);


    }

    #[test]
    fn peek(){
        let mut list = List::new();
        assert_eq!(list.peek(), None);
        assert_eq!(list.peek_mut(), None);
        list.push(1); list.push(2); list.push(3);

        assert_eq!(list.peek(), Some(&3));
        assert_eq!(list.peek_mut(), Some(&mut 3));
        list.peek_mut().map(|value| {
            *value = 42
        });

        assert_eq!(list.peek(), Some(&42));
        assert_eq!(list.pop(), Some(42));
    }

    #[test]
    fn into_iter() {
        let mut list = List::new();
        list.push(1); list.push(2); list.push(3);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);

    }

    #[test]
    fn iter() {
        let mut list = List::new();
        list.push(1); list.push(2); list.push(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));

    }

}