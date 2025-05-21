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

pub struct DAryHeap<T> {
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

    pub fn replace(&mut self, i: usize, new_value: T) -> Result<(), &'static str> {
        if i >= self.array.len() {
            return Err("i for replacement is out of range");
        }
        if new_value == self.array[i] {
            return Ok(());
        }

        if new_value < self.array[i] {
            self.array[i] = new_value;
            self.heapfy(i);
            return Ok(());
        }

        if i == 0 {
            self.array[i] = new_value;
            return Ok(());
        }

        let mut i = i;
        let mut parent = parent!(i, self.degree);

        while new_value > self.array[parent] {
            self.array[i] = self.array[parent];
            i = parent;
            if i == 0 {
                break;
            }
            parent = parent!(i, self.degree);
        }
        self.array[i] = new_value;
        Ok(())
    }

    pub fn insert(&mut self, key: T) {
        self.array.push(key);

        let mut i = self.array.len() - 1;
        if i == 0 {
            return;
        }
        let mut parent = parent!(i, self.degree);
        while key > self.array[parent] {
            self.array[i] = self.array[parent];
            i = parent;
            if i == 0 {
                break;
            }
            parent = parent!(i, self.degree);
        }
        self.array[i] = key;
    }

    pub fn extract_max(&mut self) -> Option<T> {
        let len = self.array.len();
        if len == 0 {
            return None;
        }
        exchange!(self.array, 0, len - 1);
        let v = self.array.pop();
        if self.array.len() > 1 {
            self.heapfy(0);
        }
        v
    }

    fn build_heap(mut self) -> Self {
        if self.array.is_empty() {
            return self;
        }
        // handle the case of degree = 1
        let mut i = (self.array.len() - 1) / self.degree;
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

    mod test_replace {
        use super::*;

        #[test]
        fn should_return_error_msg_index_out_of_range() {
            let mut heap: DAryHeap<u8> = DAryHeap::new(3, vec![]);

            assert!(heap.replace(0, 90).is_err());
            assert!(heap.replace(1, 90).is_err());
            assert!(heap.replace(4, 90).is_err());

            let mut heap: DAryHeap<u8> = DAryHeap::new(2, vec![1, 2, 3, 4, 5, 6]);

            assert!(heap.replace(3, 90).is_ok());
            assert!(heap.replace(6, 90).is_err());
            assert!(heap.replace(7, 90).is_err());
        }

        #[test]
        fn should_replace_elements_from_3_ary_heap() {
            let mut heap = DAryHeap::new(3, vec!['a', 'b', 'b', 't', 'i']);
            assert_eq!(heap.array, vec!['t', 'i', 'b', 'a', 'b']);

            let _ = heap.replace(4, 'c');
            assert_eq!(heap.array, vec!['t', 'i', 'b', 'a', 'c']);
            let _ = heap.replace(1, 'a');
            assert_eq!(heap.array, vec!['t', 'c', 'b', 'a', 'a']);
            let _ = heap.replace(4, 'z');
            assert_eq!(heap.array, vec!['z', 't', 'b', 'a', 'c']);
        }
    }

    mod test_insert {
        use super::*;

        #[test]
        fn should_insert_5_elements_degree_1() {
            // Insert the following elements: 0, 4, 3, 1, 7
            let mut heap = DAryHeap::new(1, vec![]);

            assert_eq!(heap.array, vec![]);
            heap.insert(0);
            assert_eq!(heap.array, vec![0]);
            heap.insert(4);
            assert_eq!(heap.array, vec![4, 0]);
            heap.insert(3);
            assert_eq!(heap.array, vec![4, 3, 0]);
            heap.insert(1);
            assert_eq!(heap.array, vec![4, 3, 1, 0]);
            heap.insert(7);
            assert_eq!(heap.array, vec![7, 4, 3, 1, 0]);
        }

        #[test]
        fn should_insert_5_elements_degree_2() {
            // Insert the following elements: 0, 4, 3, 1, 7
            let mut heap = DAryHeap::new(2, vec![]);

            assert_eq!(heap.array, vec![]);
            heap.insert(0);
            assert_eq!(heap.array, vec![0]);
            heap.insert(4);
            assert_eq!(heap.array, vec![4, 0]);
            heap.insert(3);
            assert_eq!(heap.array, vec![4, 0, 3]);
            heap.insert(1);
            assert_eq!(heap.array, vec![4, 1, 3, 0]);
            heap.insert(7);
            assert_eq!(heap.array, vec![7, 4, 3, 0, 1]);
        }
        #[test]
        fn should_insert_5_elements_degree_3() {
            // Insert the following elements: 0, 4, 3, 1, 7
            let mut heap = DAryHeap::new(3, vec![]);

            assert_eq!(heap.array, vec![]);
            heap.insert(0);
            assert_eq!(heap.array, vec![0]);
            heap.insert(4);
            assert_eq!(heap.array, vec![4, 0]);
            heap.insert(3);
            assert_eq!(heap.array, vec![4, 0, 3]);
            heap.insert(1);
            assert_eq!(heap.array, vec![4, 0, 3, 1]);
            heap.insert(7);
            assert_eq!(heap.array, vec![7, 4, 3, 1, 0]);
        }
    }

    mod test_extract_max {
        use super::*;

        #[test]
        fn should_extract_none_from_empty() {
            let mut heap: DAryHeap<u8> = DAryHeap::new(1, vec![]);
            assert!(heap.extract_max().is_none());

            let mut heap: DAryHeap<u8> = DAryHeap::new(2, vec![]);
            assert!(heap.extract_max().is_none());

            let mut heap: DAryHeap<u8> = DAryHeap::new(3, vec![]);
            assert!(heap.extract_max().is_none());

            let mut heap: DAryHeap<u8> = DAryHeap::new(123, vec![]);
            assert!(heap.extract_max().is_none());
        }

        #[test]
        fn should_extract_none_from_1_element_array() {
            let mut heap = DAryHeap::new(1, vec![123]);
            assert_eq!(heap.extract_max(), Some(123));
            assert!(heap.extract_max().is_none());
            assert!(heap.extract_max().is_none());

            let mut heap = DAryHeap::new(2, vec![123]);
            assert_eq!(heap.extract_max(), Some(123));
            assert!(heap.extract_max().is_none());
            assert!(heap.extract_max().is_none());

            let mut heap = DAryHeap::new(1000, vec![123]);
            assert_eq!(heap.extract_max(), Some(123));
            assert!(heap.extract_max().is_none());
            assert!(heap.extract_max().is_none());
        }

        #[test]
        fn should_extract_none_from_2_element_array() {
            let mut heap = DAryHeap::new(2, vec![123, 123]);
            assert_eq!(heap.extract_max(), Some(123));
            assert_eq!(heap.extract_max(), Some(123));
            assert!(heap.extract_max().is_none());
            assert!(heap.extract_max().is_none());

            let mut heap = DAryHeap::new(100, vec![0, -34]);
            assert_eq!(heap.extract_max(), Some(0));
            assert_eq!(heap.extract_max(), Some(-34));
            assert!(heap.extract_max().is_none());
            assert!(heap.extract_max().is_none());
        }

        #[test]
        fn should_extract_none_from_3_element_array() {
            let mut heap = DAryHeap::new(2, vec![1, 2, 3]);
            assert_eq!(heap.extract_max(), Some(3));
            assert_eq!(heap.extract_max(), Some(2));
            assert_eq!(heap.extract_max(), Some(1));
            assert!(heap.extract_max().is_none());
            assert!(heap.extract_max().is_none());

            let mut heap = DAryHeap::new(4, vec![0, -34, 10_000, -28938]);
            assert_eq!(heap.extract_max(), Some(10_000));
            assert_eq!(heap.extract_max(), Some(0));
            assert_eq!(heap.extract_max(), Some(-34));
            assert_eq!(heap.extract_max(), Some(-28938));
            assert!(heap.extract_max().is_none());
            assert!(heap.extract_max().is_none());
        }

        #[test]
        fn should_extract_none_from_10_element_array() {
            let mut heap = DAryHeap::new(4, vec![1, 3, 4, 6, 45, 77, 5, 7, 8, 1]);
            assert_eq!(heap.extract_max(), Some(77));
            assert_eq!(heap.extract_max(), Some(45));
            assert_eq!(heap.extract_max(), Some(8));
            assert_eq!(heap.extract_max(), Some(7));
            assert_eq!(heap.extract_max(), Some(6));
            assert_eq!(heap.extract_max(), Some(5));
            assert_eq!(heap.extract_max(), Some(4));
            assert_eq!(heap.extract_max(), Some(3));
            assert_eq!(heap.extract_max(), Some(1));
            assert_eq!(heap.extract_max(), Some(1));
            assert!(heap.extract_max().is_none());
            assert!(heap.extract_max().is_none());
        }
    }

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

            let heap: DAryHeap<u8> = DAryHeap {
                degree: 1,
                array: vec![123],
            };

            let heap = heap.build_heap();

            assert_eq!(heap.array, vec![123]);
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
