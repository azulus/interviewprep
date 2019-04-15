use std::collections::HashMap;

struct Trie {
    value: Option<String>,
    children: HashMap<char, Trie>,
}

impl Trie {
    fn new() -> Trie {
        Trie {
            value: None,
            children: HashMap::new(),
        }
    }

    fn insert(&mut self, str: String) {
        let mut curr = self;

        for c in str.chars() {
            curr.children.entry(c).or_insert(Trie::new());
            curr = curr.children.get_mut(&c).unwrap();
        }

        curr.value = Some(str.clone());
    }

    fn contains(&mut self, str: String) -> bool {
        let mut curr = self;

        for c in str.chars() {
            if !curr.children.contains_key(&c) {
                return false;
            }

            curr = curr.children.get_mut(&c).unwrap();
        }

        return match curr.value {
            Some(_) => true,
            None => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {
        let mut trie = Trie::new();
        &trie.insert("Kevin".to_string());
        &trie.insert("Jeremy".to_string());

        assert!(&trie.contains("Kevin".to_string()));
        assert!(!&trie.contains("Fred".to_string()));
        assert!(&trie.contains("Jeremy".to_string()));
        assert!(!&trie.contains("Jer".to_string()));
    }
}
