struct Node {
    children: Box<Vec<Node>>,
    value: i32,
}

impl Node {
    fn create_node(val: i32) -> Node {
        Node {
            children: Box::new(vec![]),
            value: val,
        }
    }

    fn add_child(&mut self, node: Node) {
        self.children.push(node);
    }

    fn find_node_breadth_first(root: &Node, val: i32) -> Result<&Node, String> {
        let mut search_nodes = vec![root];
        let mut idx: usize = 0;

        while idx < search_nodes.len() {
            let next = search_nodes[idx];

            if next.value == val {
                return Ok(next);
            }

            for i in 0..next.children.len() {
                search_nodes.push(&(next.children[i]));
            }

            idx = idx + 1;
        }

        return Err("Couldn't find node".to_string());
    }

    fn get_size(&self) -> i32 {
        let mut size = 1;
        for i in 0..self.children.len() {
            size = size + self.children[i].get_size();
        }
        size
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ctor() {
        let val = 123;
        let node = Node::create_node(val);
        assert!(node.value == val);
    }

    #[test]
    fn test_child() {
        let val = 123;
        let mut node = Node::create_node(val);

        let child_val = 456;
        let child = Node::create_node(child_val);
        node.add_child(child);

        assert!(node.children.len() == 1);
        assert!(node.children[0].value == child_val);
    }

    fn _add_children(node: &mut Node, count: i32, additional_tiers: i32) -> i32 {
        let mut num_children = 0;
        let base_val = node.value * 10;
        for i in 0..count {
            let mut child = Node::create_node(base_val + i + 1);
            num_children = num_children + 1;
            if additional_tiers > 0 {
                num_children = num_children + _add_children(&mut child, count, additional_tiers - 1);
            }
            node.add_child(child);
        }
        return num_children;
    }

    #[test]
    fn test_search() {
        let num_tiers = 4;
        let per_tier = 4;

        let mut root = Node::create_node(0);

        let num_nodes = 1 + _add_children(&mut root, per_tier, num_tiers - 1);
        assert!(root.get_size() == num_nodes, "Expected size {} but received {}", num_nodes, root.get_size());

        match Node::find_node_breadth_first(&root, 1) {
            Ok(n) => assert!(n.value == 1),
            Err(e) => panic!(e),
        };

        match Node::find_node_breadth_first(&root, 5) {
            Ok(n) => panic!("Should not find result, received {}", n.value),
            Err(_) => assert!(true, "should not find 5"),
        };

        match Node::find_node_breadth_first(&root, 3123) {
            Ok(n) => assert!(n.value == 3123),
            Err(e) => panic!(e),
        };

        match Node::find_node_breadth_first(&root, 3125) {
            Ok(n) => panic!("Should not find result, received {}", n.value),
            Err(_) => assert!(true, "should not find 3125"),
        };


    }
}
