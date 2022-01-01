use crate::algorithms::{
    search::binary_search,
    sorting::{comparison_sort, counting_sort, quick_sort},
};

mod algorithms;

fn main() {
    println!("Hello, world!");
    quick_sort(&[3, 2, 1]);
    comparison_sort(&[3, 2, 1]);
    binary_search(&[1, 2, 3], 3);
    counting_sort(&[1, 2, 3]);
}
