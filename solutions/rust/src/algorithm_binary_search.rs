fn binary_search() -> Result<usize, String>{
    Err("start with failure".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {
        let result = binary_search();

        match result {
            Ok(_) => assert!(true),
            Err(e) => panic!("Encountered error \"{}\"", e)
        }
    }
}
