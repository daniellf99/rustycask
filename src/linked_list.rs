pub struct Node {
    value: i64,
    next: Option<Box<Node>>,
}

impl Node {
    pub fn new(value: i64) -> Node {
        // Add head
        Node {
            value,
            next: None,
        }
    }

    pub fn append(mut self, value: i64) -> Node {
        let mut current = &mut self;

        while current.next.is_some() {
            current = current.next.as_deref_mut().unwrap();
        }

        current.next = Some(Box::new(Node::new(value)));

        self
    }

    pub fn prepend(self, value: i64) -> Node {
        Node {
            value,
            next: Some(Box::new(self)),
        }
    }

    pub fn print(&self) {
        let mut current = Some(self);
        
        while let Some(node) = current {
            print!("{}", node.value);

            current = node.next.as_deref();

            if current.is_some() {
               print!(" -> ");
            }
        }

        println!();
    }
}