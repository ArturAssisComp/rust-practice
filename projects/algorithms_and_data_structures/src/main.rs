const MAX_LEVELS: usize = 100;

mod heap {
    use crate::MAX_LEVELS;
    use std::marker::PhantomData;
    use std::{fmt::Display, str};

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

    // states of the heap
    pub struct Sorted;
    pub struct PriorityQueue;

    pub struct Heap<T, S> {
        state: PhantomData<S>,
        // Our heap starts from the index: 1 to make the left operation faster.
        array: Vec<T>,
    }

    impl<T> Display for Heap<T, PriorityQueue>
    where
        T: Default + PartialOrd + Copy + Display,
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            self.recursive_fmt(f, 1, &mut [false; MAX_LEVELS])
        }
    }

    impl<T> Display for Heap<T, Sorted>
    where
        T: Default + PartialOrd + Copy + Display,
    {
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

    impl<T> Heap<T, Sorted>
    where
        T: Default + PartialOrd + Copy + Display,
    {
        fn is_sorted(&self) -> bool {
            self.array[1..].is_sorted()
        }

        pub fn priority_queue(self) -> Heap<T, PriorityQueue> {
            let len = self.array.len();
            let mut v = self.array;
            exchange!(v, 0, len - 1);
            // remove the dummy element
            v.pop();
            Heap::<T, PriorityQueue>::build_heap(v)
        }
    }

    impl<T> Heap<T, PriorityQueue>
    where
        T: Default + PartialOrd + Copy + Display,
    {
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

        pub fn max(&self) -> Option<T> {
            self.array.get(1).copied()
        }

        /// Removes the max element from the priority queue.
        ///
        /// # Complexity
        /// - Time: O(log(n))
        /// - Space: O(1)
        pub fn extract_max(&mut self) -> Option<T> {
            let size = self.size();
            if size == 0 {
                return None;
            }
            exchange!(self.array, 1, size);
            let max = self.array.pop();
            Heap::heapfy(&mut self.array, 1);
            max
        }

        /// Deletes the `i`-th element from this heap and return its value.
        ///
        /// # Error
        /// If `i` is out of range, an error is returned.
        pub fn delete(&mut self, i: usize) -> Result<T, &'static str> {
            let size = self.size();
            if i == 0 || i > size {
                return Err("'i' is out of range. I must be in the range [1, self.size()]");
            }
            if i != size {
                exchange!(self.array, i, size);
            }
            let deleted_value = self
                .array
                .pop()
                .expect("at this point, it is guaranteed that self.array has at least 2 element");

            Heap::heapfy(&mut self.array, i);

            Ok(deleted_value)
        }

        /// Replace the heap element of index `i` with `new_value`. If `i` is out of index
        /// bounds, it returns an error.
        pub fn replace(&mut self, i: usize, new_value: T) -> Result<(), &'static str> {
            if i == 0 || i > self.size() {
                return Err("i must be in range [1, self.size()]");
            }
            if new_value < self.array[i] {
                self.array[i] = new_value;
                Heap::heapfy(&mut self.array[..], i);
                return Ok(());
            }
            // replace if greater
            if new_value == self.array[i] {
                return Ok(());
            }

            if self.size() == 1 {
                self.array[i] = new_value;
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

        /// Replace the current value of the `i`-th element of the heap with `new_value`
        /// if the latter is greater than the former.
        ///
        /// # Caveats
        /// - `i` must be within the range `[1, self.size()]`
        pub fn replace_if_greater(&mut self, i: usize, new_value: T) -> Result<(), &'static str> {
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
        pub fn insert(&mut self, key: T) {
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

        pub fn size(&self) -> usize {
            self.array.len() - 1
        }

        /// This utility is used to restore the heap property of heap subtree `v[i..]`
        /// given that the subtrees: left(i) and right(i) is already heap.
        ///
        /// # Complexity
        /// - Time: O(log(v.len()))
        /// - Size: O(1)
        ///
        /// # Contract
        /// - C1: It is expected that the element with index 0 is a dummy element. The
        /// actual values of the heap will start from `v[1..]`.
        /// - C2: `i <= v.len() - 1`
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

        mod test_delete {
            use super::*;

            #[test]
            fn should_return_error_when_i_out_of_range() {
                assert!(Heap::<u8, _>::build_heap(vec![]).delete(0).is_err());
                assert!(Heap::<u8, _>::build_heap(vec![]).delete(1).is_err());
                assert!(Heap::<u8, _>::build_heap(vec![]).delete(5).is_err());

                assert!(Heap::build_heap(vec![12]).delete(2).is_err());
                assert!(Heap::build_heap(vec![1, 2, 3]).delete(4).is_err());
                assert!(Heap::build_heap(vec![1, 2, 3]).delete(0).is_err());
            }

            #[test]
            fn should_delete_element_from_size_1_vec() {
                let heap = &mut Heap::build_heap(vec![120]);

                assert_eq!(heap.delete(1), Ok(120));
                let dummy = i32::default();

                assert_eq!(heap.array, vec![dummy]);
            }
            #[test]
            fn should_delete_element_from_size_2_vec() {
                let dummy = char::default();
                let heap1 = &mut Heap::build_heap(vec!['b', 'a']);

                assert_eq!(heap1.delete(1), Ok('b'));

                assert_eq!(heap1.array, vec![dummy, 'a']);

                assert_eq!(heap1.delete(1), Ok('a'));

                assert_eq!(heap1.array, vec![dummy]);

                let heap2 = &mut Heap::build_heap(vec!['b', 'a']);

                assert_eq!(heap2.delete(2), Ok('a'));

                assert_eq!(heap2.array, vec![dummy, 'b']);
            }

            #[test]
            fn should_delete_element_from_size_5_vec() {
                let dummy = i32::default();
                let heap = &mut Heap::build_heap(vec![55, 44, 43, 43, -23]);

                assert_eq!(heap.delete(1), Ok(55));

                assert_eq!(heap.array, vec![dummy, 44, 43, 43, -23]);

                assert_eq!(heap.delete(2), Ok(43));
                assert_eq!(heap.array, vec![dummy, 44, -23, 43]);

                assert_eq!(heap.delete(1), Ok(44));
                assert_eq!(heap.array, vec![dummy, 43, -23]);

                assert_eq!(heap.delete(2), Ok(-23));
                assert_eq!(heap.array, vec![dummy, 43]);

                assert_eq!(heap.delete(1), Ok(43));
                assert_eq!(heap.array, vec![dummy]);
            }
        }

        mod test_replace {
            use super::*;

            #[test]
            fn should_return_error_when_replacing_out_of_bounds() {
                assert!(Heap::build_heap(vec![]).replace(1, 12).is_err());
                assert!(Heap::build_heap(vec![]).replace(0, 12).is_err());
                assert!(Heap::build_heap(vec![1, 2, 3]).replace(0, 100).is_err());
                assert!(Heap::build_heap(vec![1, 2, 3]).replace(4, 100).is_err());
            }

            #[test]
            fn should_replace_size_1_array_element() {
                let heap = &mut Heap::build_heap(vec![10]);
                assert!(heap.replace(1, 12).is_ok());
                let dummy = i32::default();
                assert_eq!(heap.array, vec![dummy, 12]);

                assert!(heap.replace(1, 12).is_ok());
                assert_eq!(heap.array, vec![dummy, 12]);

                assert!(heap.replace(1, -12).is_ok());
                assert_eq!(heap.array, vec![dummy, -12]);
            }

            #[test]
            fn should_replace_size_2_array_element() {
                let heap = &mut Heap::build_heap(vec![1, 9]);
                assert!(heap.replace(2, 2).is_ok());
                let dummy = i32::default();
                assert_eq!(heap.array, vec![dummy, 9, 2]);

                assert!(heap.replace(2, 3).is_ok());
                assert_eq!(heap.array, vec![dummy, 9, 3]);

                assert!(heap.replace(2, 100).is_ok());
                assert_eq!(heap.array, vec![dummy, 100, 9]);

                assert!(heap.replace(1, 101).is_ok());
                assert_eq!(heap.array, vec![dummy, 101, 9]);

                assert!(heap.replace(1, -1000).is_ok());
                assert_eq!(heap.array, vec![dummy, 9, -1000]);
            }

            #[test]
            fn should_replace_size_5_array_element() {
                let heap = &mut Heap::build_heap(vec![12.3, 8.4, 6.7, 8.4, -23.0]);
                assert!(heap.replace(4, 23.0).is_ok());
                let dummy = f64::default();
                assert_eq!(heap.array, vec![dummy, 23.0, 12.3, 6.7, 8.4, -23.0]);

                assert!(heap.replace(1, 23.009).is_ok());
                assert_eq!(heap.array, vec![dummy, 23.009, 12.3, 6.7, 8.4, -23.0]);
            }
        }
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

mod merge_sorted_list {
    use crate::heap::Heap;
    use std::fmt::Display;

    #[derive(Debug, PartialEq, Clone, Copy)]
    struct IndexValue<T>(usize, T);

    impl<T: Display> Display for IndexValue<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "({}, {})", self.0, self.1)
        }
    }

    impl<T> Default for IndexValue<T>
    where
        T: Default,
    {
        fn default() -> Self {
            Self(usize::default(), T::default())
        }
    }

    impl<T> PartialOrd for IndexValue<T>
    where
        T: PartialOrd,
    {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            self.1.partial_cmp(&other.1)
        }
    }

    /// Merge the `sorted_lists` into one sorted list. Each element is copied from each of
    /// the sorted lists.
    ///
    /// # Contract
    /// - Each element of `sorted_list` must be an ascending sorted list.
    fn merge_sorted<T: PartialOrd + Copy + Default + Display>(sorted_lists: Vec<Vec<T>>) -> Vec<T> {
        let num_of_lists = sorted_lists.len();
        let indices = &mut vec![Some(0); num_of_lists];

        // Build heap
        let mut initial_array = vec![];

        for (i, list) in sorted_lists.iter().enumerate() {
            if !list.is_empty() {
                let last_index = list.len() - 1;
                initial_array.push(IndexValue(i, list[last_index]));
                indices[i] = last_index.checked_sub(1);
            }
        }

        let heap = &mut Heap::build_heap(initial_array);
        let mut merged_vec = vec![];

        while heap.size() >= 1 {
            let IndexValue(list_index, value) = heap.max().expect("heap.size() is greater than 0");
            match indices[list_index] {
                Some(index_in_list) => {
                    heap.replace(
                        1,
                        IndexValue(list_index, sorted_lists[list_index][index_in_list]),
                    )
                    .expect("the list is expected to have at least 1 element");
                    indices[list_index] = index_in_list.checked_sub(1);
                }
                None => {
                    heap.extract_max();
                }
            }
            merged_vec.push(value);
        }
        merged_vec.reverse();
        merged_vec
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn should_merge_empty_vectors() {
            assert_eq!(merge_sorted::<u8>(vec![vec![]]), vec![]);
            assert_eq!(merge_sorted::<u8>(vec![vec![], vec![]]), vec![]);
        }

        #[test]
        fn should_merge_1_element_vectors() {
            assert_eq!(merge_sorted(vec![vec![12]]), vec![12]);
            assert_eq!(merge_sorted(vec![vec!['1'], vec!['1']]), vec!['1', '1']);
            assert_eq!(
                merge_sorted(vec![vec![-2], vec![4], vec![1], vec![19]]),
                vec![-2, 1, 4, 19]
            );
        }

        #[test]
        fn should_merge_2_element_vectors() {
            assert_eq!(merge_sorted(vec![vec![7, 12], vec![23]]), vec![7, 12, 23]);
            assert_eq!(
                merge_sorted(vec![vec![-2, 9], vec![4], vec![1, 1], vec![-20, 19]]),
                vec![-20, -2, 1, 1, 4, 9, 19]
            );
        }

        #[test]
        fn should_handle_mixed_length_vectors() {
            assert_eq!(
                merge_sorted(vec![vec![1, 3, 5], vec![2, 4], vec![6]]),
                vec![1, 2, 3, 4, 5, 6]
            );
            assert_eq!(
                merge_sorted(vec![vec![1, 5, 9], vec![], vec![3, 7]]),
                vec![1, 3, 5, 7, 9]
            );
        }

        #[test]
        fn should_handle_duplicate_elements() {
            assert_eq!(
                merge_sorted(vec![vec![1, 3, 3], vec![2, 3], vec![1, 6]]),
                vec![1, 1, 2, 3, 3, 3, 6]
            );
            assert_eq!(
                merge_sorted(vec![vec![5, 5, 5], vec![5, 5], vec![5]]),
                vec![5, 5, 5, 5, 5, 5]
            );
        }

        #[test]
        fn should_handle_negative_numbers() {
            assert_eq!(
                merge_sorted(vec![vec![-5, -3, 0], vec![-4, -2, 1], vec![-6, 2]]),
                vec![-6, -5, -4, -3, -2, 0, 1, 2]
            );
        }

        #[test]
        fn should_handle_large_vectors() {
            let v1: Vec<i32> = (0..100).map(|x| x * 2).collect(); // [0, 2, 4, ..., 198]
            let v2: Vec<i32> = (0..100).map(|x| x * 2 + 1).collect(); // [1, 3, 5, ..., 199]
            let expected: Vec<i32> = (0..200).collect(); // [0, 1, 2, ..., 199]
            assert_eq!(merge_sorted(vec![v1, v2]), expected);
        }

        #[test]
        fn should_handle_non_integer_types() {
            assert_eq!(
                merge_sorted(vec![
                    vec!["apple", "orange"],
                    vec!["banana", "pear"],
                    vec!["grape"]
                ]),
                vec!["apple", "banana", "grape", "orange", "pear"]
            );

            assert_eq!(
                merge_sorted(vec![vec![1.1, 3.3], vec![2.2, 4.4]]),
                vec![1.1, 2.2, 3.3, 4.4]
            );
        }

        #[test]
        fn should_handle_empty_input() {
            // Empty vector of vectors
            let empty: Vec<Vec<i32>> = vec![];
            assert_eq!(merge_sorted(empty), vec![]);
        }

        #[test]
        fn should_handle_edge_cases() {
            // One empty vector and one non-empty vector
            assert_eq!(merge_sorted(vec![vec![], vec![1, 2, 3]]), vec![1, 2, 3]);

            // Multiple empty vectors and one non-empty vector
            assert_eq!(
                merge_sorted(vec![vec![], vec![], vec![1, 2, 3], vec![]]),
                vec![1, 2, 3]
            );

            // Vector with a single element that is i32::MIN and i32::MAX
            assert_eq!(
                merge_sorted(vec![vec![i32::MIN], vec![i32::MAX]]),
                vec![i32::MIN, i32::MAX]
            );
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
