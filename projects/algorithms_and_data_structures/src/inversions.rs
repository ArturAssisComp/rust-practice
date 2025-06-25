#[derive(Debug, Clone, Copy)]
pub enum Order {
    Increasing,
    Decreasing,
}
impl Order {
    fn has_inversion<T: PartialOrd>(&self) -> impl Fn(T, T) -> bool {
        match self {
            Order::Increasing => |first, second| first > second,
            Order::Decreasing => |first, second| first < second,
        }
    }
}

pub fn calculate_inversions<T: Copy + PartialOrd>(arr: &[T], order: Order) -> usize {
    if arr.is_empty() {
        return 0;
    }
    let mut arr_copy = arr.to_vec();
    let mut aux_arr = vec![arr[0]; arr.len()];
    merge_sort_to_calculate_inversions(&mut arr_copy, &mut aux_arr, order)
}

fn merge_sort_to_calculate_inversions<T: PartialOrd + Copy>(
    arr: &mut [T],
    aux_arr: &mut [T],
    order: Order,
) -> usize {
    if arr.len() <= 1 {
        return 0;
    }
    let mid = arr.len() / 2;

    let mut inversions_count =
        merge_sort_to_calculate_inversions(&mut arr[..mid], &mut aux_arr[..mid], order)
            + merge_sort_to_calculate_inversions(&mut arr[mid..], &mut aux_arr[mid..], order);

    // merge both partitions
    let has_inversion = order.has_inversion();
    let mut left = 0;
    let mut right = mid;
    let mut i = 0;
    while left < mid && right < arr.len() {
        if has_inversion(arr[left], arr[right]) {
            inversions_count += mid - left;
            aux_arr[i] = arr[right];
            right += 1;
        } else {
            aux_arr[i] = arr[left];
            left += 1;
        }
        i += 1;
    }
    while left < mid {
        aux_arr[i] = arr[left];
        left += 1;
        i += 1;
    }
    while right < arr.len() {
        aux_arr[i] = arr[right];
        right += 1;
        i += 1;
    }
    for i in 0..arr.len() {
        arr[i] = aux_arr[i];
    }
    inversions_count
}

#[cfg(test)]
mod tests {
    use super::*;
    macro_rules! test_inversions_function {
        ($mod_name:ident, $func:ident) => {
            mod $mod_name {
                use super::*;

                #[test]
                fn should_return_inversions_for_empty_slice() {
                    assert_eq!($func::<u8>(&[], Order::Increasing), 0);
                    assert_eq!($func::<u8>(&[], Order::Decreasing), 0);
                }

                #[test]
                fn should_return_inversions_for_1_element_slice() {
                    assert_eq!($func(&[12], Order::Increasing), 0);
                    assert_eq!($func(&['q'], Order::Decreasing), 0);
                }

                #[test]
                fn should_return_inversions_for_2_element_slice() {
                    assert_eq!($func(&[12, 12], Order::Increasing), 0);
                    assert_eq!($func(&[1, 12], Order::Increasing), 0);
                    assert_eq!($func(&[1, 12], Order::Decreasing), 1);
                    assert_eq!($func(&[10, -12], Order::Increasing), 1);
                    assert_eq!($func(&[1, -112], Order::Decreasing), 0);
                }

                #[test]
                fn should_return_inversions_for_3_element_slice() {
                    assert_eq!($func(&[1, 2, 3], Order::Increasing), 0);
                    assert_eq!($func(&[3, 2, 1], Order::Increasing), 3);
                    assert_eq!($func(&[1, 3, 2], Order::Increasing), 1);
                    assert_eq!($func(&[2, 1, 3], Order::Increasing), 1);
                    assert_eq!($func(&[1, 2, 3], Order::Decreasing), 3);
                    assert_eq!($func(&[3, 2, 1], Order::Decreasing), 0);
                }

                #[test]
                fn should_handle_duplicate_elements() {
                    assert_eq!($func(&[5, 5, 5, 5], Order::Increasing), 0);
                    assert_eq!($func(&[5, 5, 5, 5], Order::Decreasing), 0);
                    assert_eq!($func(&[1, 3, 3, 2], Order::Increasing), 2);
                    assert_eq!($func(&[4, 2, 2, 1], Order::Decreasing), 0);
                    assert_eq!($func(&[1, 2, 2, 4], Order::Decreasing), 5);
                }

                #[test]
                fn should_handle_negative_numbers() {
                    assert_eq!($func(&[-3, -1, -2], Order::Increasing), 1);
                    assert_eq!($func(&[-1, -5, -3], Order::Increasing), 2);
                    assert_eq!($func(&[-5, -3, -1], Order::Decreasing), 3);
                    assert_eq!($func(&[5, -2, 10, -8], Order::Increasing), 4);
                }

                #[test]
                fn should_handle_larger_arrays() {
                    assert_eq!($func(&[5, 4, 3, 2, 1], Order::Increasing), 10);
                    assert_eq!($func(&[1, 2, 3, 4, 5], Order::Decreasing), 10);
                    assert_eq!($func(&[1, 3, 2, 4, 6, 5], Order::Increasing), 2);
                    assert_eq!($func(&[6, 5, 4, 3, 2, 1, 0], Order::Increasing), 21);
                }

                #[test]
                fn should_handle_mixed_ordering() {
                    assert_eq!($func(&[2, 1, 4, 3, 6, 5], Order::Increasing), 3);
                    assert_eq!($func(&[10, 5, 15, 3, 20, 1], Order::Increasing), 9);
                    assert_eq!($func(&[1, 10, 2, 9, 3, 8], Order::Decreasing), 9);
                }

                #[test]
                fn should_handle_single_inversion_patterns() {
                    assert_eq!($func(&[1, 2, 3, 0], Order::Increasing), 3);
                    assert_eq!($func(&[0, 3, 2, 1], Order::Decreasing), 3);
                    assert_eq!($func(&[5, 1, 2, 3, 4], Order::Increasing), 4);
                }
            }
        };
    }

    test_inversions_function!(test_calculate_inversions, calculate_inversions);
}
