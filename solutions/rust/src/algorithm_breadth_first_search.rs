struct Node {
    left: Box<Option<Node>>,
    right: Box<Option<Node>>,
    value: i32,
}

impl Node {
    fn add_child(&mut self, node: &mut Node) -> &mut Node {

        self
    }

    fn find_val(&self, search_val: i32) -> Result<Node, String> {

        Err("This is a test".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {
        assert!(true);
    }

}
