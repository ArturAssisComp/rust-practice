const MAX_LEVELS: usize = 100;

mod heap;

mod merge_sorted_list;

mod macros;

mod d_ary_heap;

use d_ary_heap::DAryHeap;
use heap::Heap;

fn main() {
    let heap = Heap::build_heap(vec![10, 4, 8, 3, 4, 6, 7, 1, 2, 1, -5, 0, 2, 1, 0]);
    let mut d_ary_heap = DAryHeap::new(4, vec![10, 4, 8, 3, 4, 6, 7, 1, 2, 1, -5, 0, 2, 1, 0]);
    println!("{heap}");
    let sorted_array = heap.heapsort();
    println!("{sorted_array}");

    println!("Max from dary is: {}", d_ary_heap.extract_max().unwrap());
}
