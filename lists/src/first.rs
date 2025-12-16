pub struct List {
    head: Link,
}

enum Link {
    Empty,
    More(Box<Node>),
}

struct Node {
    elem: i32,
    next: List,
}

impl List {
    // todo make some stuff happen
    pub fn new() -> Self {
        List { head: Link::Empty }
    }
}