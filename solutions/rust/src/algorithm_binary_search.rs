fn binary_search(haystack: &Vec<i32>, needle: i32) -> Result<usize, String>{
    Err("start with failure".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {
        let mut v: Vec<i32> = Vec::new();
        for i in 1..100 {
            v.push(i * 100);
        }

        for j in 1..100 {
            let k = 100 - j;

            match binary_search(&v, k) {
                Ok(_) => assert!(true),
                Err(e) => panic!("Encountered error \"{}\"", e)
            }
        }
    }
}
