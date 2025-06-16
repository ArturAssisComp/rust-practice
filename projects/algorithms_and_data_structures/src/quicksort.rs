use rand::{rngs::ThreadRng, Rng};
use std::fmt::Debug;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Order {
    Increasing,
    Decreasing,
}

impl Order {
    const fn get_is_not_sorted<T: PartialOrd>(&self) -> impl Fn(T, T) -> bool {
        match self {
            Order::Increasing => |first, second| first > second,
            Order::Decreasing => |first, second| first < second,
        }
    }
    const fn get_left_cmp<T: PartialOrd>(&self) -> impl Fn(T, T) -> bool {
        match self {
            Order::Increasing => |first, second| first < second,
            Order::Decreasing => |first, second| first > second,
        }
    }
    const fn get_right_cmp<T: PartialOrd>(&self) -> impl Fn(T, T) -> bool {
        match self {
            Order::Increasing => |first, second| first > second,
            Order::Decreasing => |first, second| first < second,
        }
    }
}

macro_rules! exchange {
    ($arr:expr, $i:expr, $j:expr) => {{
        let tmp = $arr[$i];
        $arr[$i] = $arr[$j];
        $arr[$j] = tmp;
    }};
}

fn first_element_partition<T: PartialOrd + Copy>(
    arr: &mut [T],
    mut left: usize,
    mut right: usize,
    order: Order,
) -> usize {
    let pivot = arr[left];
    let left_cmp = order.get_left_cmp::<T>();
    let right_cmp = order.get_right_cmp::<T>();

    while right_cmp(arr[right], pivot) {
        right -= 1;
    }

    while left < right {
        exchange!(arr, left, right);
        left += 1;
        right -= 1;

        while left_cmp(arr[left], pivot) {
            left += 1;
        }
        while right_cmp(arr[right], pivot) {
            right -= 1;
        }
    }
    right + 1
}

/// [  a1     a2 ... an]
///    ^                 ^
///    |                 |
/// <start>            <end>
///
/// # Contract
/// - `end` <= arr.len()
/// - `start` < arr.len()
pub fn quicksort_ineficient<T: PartialOrd + Copy>(
    arr: &mut [T],
    start: usize,
    end: usize,
    order: Order,
) {
    if start + 1 >= end {
        return;
    }

    let left = start;
    let right = end - 1;
    let q = first_element_partition(arr, left, right, order);

    quicksort_ineficient(arr, start, q, order);

    quicksort_ineficient(arr, q, end, order);
}

/// [  a1     a2 ... an]
///    ^                 ^
///    |                 |
/// <start>            <end>
///
/// # Contract
/// - `end` <= arr.len()
/// - `start` < arr.len()
pub fn quicksort_ineficient_random_partition<T: PartialOrd + Copy>(
    arr: &mut [T],
    start: usize,
    end: usize,
    order: Order,
) {
    if start + 1 >= end {
        return;
    }

    let left = start;
    let right = end - 1;
    let mut rng = rand::rng();
    let random = median_of_3(&mut rng, arr, left, right);
    exchange!(arr, start, random);
    let q = first_element_partition(arr, left, right, order);

    quicksort_ineficient_random_partition(arr, start, q, order);
    quicksort_ineficient_random_partition(arr, q, end, order);
}

/// [  a1     a2 ... an]
///    ^                 ^
///    |                 |
/// <start>            <end>
///
/// # Contract
/// - `end` <= arr.len()
/// - `start` < arr.len()
pub fn quicksort_efficient_random_partition<T>(
    arr: &mut [T],
    start: usize,
    end: usize,
    order: Order,
) where
    T: PartialOrd + Copy + Debug,
{
    if start + 1 >= end {
        return;
    }

    let mut left = start;
    let mut right = end - 1;
    let mut left_len;
    let mut right_len;
    let mut rng = rand::rng();

    while left < right {
        let random = median_of_3(&mut rng, arr, left, right);
        exchange!(arr, left, random);
        let q = first_element_partition(arr, left, right, order);
        left_len = q - left;
        right_len = right + 1 - q;

        if left_len <= right_len {
            quicksort_efficient_random_partition(arr, left, q, order);
            left = q;
        } else {
            quicksort_efficient_random_partition(arr, q, right + 1, order);
            right = q - 1;
        }
    }
}

/// [  a1     a2 ... an]
///    ^                 ^
///    |                 |
/// <start>            <end>
///
///
/// # Contract
/// - `end` <= arr.len()
/// - `start` < arr.len()
pub fn quicksort<T: PartialOrd + Copy>(arr: &mut [T], start: usize, end: usize, order: Order) {
    const INSERTION_SORT_FACTOR: usize = 100;
    //const INSERTION_SORT_FACTOR: usize = 1;
    quicksort_efficient(arr, start, end, INSERTION_SORT_FACTOR, order);
    insertion_sort(arr, start, end, order);
}

fn median_of_3<T: PartialOrd + Copy>(
    rng: &mut ThreadRng,
    arr: &mut [T],
    left: usize,
    right: usize,
) -> usize {
    let i1 = rng.random_range(left..right + 1);
    let i2 = rng.random_range(left..right + 1);
    let i3 = rng.random_range(left..right + 1);
    let el1 = arr[i1];
    let el2 = arr[i2];
    let el3 = arr[i3];
    if el1 <= el2 {
        // 1 2
        if el2 <= el3 {
            // 1 2 3
            return i2;
        }
        if el1 <= el3 {
            // 1  3  2
            return i3;
        }
        // 3  1  2
        return i1;
    }
    // 2 1
    if el1 <= el3 {
        // 2 1 3
        return i1;
    }
    if el2 <= el3 {
        // 2 3 1
        return i3;
    }
    // 3 2 1
    i2
}

/// [  a1     a2 ... an]
///    ^                 ^
///    |                 |
/// <start>            <end>
///
/// # Arguments
/// - `insertion_sort_factor`: when `end - start <= insertion_sort_factor`, the algorithm
/// stops calling itself recursively.
///
/// # Contract
/// - `end` <= arr.len()
/// - `start` < arr.len()
fn quicksort_efficient<T: PartialOrd + Copy>(
    arr: &mut [T],
    start: usize,
    end: usize,
    insertion_sort_factor: usize,
    order: Order,
) {
    if start + 1 >= end {
        return;
    }
    let mut left = start;
    let mut right = end - 1;
    let mut left_len;
    let mut right_len;
    let mut rng = rand::rng();
    loop {
        if right - left + 1 <= insertion_sort_factor {
            return;
        }
        let random = median_of_3(&mut rng, arr, left, right);
        exchange!(arr, left, random);
        let q = first_element_partition(arr, left, right, order);

        left_len = q - left;
        right_len = right + 1 - q;

        if left_len <= right_len {
            quicksort_efficient(arr, left, q, insertion_sort_factor, order);
            left = q;
        } else {
            quicksort_efficient(arr, q, right + 1, insertion_sort_factor, order);
            right = q - 1;
        }
    }
}

fn insertion_sort<T: PartialOrd + Copy>(arr: &mut [T], start: usize, end: usize, order: Order) {
    // start < end - 1
    if start + 1 >= end {
        return;
    }
    let is_not_sorted = order.get_is_not_sorted();
    let mut i;
    let mut i_minus_1;
    for last_sorted_index in (start + 1)..end {
        i = last_sorted_index;
        let last_element = arr[i];
        while i > start && is_not_sorted(arr[i - 1], last_element) {
            i_minus_1 = i - 1;
            arr[i] = arr[i_minus_1];
            i = i_minus_1;
        }
        arr[i] = last_element;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::{collections::HashMap, hash::Hash};

    fn get_element_count_hash_map<T>(arr: &[T]) -> HashMap<T, usize>
    where
        T: Copy + Eq + Hash,
    {
        let mut answer = HashMap::new();
        for el in arr {
            match answer.get_mut(el) {
                Some(v) => *v += 1,
                None => {
                    answer.insert(*el, 1_usize);
                }
            }
        }
        answer
    }

    macro_rules! test_sorting_array {
        ($($arr:expr, $quicksort:ident),*) => {
            $({
                let mut arr_asc = $arr.clone();
                let mut arr_dsc = $arr.clone();

                // test ascending order
                let arr_asc_len = arr_asc.len();
                $quicksort(&mut arr_asc, 0, arr_asc_len, Order::Increasing);
                assert!(arr_asc.is_sorted());
                assert_eq!(arr_asc.len() , $arr.len(), "the length is not equal to the original");
                assert_eq!(get_element_count_hash_map(&arr_asc), get_element_count_hash_map(&$arr));

                // test descending order
                let arr_dsc_len = arr_dsc.len();
                $quicksort(&mut arr_dsc, 0, arr_dsc_len, Order::Decreasing);
                let arr_dsc = arr_dsc.into_iter().rev().collect::<Vec<_>>();
                assert!(arr_dsc.is_sorted());
                assert_eq!(arr_dsc.len() , $arr.len(), "the length is not equal to the original");
                assert_eq!(get_element_count_hash_map(&arr_dsc), get_element_count_hash_map(&$arr));

            })*
        };
    }

    macro_rules! test_quicksort {
        ($mod_name:ident, $quicksort:ident) => {
            mod $mod_name {
                use super::*;

                #[test]
                fn should_sort_empty_arr() {
                    let arr: Vec<u8> = Vec::new();
                    test_sorting_array!(&arr, $quicksort);
                }
                #[test]
                fn should_sort_1_element_arr() {
                    let arr = [123];
                    test_sorting_array!(arr, $quicksort);
                }
                #[test]
                fn should_sort_2_element_arr() {
                    let arr = [1, 2];
                    test_sorting_array!(arr, $quicksort);

                    let arr = [1, 1];
                    test_sorting_array!(arr, $quicksort);

                    let arr = ['z', 'a'];
                    test_sorting_array!(arr, $quicksort);
                }

                #[test]
                fn should_sort_3_element_arr() {
                    let arr = [1, 2, -23];
                    test_sorting_array!(arr, $quicksort);
                    let arr = [1, 1, 1];
                    test_sorting_array!(arr, $quicksort);
                    let arr = [3, 1, 2];
                    test_sorting_array!(arr, $quicksort);
                    let arr = [1, 3, 2];
                    test_sorting_array!(arr, $quicksort);
                    let arr = [2, 1, 3];
                    test_sorting_array!(arr, $quicksort);
                    let arr = [2, 3, 1];
                    test_sorting_array!(arr, $quicksort);
                    let arr = [3, 2, 1];
                    test_sorting_array!(arr, $quicksort);
                }
                #[test]
                fn should_sort_already_sorted_arrays() {
                    let arr = [1, 2, 3, 4, 5];
                    test_sorting_array!(arr, $quicksort);
                    let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
                    test_sorting_array!(arr, $quicksort);
                    let arr = ['a', 'b', 'c', 'd', 'e'];
                    test_sorting_array!(arr, $quicksort);
                }

                #[test]
                fn should_sort_reverse_sorted_arrays() {
                    let arr = [5, 4, 3, 2, 1];
                    test_sorting_array!(arr, $quicksort);
                    let arr = [10, 9, 8, 7, 6, 5, 4, 3, 2, 1];
                    test_sorting_array!(arr, $quicksort);
                    let arr = ['z', 'y', 'x', 'w', 'v'];
                    test_sorting_array!(arr, $quicksort);
                }

                #[test]
                fn should_sort_arrays_with_duplicates() {
                    let arr = [1, 3, 2, 3, 1];
                    test_sorting_array!(arr, $quicksort);
                    let arr = [5, 5, 5, 5, 5];
                    test_sorting_array!(arr, $quicksort);
                    let arr = [1, 1, 2, 2, 3, 3];
                    test_sorting_array!(arr, $quicksort);
                    let arr = [7, 3, 7, 1, 3, 7, 1];
                    test_sorting_array!(arr, $quicksort);
                    let arr = ['a', 'b', 'a', 'c', 'b', 'a'];
                    test_sorting_array!(arr, $quicksort);
                }

                #[test]
                fn should_sort_arrays_with_negative_numbers() {
                    let arr = [-1, -5, -3, -2, -4];
                    test_sorting_array!(arr, $quicksort);
                    let arr = [-10, 5, -3, 0, 8, -1];
                    test_sorting_array!(arr, $quicksort);
                    let arr = [-100, -1, -50, -25, -75];
                    test_sorting_array!(arr, $quicksort);
                    let arr = [0, -1, 1, -2, 2];
                    test_sorting_array!(arr, $quicksort);
                }

                #[test]
                fn should_sort_arrays_with_zeros() {
                    let arr = [0, 0, 0, 0];
                    test_sorting_array!(arr, $quicksort);
                    let arr = [1, 0, -1, 0, 2];
                    test_sorting_array!(arr, $quicksort);
                    let arr = [0, 5, 0, 3, 0, 1];
                    test_sorting_array!(arr, $quicksort);
                }
                #[test]
                fn should_sort_larger_arrays() {
                    let arr = [64, 34, 25, 12, 22, 11, 90, 88, 76, 50, 42, 30, 5, 77, 55];
                    test_sorting_array!(arr, $quicksort);
                    let arr = [
                        100, 99, 98, 97, 96, 95, 94, 93, 92, 91, 90, 89, 88, 87, 86, 85, 84, 83,
                        82, 81,
                    ];
                    test_sorting_array!(arr, $quicksort);
                }
                #[test]
                fn should_sort_extreme_values() {
                    let arr = [i32::MAX, i32::MIN, 0, 1, -1];
                    test_sorting_array!(arr, $quicksort);
                    let arr = [i32::MIN, i32::MIN + 1, i32::MAX - 1, i32::MAX];
                    test_sorting_array!(arr, $quicksort);
                }
                #[test]
                fn should_handle_stress_test_cases() {
                    // Array where pivot selection matters
                    let arr = [1, 1, 1, 1, 1, 2, 1, 1, 1, 1];
                    test_sorting_array!(arr, $quicksort);

                    // Worst case for naive pivot selection (first element)
                    let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
                    test_sorting_array!(arr, $quicksort);
                }
            }
        };
    }
    test_quicksort!(test_insertion_sort, insertion_sort);
    test_quicksort!(test_efficient_quicksort, quicksort);
    test_quicksort!(test_quicksort_ineficient, quicksort_ineficient);
    test_quicksort!(
        test_quicksort_ineficient_random_partition,
        quicksort_ineficient_random_partition
    );
    test_quicksort!(
        test_quicksort_efficient_random_partition,
        quicksort_efficient_random_partition
    );
}
