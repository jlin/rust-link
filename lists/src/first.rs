use std::mem;

pub struct List {
    head: Link,
}

enum Link {
    Empty,
    More(Box<Node>),
}

struct Node {
    elem: i32,
    next: Link,
}

impl List {
    // todo make some stuff happen
    pub fn new() -> Self {
        List { head: Link::Empty }
    }
}

impl List {
    pub fn push(&mut self, elem: i32){
        let new_node = Box::new(Node {
            elem: elem,
            next: mem::replace(&mut self.head, Link::Empty),
        });
        self.head = Link::More(new_node);
    }
}

impl List {
    pub fn pop(&mut self) -> Option<i32> {
    match mem::replace(&mut self.head, Link::Empty) {
        Link::Empty => None,
        Link::More(node) => {
            self.head = node.next;
            Some(node.elem)
        }
    }
    }
}

// crazy drop to avoid recursion
impl Drop for List {
    fn drop(&mut self) {
        let mut cur_link = mem::replace(&mut self.head, Link::Empty);
        // while let == do this until pattern doesn't match
        while let Link::More(mut boxed_node)= cur_link {
            cur_link = mem::replace(&mut boxed_node.next, Link::Empty);
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

}