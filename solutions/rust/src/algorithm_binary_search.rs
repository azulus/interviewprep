fn binary_search(haystack: &Vec<i32>, needle: i32) -> Result<usize, String>{
    let mut start: usize = 0;
    let mut end: usize = haystack.len() - 1;
    let mut mid: usize;
    let mut mid_val: i32;

    loop {
        if end < start {
            break;
        }

        mid = (end - start) / 2 + start;
        mid_val = haystack[mid];

        if mid_val == needle {
            return Ok(mid);
        } else if mid_val > needle {
            end = mid - 1;
        } else {
            start = mid + 1;
        }
    }

    return Err(format!("Exited loop without finding {} in vec", needle).to_string());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {
        let len = 100;
        let mut v: Vec<i32> = Vec::new();
        for i in 0..len {
            v.push(i * 100);
        }

        for j in 0..len {
            let k = len - j - 1;
            let val = k * 100;
            let expected_index: usize = k as usize;

            match binary_search(&v, val) {
                Ok(returned_idx) => assert!(expected_index == returned_idx),
                Err(e) => panic!("Encountered error \"{}\"", e)
            }
        }
    }
}
