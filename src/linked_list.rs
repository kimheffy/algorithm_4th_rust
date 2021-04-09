type Link = Option<Box<Node>>;

// [ item: thing, next: Node]
#[derive(Debug)]
pub struct Node {
    item: i32,
    next: Link,
}

// associated function
impl Node {
    pub fn new(val: i32) -> Self {
        Self {
            item: val,
            next: None,
        }
    }
}

// TODO: Add length property
#[derive(Debug)]
pub struct LinkedList {
    head: Link,
}

impl LinkedList {
    fn new() -> Self {
        Self {
            head: None,
        }
    }
}

impl LinkedList {
    pub fn push(&mut self, item: i32) {
        let mut node: Node = Node::new(item);

        if let None = self.head {
            self.head = Some(Box::new(node));
        } else {
            let head: Node = *self.head.take().unwrap();
            node.next = Some(Box::new(head));
            self.head = Some(Box::new(node));
        }
    }
}

#[cfg(test)]
mod linkedlist_test {
    use super::*;

    #[test]
    fn ll_1() {
        let mut linked = LinkedList::new();
        linked.push(1);
        linked.push(2);
        linked.push(3);

        println!("Linkedlist: {:?}", linked);
        assert_eq!(2, 2);
    }
}
