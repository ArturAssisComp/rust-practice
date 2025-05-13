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

    let mut heap = Heap::build_heap(initial_array);
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
