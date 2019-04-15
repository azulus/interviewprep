struct Range (usize, usize);

fn swap_vals(vals: &mut Vec<i32>, idx1: usize, idx2: usize) {
    //println!("before {} {}", vals[idx1], vals[idx2]);
    let temp_val = vals[idx1];
    vals[idx1] = vals[idx2];
    vals[idx2] = temp_val;
    //println!("after {} {}", vals[idx1], vals[idx2]);
}

fn quick_sort(vals: &Vec<i32>) -> Vec<i32> {
    let mut new_vals = vals.to_vec();
    let mut ranges = Vec::new();

    if new_vals.len() == 0 {
        return new_vals;
    }

    ranges.push(Range(0, new_vals.len() - 1));

    let mut idx = 0;
    while idx < ranges.len() {
        let range = &ranges[idx];
        let start = range.0;
        let end = range.1;
        //println!("swapping {} to {}", start, end);

        // pick a pivot
        let mid = (end - start) / 2 + start;
        let pivot_val = new_vals[mid];

        // move the pivot with the end
        swap_vals(&mut new_vals, mid, end);
        let mut pivot_idx = start;

        println!("pivot to end {} {:?}", pivot_val, new_vals);

        for i in start..end {
            if new_vals[i] <= pivot_val {
                swap_vals(&mut new_vals, i, pivot_idx);
                pivot_idx = pivot_idx + 1;
            }
        }

        swap_vals(&mut new_vals, end, pivot_idx);
        if pivot_idx > start + 1 {
            ranges.push(Range(start, pivot_idx - 1))
        }
        if pivot_idx + 1 < end {
            ranges.push(Range(pivot_idx + 1, end))
        }

        println!("pivot {} {:?}", pivot_val, new_vals);

        idx = idx + 1;
    }

    new_vals
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {
        let unsorted_vals = vec![38, 17, -4, 12, 4, 16, -22, 8, 6, 21, 25];

        let mut sorted_vals = unsorted_vals.to_vec();
        sorted_vals.sort();

        let quicksort_vals = quick_sort(&unsorted_vals);
        println!("{:?}", quicksort_vals);

        for i in 0..sorted_vals.len() - 1 {
            assert!(sorted_vals[i] == quicksort_vals[i], "{} does not equal {}", quicksort_vals[i], sorted_vals[i]);
        }
    }
}
