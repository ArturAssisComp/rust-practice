const MAX_LEVELS: usize = 100;

mod heap {
    use crate::MAX_LEVELS;
    use std::fmt::Display;
    use std::marker::PhantomData;

    macro_rules! parent {
        ($i:expr) => {
            $i >> 1
        };
    }

    macro_rules! left {
        ($i:expr) => {
            $i << 1
        };
    }

    macro_rules! right {
        ($i:expr) => {
            ($i << 1) + 1
        };
    }

    /// Exchange the elements of index `i1` and `i2` from mutable slice `v`.
    macro_rules! exchange {
        ($v:expr, $i1:expr, $i2:expr) => {{
            let tmp = $v[$i1];
            $v[$i1] = $v[$i2];
            $v[$i2] = tmp;
        }};
    }

    mod sealed {
        pub trait HeapState {}
        pub struct Sorted;
        pub struct PriorityQueue;
        impl HeapState for Sorted {}
        impl HeapState for PriorityQueue {}
    }
    use sealed::{HeapState, PriorityQueue, Sorted};

    pub struct Heap<T: Default + PartialOrd + Copy + Display, S: HeapState> {
        state: PhantomData<S>,
        // Our heap starts from the index: 1 to make the left operation faster.
        array: Vec<T>,
    }

    impl<T: Default + PartialOrd + Copy + Display> Display for Heap<T, PriorityQueue> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            self.recursive_fmt(f, 1, &mut [false; MAX_LEVELS])
        }
    }

    impl<T: Default + PartialOrd + Copy + Display> Display for Heap<T, Sorted> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "[")?;
            if self.array.len() > 1 {
                write!(f, "{}", self.array[1])?;
                for e in self.array[2..].iter() {
                    write!(f, ", {}", e)?;
                }
            }
            write!(f, "]")
        }
    }

    impl<T: Default + PartialOrd + Copy + Display> Heap<T, Sorted> {
        fn is_sorted(&self) -> bool {
            self.array[1..].is_sorted()
        }

        fn priority_queue(self) -> Heap<T, PriorityQueue> {
            let len = self.array.len();
            let mut v = self.array;
            exchange!(v, 0, len - 1);
            // remove the dummy element
            v.pop();
            Heap::<T, PriorityQueue>::build_heap(v)
        }
    }

    impl<T: Default + PartialOrd + Copy + Display> Heap<T, PriorityQueue> {
        /// Prints the current part of the tree.
        ///
        /// # Arguments
        /// - `index`: the index of the element of the heap
        /// - `vertical_bar_levels`: a reference to the vector indicating the levels in
        /// which the vertical line `│` will be printed.
        ///
        /// Special chars:
        /// ├─
        /// │
        /// └─
        /// # Example
        /// - E0: [3]
        /// 3
        ///
        /// - E1: [3, 2, 1]
        /// 3
        /// ├─1
        /// └─2
        ///
        /// - E2: [e, d, c, b, a]
        /// e
        /// ├─c
        /// └─d
        ///   ├─a
        ///   └─b
        ///
        /// - E3: [10, 8, 4, 4, 3, 2]
        /// 10
        /// ├─4
        /// │ └─2
        /// └─8
        ///   ├─3
        ///   └─4
        fn recursive_fmt(
            &self,
            f: &mut std::fmt::Formatter<'_>,
            index: usize,
            has_vertical_bar_arr: &mut [bool; MAX_LEVELS],
        ) -> std::fmt::Result {
            let level = index.ilog2() as usize;
            let parent = self.array[index];
            write!(f, "{parent}")?;
            let l = left!(index);
            if l > self.size() {
                return Ok(());
            }
            let preffix = Heap::<T, PriorityQueue>::build_preffix(level, has_vertical_bar_arr);
            let r = right!(index);
            if r <= self.size() {
                // ├─
                write!(f, "\n{preffix}├─")?;

                has_vertical_bar_arr[level] = true;
                self.recursive_fmt(f, r, has_vertical_bar_arr)?;
                has_vertical_bar_arr[level] = false;
            }
            // └─2
            write!(f, "\n{preffix}└─")?;
            self.recursive_fmt(f, l, has_vertical_bar_arr)
        }

        fn max(&self) -> Option<T> {
            self.array.get(1).copied()
        }

        /// Removes the max element from the priority queue.
        ///
        /// # Complexity
        /// - Time: O(log(n))
        /// - Space: O(1)
        fn extract_max(&mut self) -> Option<T> {
            let size = self.size();
            if size == 0 {
                return None;
            }
            exchange!(self.array, 1, size);
            let max = self.array.pop();
            Heap::heapfy(&mut self.array, 1);
            max
        }

        /// Replace the current value of the `i`-th element of the heap with `new_value`
        /// if the latter is greater than the former.
        ///
        /// # Caveats
        /// - `i` must be within the range `[1, self.size()]`
        fn replace_if_greater(&mut self, i: usize, new_value: T) -> Result<(), &'static str> {
            if i == 0 || i > self.size() {
                return Err("i must be in range [1, self.size()]");
            }
            if new_value <= self.array[i] {
                return Ok(());
            }
            let mut i = i;
            let mut parent = parent!(i);

            while i > 1 && self.array[parent] < new_value {
                self.array[i] = self.array[parent];
                i = parent;
                parent = parent!(i);
            }
            self.array[i] = new_value;
            Ok(())
        }

        /// Adds a new element to the priority queue.
        ///
        /// # Complexity
        /// - Time: O(log(n))
        /// - Space: O(1)
        fn insert(&mut self, key: T) {
            // add to the tail of the array
            self.array.push(key);

            let mut i = self.size();
            if i == 1 {
                return;
            }
            let mut parent = parent!(i);
            while i > 1 && self.array[parent] < key {
                self.array[i] = self.array[parent];
                i = parent;
                parent = parent!(i);
            }
            self.array[i] = key;
        }

        fn build_preffix(level: usize, has_vertical_bar_arr: &mut [bool; MAX_LEVELS]) -> String {
            let str_builder = &mut vec![];
            let space = "  ";
            let vertical_bar = "│ ";
            for i in 0..level {
                let v = if has_vertical_bar_arr[i] {
                    vertical_bar
                } else {
                    space
                };
                str_builder.push(v);
            }

            str_builder.join("")
        }

        /// This method creates a new heap from `initial_array`. The heap is a data
        /// structure which has the property that each parent has a higher value than its
        /// two children.
        ///
        /// # Complexity
        /// - Time: O(initial_array.len())
        /// - Space: O(1)
        pub fn build_heap(initial_array: Vec<T>) -> Self {
            let mut array = vec![T::default()];
            let mut i = initial_array.len() >> 1;
            array.extend(initial_array);
            while i >= 1 {
                Heap::heapfy(&mut array, i);
                i -= 1;
            }

            Self {
                array,
                state: PhantomData,
            }
        }

        /// Sorts this heap.
        ///
        /// # Complexity
        /// - Time: O(n.log(n))
        /// - Space: O(1)
        ///
        /// # Caveats
        /// After sorting, the heap may not be used anymore as priority
        /// queue.
        pub fn heapsort(mut self) -> Heap<T, Sorted> {
            let mut length = self.size();
            // Remember: self.array = [dummy, e1, e2, ..., eLength]
            let v = &mut self.array[0..length + 1];
            while length >= 2 {
                exchange!(v, 1, length);
                length -= 1;
                // Obs: check the contract of heapfy
                Heap::heapfy(&mut v[..length + 1], 1);
            }
            Heap {
                state: PhantomData::<Sorted>,
                array: self.array,
            }
        }

        fn size(&self) -> usize {
            self.array.len() - 1
        }

        /// This utility is used to restore the heap property of heap subtree v[i..] given
        /// that the subtrees: left(i) and right(i) is already heap.
        ///
        /// # Complexity
        /// - Time: O(log(v.len()))
        /// - Size: O(1)
        ///
        /// # Contract
        /// - C1: It is expected that the element with index 0 is a dummy element. The
        /// actual values of the heap will start from v[1..].
        /// - C2: i <= v.len() - 1
        fn heapfy(v: &mut [T], i: usize) {
            let v_size = v.len() - 1;
            let mut i = i;
            loop {
                let l = left!(i);
                let mut index_to_max = i;
                if l > v_size {
                    return;
                }
                if v[l] > v[index_to_max] {
                    index_to_max = l;
                }
                let r = right!(i);
                if r <= v_size && v[r] > v[index_to_max] {
                    index_to_max = r;
                }

                if i == index_to_max {
                    return;
                }
                exchange!(v, i, index_to_max);
                i = index_to_max;
            }
        }

        /// This utility is used to restore the heap property of heap subtree v[i..] given
        /// that the subtrees: left(i) and right(i) is already heap.
        ///
        /// # Complexity
        /// - Time: O(log(v.len()))
        /// - Size: O(1)
        ///
        /// # Contract
        /// - C1: It is expected that the element with index 0 is a dummy element. The
        /// actual values of the heap will start from v[1..].
        /// - C2: i < v.len() - 1
        ///
        /// # Caveats
        /// - This is the recursive version for the heapfy.
        fn recursive_heapfy(v: &mut Vec<T>, i: usize) {
            let v_size = v.len() - 1;
            let l = left!(i);
            let mut index_to_max = i;
            if l > v_size {
                return;
            }
            if v[l] > v[index_to_max] {
                index_to_max = l;
            }
            let r = right!(i);
            if r <= v_size && v[r] > v[index_to_max] {
                index_to_max = r;
            }

            if i != index_to_max {
                exchange!(v, i, index_to_max);
                Heap::recursive_heapfy(v, index_to_max);
            }
        }

        /// Utility function that checks if the input array is a heap or not.
        /// In order to maintain the heap property, an array A must have, for any i,
        /// A[parent!(i)] >= A[i]
        ///
        /// # Contract
        /// - C1: It is expected that the element with index 0 is a dummy element. The
        /// actual values of the heap will start from v[1..].
        /// - C2: Although the B is `PartialEq`, it is expected only vectors of elements
        /// that are comparable. That allows this function to be applied to floating
        /// points, for example.
        fn is_heap(v: &Vec<T>) -> bool {
            for (i, &current) in v.iter().skip(1).enumerate() {
                let i = i + 1; // fix the ignored 0
                match v.get(left!(i)) {
                    Some(&left) => {
                        if current < left {
                            return false;
                        }
                    }
                    None => {
                        return true;
                    }
                }
                match v.get(right!(i)) {
                    Some(&right) => {
                        if current < right {
                            return false;
                        }
                    }
                    None => {
                        return true;
                    }
                }
            }
            true
        }
    }

    #[cfg(test)]
    mod test {
        use super::*;

        macro_rules! test_heapfy {
            ($name:ident, $heapfy:ident) => {
                mod $name {
                    use super::*;
                    #[test]
                    fn should_heapfy_one_element_vec() {
                        let v = &mut vec![i32::default(), 12];

                        Heap::$heapfy(v, 1);
                        assert_eq!(*v, vec![i32::default(), 12]);
                    }

                    #[test]
                    fn should_heapfy_two_element_vec() {
                        // c1: not heap - i:1
                        let v = &mut vec![u8::default(), 1, 2];

                        Heap::$heapfy(v, 1);
                        assert_eq!(*v, vec![u8::default(), 2, 1]);

                        // c2: heap - i:1
                        let v = &mut vec![u8::default(), 2, 1];

                        Heap::$heapfy(v, 1);
                        assert_eq!(*v, vec![u8::default(), 2, 1]);

                        // c3: heap - i:2
                        let v = &mut vec![u8::default(), 2, 1];

                        Heap::$heapfy(v, 2);
                        assert_eq!(*v, vec![u8::default(), 2, 1]);
                    }

                    #[test]
                    fn should_heapfy_three_element_vec() {
                        // c1: not heap - i:1
                        let v = &mut vec![u8::default(), 1, 2, 3];

                        Heap::$heapfy(v, 1);
                        assert_eq!(*v, vec![u8::default(), 3, 2, 1]);

                        // c2: heap - i:1
                        let v = &mut vec![u8::default(), 3, 2, 1];

                        Heap::$heapfy(v, 1);
                        assert_eq!(*v, vec![u8::default(), 3, 2, 1]);

                        // c3: not-heap - i:1
                        let v = &mut vec![char::default(), 'a', 'z', 'z'];

                        Heap::$heapfy(v, 1);
                        assert_eq!(*v, vec![char::default(), 'z', 'a', 'z']);
                    }

                    #[test]
                    fn should_heapfy_complex_element_vec() {
                        let v = &mut vec![
                            f64::default(),
                            30.4,
                            30.0,
                            5.7,
                            -3.8, // 4 --> start heapfy from here
                            10.0,
                            4.8,
                            3.0,
                            20.4, // 8 --> first exchange
                            3.75,
                            2.0,
                            5.0,
                            4.8,
                            4.8,
                            2.0,
                            1.0,
                            -3.0,
                            6.5, // 17 --> last exchange
                            4.3,
                            2.1,
                        ];

                        Heap::$heapfy(v, 4);
                        assert_eq!(
                            *v,
                            vec![
                                f64::default(),
                                30.4,
                                30.0,
                                5.7,
                                20.4, // 4
                                10.0,
                                4.8,
                                3.0,
                                6.5, // 8
                                3.75,
                                2.0,
                                5.0,
                                4.8,
                                4.8,
                                2.0,
                                1.0,
                                -3.0,
                                -3.8, // 17
                                4.3,
                                2.1,
                            ]
                        );
                    }
                }
            };
        }

        test_heapfy!(test_recursive_heapfy, recursive_heapfy);
        test_heapfy!(test_heapfy, heapfy);

        mod test_replace_if_greater {
            use super::*;

            #[test]
            fn should_return_error_when_replacing_out_of_bounds() {
                assert!(Heap::build_heap(vec![]).replace_if_greater(1, 12).is_err());
                assert!(Heap::build_heap(vec![]).replace_if_greater(0, 12).is_err());
                assert!(Heap::build_heap(vec![1, 2, 3])
                    .replace_if_greater(0, 100)
                    .is_err());
                assert!(Heap::build_heap(vec![1, 2, 3])
                    .replace_if_greater(4, 100)
                    .is_err());
            }

            #[test]
            fn should_replace_size_1_array_element() {
                let heap = &mut Heap::build_heap(vec![10]);
                assert!(heap.replace_if_greater(1, 12).is_ok());
                let dummy = i32::default();
                assert_eq!(heap.array, vec![dummy, 12]);

                assert!(heap.replace_if_greater(1, 12).is_ok());
                assert_eq!(heap.array, vec![dummy, 12]);

                assert!(heap.replace_if_greater(1, -12).is_ok());
                assert_eq!(heap.array, vec![dummy, 12]);
            }

            #[test]
            fn should_replace_size_2_array_element() {
                let heap = &mut Heap::build_heap(vec![1, 9]);
                assert!(heap.replace_if_greater(2, 2).is_ok());
                let dummy = i32::default();
                assert_eq!(heap.array, vec![dummy, 9, 2]);

                assert!(heap.replace_if_greater(2, 3).is_ok());
                assert_eq!(heap.array, vec![dummy, 9, 3]);

                assert!(heap.replace_if_greater(2, 100).is_ok());
                assert_eq!(heap.array, vec![dummy, 100, 9]);

                assert!(heap.replace_if_greater(1, 101).is_ok());
                assert_eq!(heap.array, vec![dummy, 101, 9]);
            }

            #[test]
            fn should_replace_size_5_array_element() {
                let heap = &mut Heap::build_heap(vec![12.3, 8.4, 6.7, 8.4, -23.0]);
                assert!(heap.replace_if_greater(4, 23.0).is_ok());
                let dummy = f64::default();
                assert_eq!(heap.array, vec![dummy, 23.0, 12.3, 6.7, 8.4, -23.0]);

                assert!(heap.replace_if_greater(1, 23.009).is_ok());
                assert_eq!(heap.array, vec![dummy, 23.009, 12.3, 6.7, 8.4, -23.0]);
            }
        }
        mod test_insert {
            use super::*;
            use std::u8;

            #[test]
            fn should_insert_8_elements_into_an_empty_vec() {
                let heap = &mut Heap::<u8, _>::build_heap(vec![]);
                let dummy = u8::default();
                assert_eq!(heap.array, vec![dummy]);
                heap.insert(0);
                assert_eq!(heap.array, vec![dummy, 0]);
                heap.insert(2);
                assert_eq!(heap.array, vec![dummy, 2, 0]);
                heap.insert(4);
                assert_eq!(heap.array, vec![dummy, 4, 0, 2]);
                heap.insert(3);
                assert_eq!(heap.array, vec![dummy, 4, 3, 2, 0]);
                heap.insert(1);
                assert_eq!(heap.array, vec![dummy, 4, 3, 2, 0, 1]);
                heap.insert(6);
                assert_eq!(heap.array, vec![dummy, 6, 3, 4, 0, 1, 2]);
                heap.insert(5);
                assert_eq!(heap.array, vec![dummy, 6, 3, 5, 0, 1, 2, 4]);
                heap.insert(100);
                assert_eq!(heap.array, vec![dummy, 100, 6, 5, 3, 1, 2, 4, 0]);
            }

            #[test]
            fn should_insert_4_repeated_elements_into_an_empty_vec() {
                let heap = &mut Heap::<u8, _>::build_heap(vec![]);
                let dummy = u8::default();
                assert_eq!(heap.array, vec![dummy]);
                heap.insert(2);
                assert_eq!(heap.array, vec![dummy, 2]);
                heap.insert(2);
                assert_eq!(heap.array, vec![dummy, 2, 2]);
                heap.insert(2);
                assert_eq!(heap.array, vec![dummy, 2, 2, 2]);
                heap.insert(2);
                assert_eq!(heap.array, vec![dummy, 2, 2, 2, 2]);
            }

            #[test]
            fn should_insert_2_element_into_a_vec() {
                let heap = &mut Heap::<u8, _>::build_heap(vec![1, 2, 3]);
                let dummy = u8::default();
                assert_eq!(heap.array, vec![dummy, 3, 2, 1]);
                heap.insert(2);
                assert_eq!(heap.array, vec![dummy, 3, 2, 1, 2]);
                heap.insert(8);
                assert_eq!(heap.array, vec![dummy, 8, 3, 1, 2, 2]);
            }
        }

        mod test_extract_max {
            use super::*;
            #[test]
            fn should_extract_empty() {
                let heap = &mut Heap::<u8, _>::build_heap(vec![]);
                assert_eq!(heap.extract_max(), None);
                assert_eq!(heap.extract_max(), None);
            }

            #[test]
            fn should_extract_one_element() {
                let heap = &mut Heap::build_heap(vec!['9']);
                assert_eq!(heap.extract_max(), Some('9'));
                assert_eq!(heap.extract_max(), None);
                assert_eq!(heap.extract_max(), None);
            }
            #[test]
            fn should_extract_two_elements() {
                let heap = &mut Heap::build_heap(vec![1, 2]);
                assert_eq!(heap.extract_max(), Some(2));
                assert_eq!(heap.extract_max(), Some(1));
                assert_eq!(heap.extract_max(), None);
                assert_eq!(heap.extract_max(), None);
            }
            #[test]
            fn should_extract_three_elements() {
                let heap = &mut Heap::build_heap(vec![43, 3847, -234]);
                assert_eq!(heap.extract_max(), Some(3847));
                assert_eq!(heap.extract_max(), Some(43));
                assert_eq!(heap.extract_max(), Some(-234));
                assert_eq!(heap.extract_max(), None);
                assert_eq!(heap.extract_max(), None);
            }
            #[test]
            fn should_extract_10_elements() {
                let heap = &mut Heap::build_heap(vec![9, 4, 2, 3, 4, 6, 7, 8, 0, -2]);
                assert_eq!(heap.extract_max(), Some(9));
                assert_eq!(heap.extract_max(), Some(8));
                assert_eq!(heap.extract_max(), Some(7));
                assert_eq!(heap.extract_max(), Some(6));
                assert_eq!(heap.extract_max(), Some(4));
                assert_eq!(heap.extract_max(), Some(4));
                assert_eq!(heap.extract_max(), Some(3));
                assert_eq!(heap.extract_max(), Some(2));
                assert_eq!(heap.extract_max(), Some(0));
                assert_eq!(heap.extract_max(), Some(-2));
                assert_eq!(heap.extract_max(), None);
                assert_eq!(heap.extract_max(), None);
            }
            #[test]
            fn should_extract_4_repeated_elements() {
                let heap = &mut Heap::build_heap(Vec::from(["Hello"; 4]));
                assert_eq!(heap.extract_max(), Some("Hello"));
                assert_eq!(heap.extract_max(), Some("Hello"));
                assert_eq!(heap.extract_max(), Some("Hello"));
                assert_eq!(heap.extract_max(), Some("Hello"));
                assert_eq!(heap.extract_max(), None);
                assert_eq!(heap.extract_max(), None);
            }
            #[test]
            fn should_extract_100_elements() {
                let size = 100;
                let heap = &mut Heap::build_heap((1..=size).collect());
                for i in (1..=size).rev() {
                    assert_eq!(heap.extract_max(), Some(i));
                }
                assert_eq!(heap.extract_max(), None);
                assert_eq!(heap.extract_max(), None);
            }
        }

        #[test]
        fn test_is_sorted() {
            // empty
            assert!(Heap::<char, _>::build_heap(vec![]).heapsort().is_sorted());
            // 1 element
            assert!(Heap::build_heap(vec!['a']).heapsort().is_sorted());
            assert!(Heap::build_heap(vec![12]).heapsort().is_sorted());
            // 2 elements
            assert!(Heap::build_heap(vec![1, 1]).heapsort().is_sorted());
            assert!(Heap::build_heap(vec![-1, 2]).heapsort().is_sorted());
            assert!(Heap::build_heap(vec![2, -1]).heapsort().is_sorted());
            // 3 elements
            assert!(Heap::build_heap(vec![1, 2, 3]).heapsort().is_sorted());
            assert!(Heap::build_heap(vec![1, 3, 2]).heapsort().is_sorted());
            assert!(Heap::build_heap(vec![2, 1, 3]).heapsort().is_sorted());
            assert!(Heap::build_heap(vec![2, 3, 1]).heapsort().is_sorted());
            assert!(Heap::build_heap(vec![3, 1, 2]).heapsort().is_sorted());
            assert!(Heap::build_heap(vec![3, 2, 1]).heapsort().is_sorted());

            // 6 elements
            assert!(Heap::build_heap(vec![2, -41, 23, -412, 34, -1234])
                .heapsort()
                .is_sorted());
        }

        #[test]
        fn test_sorted_display() {
            // empty
            assert_eq!(
                Heap::<char, _>::build_heap(vec![]).heapsort().to_string(),
                "[]"
            );
            // 1 element
            assert_eq!(Heap::build_heap(vec!['a']).heapsort().to_string(), "[a]");
            assert_eq!(Heap::build_heap(vec![12]).heapsort().to_string(), "[12]");
            // 2 elements
            assert_eq!(
                Heap::build_heap(vec![1, 1]).heapsort().to_string(),
                "[1, 1]"
            );
            assert_eq!(
                Heap::build_heap(vec![-1, 2]).heapsort().to_string(),
                "[-1, 2]"
            );
            assert_eq!(
                Heap::build_heap(vec![2, -1]).heapsort().to_string(),
                "[-1, 2]"
            );
            // 3 elements
            assert_eq!(
                Heap::build_heap(vec![1, 2, 3]).heapsort().to_string(),
                "[1, 2, 3]"
            );
            assert_eq!(
                Heap::build_heap(vec![1, 3, 2]).heapsort().to_string(),
                "[1, 2, 3]"
            );
            assert_eq!(
                Heap::build_heap(vec![2, 1, 3]).heapsort().to_string(),
                "[1, 2, 3]"
            );
            assert_eq!(
                Heap::build_heap(vec![2, 3, 1]).heapsort().to_string(),
                "[1, 2, 3]"
            );
            assert_eq!(
                Heap::build_heap(vec![3, 1, 2]).heapsort().to_string(),
                "[1, 2, 3]"
            );
            assert_eq!(
                Heap::build_heap(vec![3, 2, 1]).heapsort().to_string(),
                "[1, 2, 3]"
            );

            // 6 elements
            assert_eq!(
                Heap::build_heap(vec![2, -41, 23, -412, 34, -1234])
                    .heapsort()
                    .to_string(),
                "[-1234, -412, -41, 2, 23, 34]"
            );
        }

        #[test]
        fn test_priority_queue_display() {
            //
            assert_eq!(Heap::build_heap(vec![10]).to_string(), "10");
            assert_eq!(
                Heap::build_heap(vec![2, 3]).to_string(),
                "\
3
└─2\
                "
            );
            assert_eq!(
                Heap::build_heap(vec![1, 2, 3]).to_string(),
                "\
3
├─1
└─2\
                "
            );
            assert_eq!(
                Heap::build_heap(vec!['e', 'd', 'c', 'b', 'a']).to_string(),
                "\
e
├─c
└─d
  ├─a
  └─b\
                "
            );

            assert_eq!(
                Heap::build_heap(vec![10, 8, 4, 4, 3, 2]).to_string(),
                "\
10
├─4
│ └─2
└─8
  ├─3
  └─4\
                "
            );

            assert_eq!(
                Heap::build_heap(vec![10, 8, 4, 4, 3, 2]).to_string(),
                "\
10
├─4
│ └─2
└─8
  ├─3
  └─4\
                "
            );
            assert_eq!(
                Heap::build_heap(vec![10, 4, 8, 3, 4, 6, 7, 1, 2, 1, -5, 0, 2, 1, 0]).to_string(),
                "\
10
├─8
│ ├─7
│ │ ├─0
│ │ └─1
│ └─6
│   ├─2
│   └─0
└─4
  ├─4
  │ ├─-5
  │ └─1
  └─3
    ├─2
    └─1\
    "
            );
        }

        #[test]
        fn test_is_heap() {
            assert!(Heap::<u8, _>::is_heap(&vec![u8::default()]));
            assert!(Heap::is_heap(&vec![char::default(), 'a']));
            assert!(Heap::is_heap(&vec![char::default(), 'c', 'b', 'a']));
            assert!(!Heap::is_heap(&vec!["", "a", "b", "a"]));
            assert!(Heap::is_heap(&vec![
                i32::default(),
                9,
                8,
                7,
                6,
                5,
                4,
                3,
                2,
                1
            ]));
            assert!(!Heap::is_heap(&vec![
                i32::default(),
                10,
                9,
                8,
                7,
                6,
                5,
                4,
                3,
                2,
                7
            ]));
            assert!(!Heap::is_heap(&vec![
                i32::default(),
                23,
                17,
                14,
                6,
                13,
                10,
                1,
                5,
                7,
                12
            ]));
        }

        #[test]
        fn test_build_heap() {
            // Empty heap
            assert!(Heap::is_heap(&Heap::<u8, _>::build_heap(vec![]).array));

            // Single element
            assert!(Heap::is_heap(&Heap::<u8, _>::build_heap(vec![1]).array));

            // Two elements
            assert!(Heap::is_heap(&Heap::<u8, _>::build_heap(vec![1, 2]).array));
            assert!(Heap::is_heap(&Heap::<u8, _>::build_heap(vec![2, 1]).array));

            // Three elements
            assert!(Heap::is_heap(
                &Heap::<u8, _>::build_heap(vec![2, 1, 45]).array
            ));
            assert!(Heap::is_heap(
                &Heap::<u8, _>::build_heap(vec![1, 2, 3]).array
            ));
            assert!(Heap::is_heap(
                &Heap::<u8, _>::build_heap(vec![3, 2, 1]).array
            ));

            // Duplicate elements
            assert!(Heap::is_heap(
                &Heap::<u8, _>::build_heap(vec![1, 1, 1]).array
            ));
            assert!(Heap::is_heap(
                &Heap::<u8, _>::build_heap(vec![2, 1, 2, 1, 2]).array
            ));
            assert!(Heap::is_heap(
                &Heap::<u8, _>::build_heap(vec![5, 5, 3, 3, 1, 1]).array
            ));

            // Already a valid max heap
            assert!(Heap::is_heap(
                &Heap::<u8, _>::build_heap(vec![10, 8, 9, 4, 5, 6, 7]).array
            ));

            // Already a valid min heap (should be converted to max heap)
            assert!(Heap::is_heap(
                &Heap::<u8, _>::build_heap(vec![1, 2, 3, 4, 5, 6, 7]).array
            ));

            // Reverse sorted array
            assert!(Heap::is_heap(
                &Heap::<u8, _>::build_heap(vec![7, 6, 5, 4, 3, 2, 1]).array
            ));

            // Random unordered array
            assert!(Heap::is_heap(
                &Heap::<u8, _>::build_heap(vec![3, 7, 1, 9, 2, 8, 4, 6, 5]).array
            ));

            // Edge values for u8
            assert!(Heap::is_heap(&Heap::<u8, _>::build_heap(vec![0]).array));
            assert!(Heap::is_heap(&Heap::<u8, _>::build_heap(vec![255]).array));
            assert!(Heap::is_heap(
                &Heap::<u8, _>::build_heap(vec![0, 255]).array
            ));
            assert!(Heap::is_heap(
                &Heap::<u8, _>::build_heap(vec![255, 0]).array
            ));
            assert!(Heap::is_heap(
                &Heap::<u8, _>::build_heap(vec![0, 127, 255]).array
            ));

            // Large heap with repeated pattern
            assert!(Heap::is_heap(
                &Heap::<u8, _>::build_heap(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]).array
            ));

            // Power of 2 sized arrays (complete binary trees)
            assert!(Heap::is_heap(
                &Heap::<u8, _>::build_heap(vec![5, 4, 3, 2, 1, 0, 9, 8]).array
            )); // 8 elements
            assert!(Heap::is_heap(
                &Heap::<u8, _>::build_heap(vec![
                    1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16
                ])
                .array
            )); // 16 elements

            // Nearly complete binary tree
            assert!(Heap::is_heap(
                &Heap::<u8, _>::build_heap(vec![5, 4, 3, 2, 1, 0, 9]).array
            )); // 7 elements
            assert!(Heap::is_heap(
                &Heap::<u8, _>::build_heap(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]).array
            )); // 9 elements

            // Alternating high-low pattern
            assert!(Heap::is_heap(
                &Heap::<u8, _>::build_heap(vec![10, 1, 9, 2, 8, 3, 7, 4, 6, 5]).array
            ));

            // All elements except one are the same
            assert!(Heap::is_heap(
                &Heap::<u8, _>::build_heap(vec![5, 5, 5, 5, 5, 5, 5, 10, 5]).array
            ));
            assert!(Heap::is_heap(
                &Heap::<u8, _>::build_heap(vec![5, 5, 5, 5, 1, 5, 5, 5, 5]).array
            ));
        }
        #[test]
        fn test_build_heap_with_different_types() {
            // Test with i32
            assert!(Heap::is_heap(
                &Heap::<i32, _>::build_heap(vec![-1, 0, 1]).array
            ));
            assert!(Heap::is_heap(
                &Heap::<i32, _>::build_heap(vec![i32::MIN, 0, i32::MAX]).array
            ));

            // Test with f64
            assert!(Heap::is_heap(
                &Heap::<i32, _>::build_heap(vec![1, 2, 3]).array
            ));

            // Test with char
            assert!(Heap::is_heap(
                &Heap::<char, _>::build_heap(vec!['a', 'b', 'c']).array
            ));
            assert!(Heap::is_heap(
                &Heap::<char, _>::build_heap(vec!['z', 'a', 'm']).array
            ));
        }

        #[test]
        fn test_build_heap_maintains_size() {
            let original = vec![3, 7, 1, 9, 2, 8, 4, 6, 5];
            let heap = Heap::<u8, _>::build_heap(original.clone());
            assert_eq!(heap.array.len(), original.len() + 1);

            // Verify all elements are still present (heap property doesn't lose elements)
            let mut sorted_original = original;
            sorted_original.push(u8::default()); // add the dummy element
            let mut sorted_heap = heap.array.clone();
            sorted_original.sort();
            sorted_heap.sort();
            assert_eq!(sorted_original, sorted_heap);
        }

        mod test_macros {
            use super::*;

            #[test]
            fn should_match_the_correct_parent() {
                assert_eq!(parent!(2), 1);
                assert_eq!(parent!(3), 1);
                assert_eq!(parent!(4), 2);
                assert_eq!(parent!(5), 2);
                assert_eq!(parent!(6), 3);
                assert_eq!(parent!(12340), 6170);
                assert_eq!(parent!(12341), 6170);
                assert_eq!(parent!(12342), 6171);
                assert_eq!(parent!(12342), 6171);
            }

            #[test]
            fn should_match_the_correct_left() {
                assert_eq!(left!(1), 2);
                assert_eq!(left!(2), 4);
                assert_eq!(left!(3), 6);
                assert_eq!(left!(4), 8);
                assert_eq!(left!(5), 10);
                assert_eq!(left!(6170), 12340);
                assert_eq!(left!(6171), 12342);
            }

            #[test]
            fn should_match_the_correct_right() {
                assert_eq!(right!(1), 3);
                assert_eq!(right!(2), 5);
                assert_eq!(right!(3), 7);
                assert_eq!(right!(4), 9);
                assert_eq!(right!(5), 11);
                assert_eq!(right!(6170), 12341);
                assert_eq!(right!(6171), 12343);
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
