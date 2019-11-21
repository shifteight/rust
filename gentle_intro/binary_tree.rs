type NodeBox = Option<Box<Node>>;

#[derive(Debug)]
struct Node {
    payload: String,
    left: NodeBox,
    right: NodeBox,
}

impl Node {
    fn new(s: &str) -> Node {
        Node {
            payload: s.to_string(),
            left: None,
            right: None,
        }
    }
    fn boxer(node: Node) -> NodeBox {
        Some(Box::new(node))
    }
    fn set_left(&mut self, node: Node) {
        self.left = Self::boxer(node);
    }
    fn set_right(&mut self, node: Node) {
        self.right = Self::boxer(node);
    }

    fn insert(&mut self, data: &str) {
        if data < &self.payload {
            match self.left {
                Some(ref mut n) => n.insert(data),
                None => self.set_left(Self::new(data)),
            }
        } else {
            match self.right {
                Some(ref mut n) => n.insert(data),
                None => self.set_right(Self::new(data)),
            }
        }
    }

    fn visit(&self) {
        if let Some(ref left) = self.left {
            left.visit();
        }
        println!("'{}'", self.payload);
        if let Some(ref right) = self.right {
            right.visit();
        }
    }
}

fn main() {
    let mut root = Node::new("root");
    root.insert("one");
    root.insert("two");
    root.insert("four");

    println!("{:#?}", root);

    root.visit();
}
