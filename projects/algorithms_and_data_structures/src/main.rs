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
        ($v:ident, $i1:expr, $i2:expr) => {{
            let tmp = $v[$i1];
            $v[$i1] = $v[$i2];
            $v[$i2] = tmp;
        }};
    }

    mod sealed {
        pub trait HeapState {}
        pub struct Sorted;
        pub struct NotSorted;
        impl HeapState for Sorted {}
        impl HeapState for NotSorted {}
    }
    use sealed::{HeapState, NotSorted, Sorted};

    pub struct Heap<T: Default + PartialOrd + Copy + Display, S: HeapState> {
        state: PhantomData<S>,
        // Our heap starts from the index: 1 to make the left operation faster.
        array: Vec<T>,
    }

    impl<T: Default + PartialOrd + Copy + Display> Display for Heap<T, NotSorted> {
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
    }

    impl<T: Default + PartialOrd + Copy + Display> Heap<T, NotSorted> {
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
            let preffix = Heap::<T, NotSorted>::build_preffix(level, has_vertical_bar_arr);
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
        fn test_not_sorted_display() {
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
