use std::vec;

pub fn quick_sort(slice: &[i32]) -> Vec<i32> {
    if slice.is_empty() {
        return Vec::new();
    }
    fn quick_sort_mut(arr: &mut [i32], left: usize, right: usize) {
        if arr.len() <= 1 || left >= right {
            return;
        }

        let pivot = {
            let pivot = arr[right];
            let mut i = left;
            for j in left..right {
                if arr[j] <= pivot {
                    arr.swap(i, j);
                    i += 1;
                }
            }
            arr.swap(i, right);
            i
        };
        if pivot > 0 {
            quick_sort_mut(arr, left, pivot - 1);
        }
        quick_sort_mut(arr, pivot + 1, right);
    }

    let mut sorted = slice.to_vec();
    quick_sort_mut(&mut sorted, 0, slice.len() - 1);
    sorted
}

pub fn comparison_sort(slice: &[i32]) -> Vec<i32> {
    if slice.is_empty() {
        return Vec::new();
    }
    let mut sorted = slice.to_vec();
    let mut is_clean = true;

    while is_clean {
        let arr_len = slice.len();
        for j in 0..arr_len - 1 {
            if j == arr_len - 1 {
                continue;
            }
            let (current, next) = (sorted[j], sorted[j + 1]);
            if current > next {
                is_clean = false;
                sorted[j] = next;
                sorted[j + 1] = current;
            }
        }
        if is_clean {
            break;
        }
        is_clean = true;
    }

    sorted
}

pub fn counting_sort(slice: &[usize]) -> Vec<usize> {
    if slice.is_empty() {
        return Vec::new();
    }
    let mut sorted = vec![0; slice.len()];
    let mut counter = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

    for elem in slice {
        counter[*elem] += 1;
    }
    for i in 1..10 {
        counter[i] += counter[i - 1];
    }

    for elem in slice {
        let value = *elem;
        let new_index = counter[value] - 1;
        sorted[new_index] = value;
        counter[value] -= 1;
    }

    sorted
}

#[cfg(test)]
mod test {
    use crate::algorithms::sorting::{comparison_sort, counting_sort, quick_sort};

    #[test]
    fn comparison_sort_works() {
        assert_eq!(comparison_sort(&[1, 2, 3]), [1, 2, 3]);
        assert_eq!(comparison_sort(&[3, 2, 1]), [1, 2, 3]);
        assert_eq!(comparison_sort(&[3, 9, 2, 1]), [1, 2, 3, 9]);
        assert_eq!(comparison_sort(&[]), []);
    }

    #[test]
    fn quick_sort_works() {
        assert_eq!(quick_sort(&[1, 2, 3]), [1, 2, 3]);
        assert_eq!(quick_sort(&mut [3, 2, 1]), [1, 2, 3]);
        assert_eq!(quick_sort(&[3, 2, 9, 1]), [1, 2, 3, 9]);
        assert_eq!(quick_sort(&[]), []);
    }

    #[test]
    fn counting_sort_works() {
        assert_eq!(counting_sort(&[1, 2, 3]), [1, 2, 3]);
        assert_eq!(counting_sort(&[1, 4, 1, 2, 7, 5, 2]), [1, 1, 2, 2, 4, 5, 7]);
        assert_eq!(counting_sort(&[3, 9, 2, 1]), [1, 2, 3, 9]);
        assert_eq!(counting_sort(&[]), []);
    }
}
