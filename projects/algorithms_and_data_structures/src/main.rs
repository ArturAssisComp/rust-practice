const MAX_LEVELS: usize = 100;

mod heap;

mod merge_sorted_list;

mod d_ary_heap {
    // macros
    /// Given the current index in the array (from 0 to len()-1), this macro returns the
    /// range for all `degree` children that a d-ary heap element may have.
    ///
    /// # Example
    /// ```rust
    /// // Remember that the range is left-inclusive and right-exclusive.
    /// // 1..3 is equivalent to 1..=2
    /// assert_eq!(children_range!(0, 2), 1..3);
    /// ```
    macro_rules! children_range {
        ($i:expr, $degree:expr) => {{
            let min = $i * $degree + 1;
            min..min + $degree
        }};
    }

    // TODO test and comment
    macro_rules! parent {
        ($i:expr, $degree:expr) => {
            ($i - 1) / $degree
        };
    }

    struct DAryHeap<T> {
        degree: usize,
        array: Vec<T>,
    }

    impl<T> DAryHeap<T> {
        pub fn new(degree: usize, initial_array: Vec<T>) -> Self {
            assert!(degree > 0, "degree must be greater than 0");
            DAryHeap::build_heap(Self {
                degree,
                array: initial_array,
            })
        }

        fn build_heap(self) -> Self {
            self
        }

        fn heapfy(&mut self, i: usize) {
            let degree = self.degree;
            // TODO continue the implementation of the heapfy...
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        mod test_macro_children_range {
            #[test]
            fn should_generate_range_for_unary_tree() {
                let degree = 1;
                assert_eq!(children_range!(0, degree), 1..2);
                assert_eq!(children_range!(1, degree), 2..3);
                assert_eq!(children_range!(135, degree), 136..137);
            }

            #[test]
            fn should_generate_range_for_binary_tree() {
                let degree = 2;
                assert_eq!(children_range!(0, degree), 1..3);
                assert_eq!(children_range!(1, degree), 3..5);
                assert_eq!(children_range!(2, degree), 5..7);
                assert_eq!(children_range!(135, degree), 271..273);
            }
            #[test]
            fn should_generate_range_for_ternary_tree() {
                let degree = 3;
                assert_eq!(children_range!(0, degree), 1..4);
                assert_eq!(children_range!(1, degree), 4..7);
                assert_eq!(children_range!(2, degree), 7..10);
                assert_eq!(children_range!(135, degree), 406..409);
            }
            #[test]
            fn should_generate_range_for_10_ary_tree() {
                let degree = 10;
                assert_eq!(children_range!(0, degree), 1..11);
                assert_eq!(children_range!(1, degree), 11..21);
                assert_eq!(children_range!(2, degree), 21..31);
                assert_eq!(children_range!(135, degree), 1351..1361);
            }
        }
    }
}

use heap::Heap;

fn main() {
    let heap = Heap::build_heap(vec![10, 4, 8, 3, 4, 6, 7, 1, 2, 1, -5, 0, 2, 1, 0]);
    println!("{heap}");
    let sorted_array = heap.heapsort();
    println!("{sorted_array}");
}
