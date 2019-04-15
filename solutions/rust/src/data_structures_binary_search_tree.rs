struct Node {
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
    value: i32,
}

impl Node {
    fn create_node(val: i32) -> Node {
        Node {
            value: val,
            left: None,
            right: None,
        }
    }

    fn add_child(self, node: Node) -> Node {
        if node.value < self.value {
            return match self.left {
                Some(child) => Node {
                    value: self.value,
                    left: Some(Box::new(child.add_child(node))),
                    right: self.right,
                },
                None => Node {
                    value: self.value,
                    left: Some(Box::new(node)),
                    right: self.right,
                }
            };
        }

        return match self.right {
            Some(child) => Node {
                value: self.value,
                left: self.left,
                right: Some(Box::new(child.add_child(node))),
            },
            None => Node {
                value: self.value,
                left: self.left,
                right: Some(Box::new(node)),
            }
        };
    }

    fn find_val(&self, search_val: i32) -> Result<&Node, String> {
        if search_val == self.value {
            Ok(self)
        } else if self.value > search_val {
            //let left: Option<Node> = *(self.left);
            return match &self.left {
                Some(child) => child.find_val(search_val),
                None => Err(format!("Unable to find {}", search_val)),
            };
        } else {
            //let right: Option<Node> = *(self.right);
            return match &self.right {
                Some(child) => child.find_val(search_val),
                None => Err(format!("Unable to find {}", search_val)),
            };
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {
        let mut root: Node = Node::create_node(0);
        let mut tree_values: Vec<i32> = vec![0];
        let len = 100;

        for i in 1..len {
            let mut val = i;
            if i % 2 == 0 {
                val = val * -1;
            }

            tree_values.push(val);
            root = root.add_child(Node::create_node(val));
        }

        for j in 0..tree_values.len() - 1 {
            let val = tree_values[j];

            match root.find_val(val) {
                Ok(return_val) => assert!(return_val.value == val, "{} is not equal to {}", val, return_val.value),
                Err(e) => panic!("Encountered error \"{}\"", e)
            }
        }
    }

}
