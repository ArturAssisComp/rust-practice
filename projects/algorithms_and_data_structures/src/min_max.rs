/// Returns a tuple with respectively the position of the minimum and the maximum
/// values of the slice `arr`. The positions are relative to the input slice.
///
/// # Return
/// If there are multiple elements that are equal, the first is returned both for
/// max or for min or both.
pub fn min_max<T: PartialOrd + Copy>(arr: &[T]) -> Option<(usize, usize)> {
    if arr.is_empty() {
        return None;
    }
    let len = arr.len();
    let len_half = len / 2;
    let mut max_arr = Vec::with_capacity(len_half + 1);
    let mut min_arr = Vec::with_capacity(len_half + 1);
    for i in 0..len_half {
        let i1 = i << 1;
        let i2 = i1 + 1;

        if arr[i1] > arr[i2] {
            max_arr.push(i1);
            min_arr.push(i2);
            continue;
        }
        if arr[i1] < arr[i2] {
            max_arr.push(i2);
            min_arr.push(i1);
            continue;
        }
        max_arr.push(i1);
        min_arr.push(i1);
    }
    if len % 2 == 1 {
        let last = len - 1;
        max_arr.push(last);
        min_arr.push(last);
    }
    let mut i_of_max = max_arr[0];
    let mut i_of_min = min_arr[0];

    // find the max
    let mut reference_value = arr[i_of_max];
    let mut tmp_value;
    for &i in max_arr[1..].iter() {
        tmp_value = arr[i];
        if tmp_value > reference_value {
            reference_value = tmp_value;
            i_of_max = i;
        }
    }

    reference_value = arr[i_of_min];
    for &i in min_arr[1..].iter() {
        tmp_value = arr[i];
        if tmp_value < reference_value {
            reference_value = tmp_value;
            i_of_min = i;
        }
    }

    Some((i_of_min, i_of_max))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_return_none_for_empty_slice() {
        assert_eq!(min_max::<u8>(&[]), None);
    }

    #[test]
    fn should_return_the_position_for_the_same_element_for_1_len_array() {
        assert_eq!(min_max(&[12]), Some((0, 0)));
    }

    #[test]
    fn should_return_the_min_max_positions_for_2_len_array() {
        assert_eq!(min_max(&[1, 2]), Some((0, 1)));
        assert_eq!(min_max(&['i', 'z']), Some((0, 1)));
        assert_eq!(min_max(&[1000, 2]), Some((1, 0)));
        assert_eq!(min_max(&[1000, 1000]), Some((0, 0)));
    }

    #[test]
    fn should_work_with_three_elements() {
        assert_eq!(min_max(&[2, 1, 3]), Some((1, 2)));
        assert_eq!(min_max(&[3, 1, 2]), Some((1, 0)));
        assert_eq!(min_max(&[1, 3, 2]), Some((0, 1)));
        assert_eq!(min_max(&['a', 'a', 'z']), Some((0, 2)));
    }

    #[test]
    fn should_handle_multiple_elements_with_same_min_value() {
        assert_eq!(min_max(&[1, 2, 1, 3]), Some((0, 3))); // First occurrence of min
        assert_eq!(min_max(&[5, 1, 3, 1, 4]), Some((1, 0))); // First min, first max
    }

    #[test]
    fn should_handle_multiple_elements_with_same_max_value() {
        assert_eq!(min_max(&[1, 5, 3, 5, 2]), Some((0, 1))); // First occurrence of max
        assert_eq!(min_max(&[3, 3, 1, 3]), Some((2, 0))); // First max, only min
    }

    #[test]
    fn should_handle_all_elements_same() {
        assert_eq!(min_max(&[7, 7, 7, 7]), Some((0, 0)));
        assert_eq!(min_max(&['a', 'a', 'a']), Some((0, 0)));
    }

    #[test]
    fn should_work_with_negative_numbers() {
        assert_eq!(min_max(&[-1, -5, -2, -10, -3]), Some((3, 0)));
        assert_eq!(min_max(&[-1, 0, -2]), Some((2, 1)));
        assert_eq!(min_max(&[10, -5, 15, -20]), Some((3, 2)));
    }

    #[test]
    fn should_work_with_larger_arrays() {
        assert_eq!(min_max(&[5, 2, 8, 1, 9, 3, 7, 4, 6]), Some((3, 4)));
        let large_vec: Vec<i32> = (0..100).collect();
        assert_eq!(min_max(&large_vec), Some((0, 99)));
    }

    #[test]
    fn should_work_with_descending_order() {
        assert_eq!(min_max(&[10, 8, 6, 4, 2]), Some((4, 0)));
        assert_eq!(min_max(&['z', 'y', 'x', 'w']), Some((3, 0)));
    }

    #[test]
    fn should_work_with_ascending_order() {
        assert_eq!(min_max(&[1, 3, 5, 7, 9]), Some((0, 4)));
        assert_eq!(min_max(&['a', 'b', 'c', 'd']), Some((0, 3)));
    }

    #[test]
    fn should_work_with_other_than_integer_data_types() {
        assert_eq!(min_max(&[1.5, 2.7, 0.3, 4.1]), Some((2, 3)));
        assert_eq!(min_max(&["zebra", "apple", "banana"]), Some((1, 0)));
    }

    #[test]
    fn should_handle_min_max_at_edges() {
        assert_eq!(min_max(&[1, 5, 3, 2]), Some((0, 1))); // Min first, max second
        assert_eq!(min_max(&[5, 3, 2, 1]), Some((3, 0))); // Min last, max first
        assert_eq!(min_max(&[3, 1, 5, 2]), Some((1, 2))); // Min and max in middle
    }
}
