mod palindrome {
    use std::{cmp::min, ops::Range};

    /// Indicates the max wing size that a palindrome started with mid `(left0, right0)`
    /// can achieve within an array of size `length`.
    ///
    /// If the mid is a central element (odd number of elements):
    /// - `m+wing m+2 m+1 m m+1 m+2 ... m+wing`
    /// If the mid is not a central element (even number of elements):
    /// - `m+wing m+2 m+1 m+1 m+2 ... m+wing`
    ///
    /// # Contracts
    /// - `left0` and `right0` must be within [0, length - 1];
    /// - `right0 - left0` must be in {0, 1}
    fn max_wing_size(length: usize, left0: usize, right0: usize) -> usize {
        let ans = min(left0, length - 1 - right0);
        if left0 != right0 {
            return ans + 1;
        }
        ans
    }

    /// This function tries to find the maximum length of a palindrome that can be formed
    /// from `left0 and right0`. It returns the range and the size of the range.
    ///
    /// # Contracts
    /// - `left0` and `right0` must be within [0, length - 1];
    /// - `right0 - left0` must be in {0, 1}
    fn max_palindrome_from_middle<T: PartialEq>(
        s: &[T],
        left0: usize,
        right0: usize,
    ) -> (Range<usize>, usize) {
        let max_wing = max_wing_size(s.len(), left0, right0);
        let mut left0 = left0;
        let mut right0 = right0;
        if left0 != right0 {
            left0 += 1;
            right0 -= 1;
        }

        for _ in 0..max_wing {
            left0 -= 1;
            right0 += 1;
            if s[left0] != s[right0] {
                let left = left0 + 1;
                let len = right0 - left;
                return (left..right0, len);
            }
        }

        let right = right0 + 1;
        (left0..right, right - left0)
    }

    #[cfg(test)]
    mod test {
        use super::*;

        mod test_max_palindrome_from_middle {
            use super::*;

            #[test]
            fn should_return_max_palindrome_from_1_element_array() {
                assert_eq!(max_palindrome_from_middle(&[123], 0, 0), (0..1, 1));
            }

            #[test]
            fn should_return_max_palindrome_from_2_element_array() {
                let s1 = &['a', 'w'];
                assert_eq!(max_palindrome_from_middle(s1, 0, 0), (0..1, 1));
                assert_eq!(max_palindrome_from_middle(s1, 0, 1), (1..1, 0));
                assert_eq!(max_palindrome_from_middle(s1, 1, 1), (1..2, 1));

                let s2 = "aa".as_bytes();
                assert_eq!(max_palindrome_from_middle(s2, 0, 0), (0..1, 1));
                assert_eq!(max_palindrome_from_middle(s2, 0, 1), (0..2, 2));
                assert_eq!(max_palindrome_from_middle(s2, 1, 1), (1..2, 1));
            }

            #[test]
            fn should_return_max_palindrome_from_3_element_array() {
                let s1 = &['a', 'w', 'w'];
                assert_eq!(max_palindrome_from_middle(s1, 0, 0), (0..1, 1));
                assert_eq!(max_palindrome_from_middle(s1, 0, 1), (1..1, 0));
                assert_eq!(max_palindrome_from_middle(s1, 1, 1), (1..2, 1));
                assert_eq!(max_palindrome_from_middle(s1, 1, 2), (1..3, 2));
                assert_eq!(max_palindrome_from_middle(s1, 2, 2), (2..3, 1));

                let s2 = "aga".as_bytes();
                assert_eq!(max_palindrome_from_middle(s2, 0, 0), (0..1, 1));
                assert_eq!(max_palindrome_from_middle(s2, 0, 1), (1..1, 0));
                assert_eq!(max_palindrome_from_middle(s2, 1, 1), (0..3, 3));
                assert_eq!(max_palindrome_from_middle(s2, 1, 2), (2..2, 0));
                assert_eq!(max_palindrome_from_middle(s2, 2, 2), (2..3, 1));
            }

            #[test]
            fn should_return_max_palindrome_from_8_element_array() {
                let s2 = "abakagak".as_bytes();
                assert_eq!(max_palindrome_from_middle(s2, 0, 0), (0..1, 1));
                assert_eq!(max_palindrome_from_middle(s2, 0, 1), (1..1, 0));
                assert_eq!(max_palindrome_from_middle(s2, 1, 1), (0..3, 3));
                assert_eq!(max_palindrome_from_middle(s2, 1, 2), (2..2, 0));
                assert_eq!(max_palindrome_from_middle(s2, 2, 2), (2..3, 1));
                assert_eq!(max_palindrome_from_middle(s2, 2, 3), (3..3, 0));
                assert_eq!(max_palindrome_from_middle(s2, 3, 3), (2..5, 3));
                assert_eq!(max_palindrome_from_middle(s2, 3, 4), (4..4, 0));
                assert_eq!(max_palindrome_from_middle(s2, 4, 4), (4..5, 1));
                assert_eq!(max_palindrome_from_middle(s2, 4, 5), (5..5, 0));
                assert_eq!(max_palindrome_from_middle(s2, 5, 5), (3..8, 5));
            }
        }
        mod test_max_size {
            use super::*;
            #[test]
            fn should_return_max_wing_size_for_1_element_array() {
                assert_eq!(max_wing_size(1, 0, 0), 0);
            }
            #[test]
            fn should_return_max_wing_size_for_2_element_array() {
                let length = 2;
                assert_eq!(max_wing_size(length, 0, 0), 0);
                assert_eq!(max_wing_size(length, 0, 1), 1);
                assert_eq!(max_wing_size(length, 1, 1), 0);
            }

            #[test]
            fn should_return_max_wing_size_for_3_element_array() {
                let length = 3;
                /* 0 1 2 */
                assert_eq!(max_wing_size(length, 0, 0), 0);
                assert_eq!(max_wing_size(length, 0, 1), 1);
                assert_eq!(max_wing_size(length, 1, 1), 1);
                assert_eq!(max_wing_size(length, 1, 2), 1);
                assert_eq!(max_wing_size(length, 2, 2), 0);
            }

            #[test]
            fn should_return_max_wing_size_for_4_element_array() {
                let length = 4;
                /* 0 1 2 3 */
                assert_eq!(max_wing_size(length, 0, 0), 0);
                assert_eq!(max_wing_size(length, 0, 1), 1);
                assert_eq!(max_wing_size(length, 1, 1), 1);
                assert_eq!(max_wing_size(length, 1, 2), 2);
                assert_eq!(max_wing_size(length, 2, 2), 1);
                assert_eq!(max_wing_size(length, 2, 3), 1);
                assert_eq!(max_wing_size(length, 3, 3), 0);
            }

            #[test]
            fn should_return_max_wing_size_for_10_element_array() {
                let length = 10;
                /* 0 1 2 3 4 5 6 7 8 9 */
                assert_eq!(max_wing_size(length, 0, 0), 0);
                assert_eq!(max_wing_size(length, 0, 1), 1);
                assert_eq!(max_wing_size(length, 1, 1), 1);
                assert_eq!(max_wing_size(length, 1, 2), 2);
                assert_eq!(max_wing_size(length, 2, 2), 2);
                assert_eq!(max_wing_size(length, 2, 3), 3);
                assert_eq!(max_wing_size(length, 3, 3), 3);
                assert_eq!(max_wing_size(length, 3, 4), 4);
                assert_eq!(max_wing_size(length, 4, 4), 4);
                assert_eq!(max_wing_size(length, 4, 5), 5);
                assert_eq!(max_wing_size(length, 5, 5), 4);
                assert_eq!(max_wing_size(length, 5, 6), 4);
                assert_eq!(max_wing_size(length, 6, 6), 3);
                assert_eq!(max_wing_size(length, 6, 7), 3);
                assert_eq!(max_wing_size(length, 7, 7), 2);
                assert_eq!(max_wing_size(length, 7, 8), 2);
                assert_eq!(max_wing_size(length, 8, 8), 1);
                assert_eq!(max_wing_size(length, 8, 9), 1);
                assert_eq!(max_wing_size(length, 9, 9), 0);
            }
        }
    }
}
