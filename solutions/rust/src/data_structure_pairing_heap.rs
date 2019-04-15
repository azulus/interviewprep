#[derive(Debug, Clone)]
struct Node {
    subheaps: Box<Vec<Node>>,
    value: i32,
}

impl Node {
    fn create_node(val: i32) -> Node {
        Node {
            value: val,
            subheaps: Box::new(vec![]),
        }
    }

    fn merge_into(node1: Node, node2: Node) -> Node {
        let mut subheaps = node2.subheaps.clone();
        subheaps.push(node1);
        Node {
            value: node2.value,
            subheaps: subheaps,
        }
    }

    fn merge_two(node1: Node, node2: Node) -> Node {
        let node = match node1.value < node2.value {
            true => Node::merge_into(node2, node1),
            false => Node::merge_into(node1, node2),
        };
        node
    }

    fn merge_multiple(subheaps1: Vec<Node>, subheaps2: Vec<Node>) -> Node {
        let node = Node::merge_two(
           Node::merge_subheaps(subheaps1),
           Node::merge_subheaps(subheaps2)
       );

       node
    }

    fn merge_subheaps(subheaps: Vec<Node>) -> Node {
        let len = subheaps.len();
        let mid = len / 2;

        let mut total_len = 0;
        for i in 0..subheaps.len() {
            total_len = total_len + subheaps[i].get_size();
        }

        let after = match len {
            1 => subheaps[0].clone(),
            2 => Node::merge_two(subheaps[0].clone(), subheaps[1].clone()),
            _ => Node::merge_multiple(
               subheaps[0..mid].to_vec(),
               subheaps[mid..len].to_vec(),
            ),
        };

        after
    }

    fn insert(self, node: Node) -> Node {
        Node::merge_two(self, node)
    }

    fn remove_min(self) -> Option<Node> {
        if self.subheaps.len() == 0 {
            return None;
        }

        Some(Node::merge_subheaps(*(self.subheaps)))
    }

    fn get_size(&self) -> usize {
        (1 as usize) + Node::get_subheaps_sizes(&self.subheaps)
    }

    fn get_subheaps_sizes(subheaps: &Vec<Node>) -> usize {
        let mut size: usize = 0;

        for i in 0..subheaps.len() {
            size = size + subheaps[i].get_size();
        }

        size
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {
        let mut root: Node = Node::create_node(0);
        let mut tree_values = vec![38, 17, -4, 12, 4, 16, -22, 8, 6, 21, 25];

        for i in 0..tree_values.len() {
            root = root.insert(Node::create_node(tree_values[i]));
        }
        tree_values.push(0);
        let len = tree_values.len();
        tree_values.sort();

        println!("{:?}", tree_values);
        let mut curr_root = Some(root);

        for i in 0..len {
            let val = tree_values[i];
            println!("{} remaining", len - i);
            match &curr_root {
                Some(node) => assert!(node.get_size() == (len as usize) - i, "Remaining nodes count {} does not match expected {}", node.get_size(), len - i),
                None => panic!("Reached end of the pairing heap"),
            }

            match &curr_root {
                Some(node) => assert!(val == node.value, "{} does not equal {}", val, node.value),
                None => panic!("Reached end of the pairing heap"),
            };

            curr_root = match curr_root {
                Some(node) => node.remove_min(),
                None => None,
            }
        }

        assert!(true);
    }
}
