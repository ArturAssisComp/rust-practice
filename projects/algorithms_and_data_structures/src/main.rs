const MAX_LEVELS: usize = 100;

mod heap;

mod merge_sorted_list;

mod macros;

mod d_ary_heap {
    use crate::exchange;

    // macros
    /// Given the current index in the array (from 0 to len()-1), this macro returns the
    /// range for all `degree` children that a d-ary heap element may have.
    ///
    /// # Patterns
    /// - ($i:expr, $degree:expr)=>range
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

    /// Given the current index in the array (from 0 to len()-1), this macro returns the
    /// parent's index that a d-ary heap element may have.
    ///
    /// # Patterns
    /// - ($i:expr, $degree:expr)=>parent's index
    ///
    /// # Example
    /// ```rust
    /// assert_eq!(children_range!(1, 2), 0);
    ///
    /// assert_eq!(children_range!(3, 3), 0);
    /// assert_eq!(children_range!(4, 3), 1);
    /// ```
    macro_rules! parent {
        ($i:expr, $degree:expr) => {
            ($i - 1) / $degree
        };
    }

    struct DAryHeap<T> {
        degree: usize,
        array: Vec<T>,
    }

    impl<T> DAryHeap<T>
    where
        T: Copy + PartialOrd,
    {
        pub fn new(degree: usize, initial_array: Vec<T>) -> Self {
            assert!(degree > 0, "degree must be greater than 0");
            DAryHeap::build_heap(Self {
                degree,
                array: initial_array,
            })
        }

        fn build_heap(mut self) -> Self {
            if self.array.is_empty() {
                return self;
            }
            let mut i = self.array.len() / self.degree;
            loop {
                self.heapfy(i);

                if i == 0 {
                    return self;
                }
                i -= 1;
            }
        }

        /// Given the index `i` on our array
        fn heapfy(&mut self, i: usize) {
            let degree = self.degree;
            assert!(
                i < self.array.len(),
                "heapfy index must be within the array limits"
            );

            let mut parent = i;
            let mut max = parent;
            loop {
                for j in children_range!(parent, degree) {
                    if j >= self.array.len() {
                        break;
                    }
                    if self.array[j] > self.array[max] {
                        max = j;
                    }
                }

                println!("max == {max}");
                println!("parent == {parent}");
                if max == parent {
                    break;
                }
                exchange!(self.array, max, parent);
                parent = max;
                max = parent;
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        mod test_build_heap {
            use super::*;

            #[test]
            fn should_build_heap_from_empty_array() {
                let heap: DAryHeap<u8> = DAryHeap {
                    degree: 3,
                    array: vec![],
                };

                let heap = heap.build_heap();

                assert_eq!(heap.array, vec![]);
            }

            #[test]
            fn should_build_heap_from_1_element_array() {
                let heap: DAryHeap<u8> = DAryHeap {
                    degree: 4,
                    array: vec![34],
                };

                let heap = heap.build_heap();

                assert_eq!(heap.array, vec![34]);
            }

            #[test]
            fn should_build_heap_from_2_element_array() {
                let heap: DAryHeap<u8> = DAryHeap {
                    degree: 4,
                    array: vec![2, 16],
                };

                let heap = heap.build_heap();

                assert_eq!(heap.array, vec![16, 2]);
            }

            #[test]
            fn should_build_heap_from_15_element_array() {
                let heap: DAryHeap<u8> = DAryHeap {
                    degree: 3,
                    array: vec![1, 2, 9, 8, 5, 6, 7, 8, 9, 9, 4, 6, 2, 9, 0],
                };

                let heap = heap.build_heap();

                assert_eq!(
                    heap.array,
                    vec![9, 7, 9, 8, 5, 6, 1, 8, 9, 9, 4, 6, 2, 2, 0]
                );
            }
        }

        mod test_heapfy {
            use super::*;

            #[test]
            fn should_heapfy_an_1_element_array() {
                let mut degree1 = DAryHeap {
                    degree: 1,
                    array: vec![2],
                };
                degree1.heapfy(0);
                assert_eq!(degree1.array, vec![2]);

                let mut degree2 = DAryHeap {
                    degree: 2,
                    array: vec![100],
                };
                degree2.heapfy(0);
                assert_eq!(degree2.array, vec![100]);
            }

            #[test]
            fn should_heapfy_an_2_element_array() {
                let mut degree1 = DAryHeap {
                    degree: 1,
                    array: vec![2, 1],
                };
                degree1.heapfy(0);
                assert_eq!(degree1.array, vec![2, 1]);

                let mut degree1 = DAryHeap {
                    degree: 1,
                    array: vec![1, 2],
                };
                degree1.heapfy(0);
                assert_eq!(degree1.array, vec![2, 1]);

                let mut degree2 = DAryHeap {
                    degree: 2,
                    array: vec![100, 99],
                };
                degree2.heapfy(0);
                assert_eq!(degree2.array, vec![100, 99]);

                let mut degree2 = DAryHeap {
                    degree: 2,
                    array: vec![99, 100],
                };
                degree2.heapfy(0);
                assert_eq!(degree2.array, vec![100, 99]);

                let mut degree4 = DAryHeap {
                    degree: 4,
                    array: vec![99, 100],
                };
                degree4.heapfy(0);
                assert_eq!(degree4.array, vec![100, 99]);
            }

            #[test]
            fn should_heapfy_an_10_element_array() {
                let mut degree1 = DAryHeap {
                    degree: 1,
                    array: vec![1, 10, 9, 8, 7, 6, 5, 4, 3, 2],
                };
                degree1.heapfy(0);
                assert_eq!(degree1.array, vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1]);

                let mut degree3 = DAryHeap {
                    degree: 3,
                    array: vec![10, -4, 5, 5, 6, 8, 9, 9, 6, 2],
                };
                degree3.heapfy(1);
                assert_eq!(degree3.array, vec![10, 9, 5, 5, 6, 8, -4, 9, 6, 2]);

                let mut degree4 = DAryHeap {
                    degree: 4,
                    array: vec![10, -4, 5, 5, 6, 8, 9, 9, 6, 2],
                };
                degree4.heapfy(1);
                assert_eq!(degree4.array, vec![10, 9, 5, 5, 6, 8, -4, 9, 6, 2]);
            }

            #[test]
            fn should_heapfy_complex_element_vec() {
                let mut degree2 = DAryHeap {
                    degree: 2,
                    array: vec![
                        30.4, 30.0, 5.7, -3.8, // 4 --> start heapfy from here
                        10.0, 4.8, 3.0, 20.4, // 8 --> first exchange
                        3.75, 2.0, 5.0, 4.8, 4.8, 2.0, 1.0, -3.0, 6.5, // 17 --> last exchange
                        4.3, 2.1,
                    ],
                };
                degree2.heapfy(3);

                assert_eq!(
                    degree2.array,
                    vec![
                        30.4, 30.0, 5.7, 20.4, // 4
                        10.0, 4.8, 3.0, 6.5, // 8
                        3.75, 2.0, 5.0, 4.8, 4.8, 2.0, 1.0, -3.0, -3.8, // 17
                        4.3, 2.1,
                    ]
                );
            }
        }

        mod test_macro_parent {
            #[test]
            fn should_return_parent_index_for_degree_1() {
                let degree = 1;
                assert_eq!(parent!(1, degree), 0);
                assert_eq!(parent!(2, degree), 1);
                assert_eq!(parent!(3, degree), 2);
                assert_eq!(parent!(4, degree), 3);
            }

            #[test]
            fn should_return_parent_index_for_degree_2() {
                let degree = 2;
                assert_eq!(parent!(1, degree), 0);
                assert_eq!(parent!(2, degree), 0);
                assert_eq!(parent!(3, degree), 1);
                assert_eq!(parent!(4, degree), 1);
                assert_eq!(parent!(5, degree), 2);
            }

            #[test]
            fn should_return_parent_index_for_degree_3() {
                let degree = 3;
                assert_eq!(parent!(1, degree), 0);
                assert_eq!(parent!(2, degree), 0);
                assert_eq!(parent!(3, degree), 0);
                assert_eq!(parent!(4, degree), 1);
                assert_eq!(parent!(5, degree), 1);
                assert_eq!(parent!(7, degree), 2);
            }

            #[test]
            fn should_return_parent_index_for_degree_10() {
                let degree = 10;
                assert_eq!(parent!(1, degree), 0);
                assert_eq!(parent!(2, degree), 0);
                assert_eq!(parent!(3, degree), 0);
                assert_eq!(parent!(4, degree), 0);
                assert_eq!(parent!(10, degree), 0);
                assert_eq!(parent!(11, degree), 1);
                assert_eq!(parent!(20, degree), 1);
                assert_eq!(parent!(21, degree), 2);
            }
        }

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
