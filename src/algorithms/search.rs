pub fn binary_search(slice: &[i32], target: i32) -> Option<usize> {
    if slice.is_empty() {
        return None;
    }

    let mut left: usize = 0;
    let mut right = slice.len() - 1;

    while left <= right {
        let mid = (left + right) / 2;
        if slice[mid] == target {
            return Some(mid);
        }
        if slice[mid] > target {
            right = mid - 1;
            continue;
        }
        if slice[mid] < target {
            left = mid + 1;
            continue;
        }
    }

    None
}

#[cfg(test)]
mod test {
    use crate::algorithms::search::binary_search;

    #[test]
    fn binary_search_works() {
        assert_eq!(binary_search(&[], 4), None);
        assert_eq!(binary_search(&[1, 2, 3, 4,], 4), Some(3));
        assert_eq!(binary_search(&[1, 2, 3, 10], 4), None);
    }
}
