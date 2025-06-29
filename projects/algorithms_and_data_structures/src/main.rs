const MAX_LEVELS: usize = 100;

mod binary_add;

mod heap;

mod merge_sorted_list;

mod macros;

mod d_ary_heap;

mod sort;

mod random;

mod min_max;

mod inversions;

mod matrix;

use sort::{quicksort_ineficient, Order};

fn main() {
    /*
    let heap = Heap::build_heap(vec![10, 4, 8, 3, 4, 6, 7, 1, 2, 1, -5, 0, 2, 1, 0]);
    let mut d_ary_heap = DAryHeap::new(4, vec![10, 4, 8, 3, 4, 6, 7, 1, 2, 1, -5, 0, 2, 1, 0]);
    println!("{heap}");
    let sorted_array = heap.heapsort();
    println!("{sorted_array}");

    println!("Max from dary is: {}", d_ary_heap.extract_max().unwrap());
    */
    let mut arr = [1, 2, -23];
    let len = arr.len();
    println!("Before: {arr:?}");
    quicksort_ineficient(&mut arr, 0, len, Order::Decreasing);
    println!("After: {arr:?}");
}
