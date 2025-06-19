/// Add two binary numbers represented as arrays. The high level representation of
/// the operation is: arr1[0..=n] + arr2[0..=n] stored into result[0..=n + 1].
///
/// # Contract
/// - It is expected that `arr1.len() == arr2.len() == result.len() - 1`
/// - Each slice may store only 0's and 1's.
/// - The representation of the binary number stored into the array `arr[0..=p]`
/// is: arr[0]*2^0 + arr[1]*2^1 + ... + arr[p]*2^p
///
/// # Warning
/// - `result` will be overwritten
pub fn binary_add(arr1: &[u8], arr2: &[u8], result: &mut [u8]) {
    let mut carry = 0;
    let mut sum;
    let mut b1;
    let mut b2;
    for i in 0..arr1.len() {
        b1 = arr1[i];
        b2 = arr2[i];
        sum = b1 + b2 + carry;

        result[i] = sum % 2;
        carry = sum >> 1;
    }
    result[arr1.len()] = carry;
}

#[cfg(test)]
mod test {
    use super::*;

    macro_rules! test_binary_add {
        ($(($arr1:expr, $arr2:expr, $expected:expr)),*) => {
            $({
                assert!($arr1.len() == $arr2.len(), "Invalid test case: arr1 and arr2 must have the same length");
                let mut result = vec![0; $arr1.len() + 1];
                binary_add($arr1, $arr2, &mut result);
                assert_eq!(result, $expected);
            })*
        };
    }

    #[test]
    fn should_add_1_bit_numbers() {
        test_binary_add!(
            (&[0], &[0], [0, 0]),
            (&[1], &[0], [1, 0]),
            (&[0], &[1], [1, 0]),
            (&[1], &[1], [0, 1])
        );
    }

    #[test]
    fn should_add_2_bits_number() {
        test_binary_add!(
            (&[0, 0], &[0, 0], [0, 0, 0]),
            (&[0, 0], &[0, 1], [0, 1, 0]),
            (&[0, 0], &[1, 0], [1, 0, 0]),
            (&[0, 0], &[1, 1], [1, 1, 0]),
            (&[0, 1], &[0, 0], [0, 1, 0]),
            (&[0, 1], &[0, 1], [0, 0, 1]),
            (&[0, 1], &[1, 0], [1, 1, 0]),
            (&[0, 1], &[1, 1], [1, 0, 1]),
            (&[1, 0], &[0, 0], [1, 0, 0]),
            (&[1, 0], &[0, 1], [1, 1, 0]),
            (&[1, 0], &[1, 0], [0, 1, 0]),
            (&[1, 0], &[1, 1], [0, 0, 1]),
            (&[1, 1], &[0, 0], [1, 1, 0]),
            (&[1, 1], &[0, 1], [1, 0, 1]),
            (&[1, 1], &[1, 0], [0, 0, 1]),
            (&[1, 1], &[1, 1], [0, 1, 1])
        );
    }

    #[test]
    fn should_add_3_bits_number() {
        test_binary_add!(
            // Adding 0 (000) to various numbers
            (&[0, 0, 0], &[0, 0, 0], [0, 0, 0, 0]),
            (&[0, 0, 0], &[1, 0, 0], [1, 0, 0, 0]),
            (&[0, 0, 0], &[1, 1, 1], [1, 1, 1, 0]),
            // Adding 1 (001) to various numbers
            (&[1, 0, 0], &[0, 0, 0], [1, 0, 0, 0]),
            (&[1, 0, 0], &[1, 0, 0], [0, 1, 0, 0]), // 1 + 1 = 2
            (&[1, 0, 0], &[1, 1, 0], [0, 0, 1, 0]), // 1 + 3 = 4
            (&[1, 0, 0], &[1, 1, 1], [0, 0, 0, 1]), // 1 + 7 = 8
            // Adding 2 (010) to various numbers
            (&[0, 1, 0], &[0, 1, 0], [0, 0, 1, 0]), // 2 + 2 = 4
            (&[0, 1, 0], &[1, 1, 0], [1, 0, 1, 0]), // 2 + 3 = 5
            (&[0, 1, 0], &[0, 0, 1], [0, 1, 1, 0]), // 2 + 4 = 6
            // Adding 3 (011) to various numbers
            (&[1, 1, 0], &[1, 1, 0], [0, 1, 1, 0]), // 3 + 3 = 6
            (&[1, 1, 0], &[0, 0, 1], [1, 1, 1, 0]), // 3 + 4 = 7
            (&[1, 1, 0], &[1, 0, 1], [0, 0, 0, 1]), // 3 + 5 = 8
            // Adding 4 (100) to various numbers
            (&[0, 0, 1], &[0, 0, 1], [0, 0, 0, 1]), // 4 + 4 = 8
            (&[0, 0, 1], &[1, 0, 1], [1, 0, 0, 1]), // 4 + 5 = 9
            (&[0, 0, 1], &[1, 1, 1], [1, 1, 0, 1]), // 4 + 7 = 11
            // Maximum values
            (&[1, 1, 1], &[1, 1, 1], [0, 1, 1, 1]) // 7 + 7 = 14
        );
    }

    #[test]
    fn should_add_4_bits_number() {
        test_binary_add!(
            // Edge cases with 4-bit numbers
            (&[0, 0, 0, 0], &[0, 0, 0, 0], [0, 0, 0, 0, 0]), // 0 + 0 = 0
            (&[1, 1, 1, 1], &[0, 0, 0, 0], [1, 1, 1, 1, 0]), // 15 + 0 = 15
            (&[1, 1, 1, 1], &[1, 0, 0, 0], [0, 0, 0, 0, 1]), // 15 + 1 = 16
            (&[1, 1, 1, 1], &[1, 1, 1, 1], [0, 1, 1, 1, 1]), // 15 + 15 = 30
            // Powers of 2
            (&[0, 0, 0, 1], &[0, 0, 0, 1], [0, 0, 0, 0, 1]), // 8 + 8 = 16
            (&[0, 0, 1, 0], &[0, 0, 1, 0], [0, 0, 0, 1, 0]), // 4 + 4 = 8
            (&[0, 1, 0, 0], &[0, 1, 0, 0], [0, 0, 1, 0, 0]), // 2 + 2 = 4
            // Mixed combinations
            (&[1, 0, 1, 0], &[0, 1, 0, 1], [1, 1, 1, 1, 0]), // 5 + 10 = 15
            (&[1, 1, 0, 1], &[0, 1, 1, 0], [1, 0, 0, 0, 1])  // 11 + 6 = 17
        );
    }

    #[test]
    fn should_add_larger_numbers() {
        test_binary_add!(
            // 5-bit numbers
            (&[1, 0, 1, 0, 1], &[0, 1, 0, 1, 0], [1, 1, 1, 1, 1, 0]), // 21 + 10 = 31
            (&[1, 1, 1, 1, 1], &[1, 0, 0, 0, 0], [0, 0, 0, 0, 0, 1]), // 31 + 1 = 32
            // 6-bit numbers
            (
                &[1, 1, 1, 1, 1, 1],
                &[1, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 1]
            ), // 63 + 1 = 64
            // 8-bit numbers (common byte size)
            (
                &[1, 1, 1, 1, 1, 1, 1, 1],
                &[1, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 1]
            ) // 255 + 1 = 256
        );
    }

    #[test]
    fn should_handle_carry_propagation() {
        test_binary_add!(
            // Cases that test multiple carry propagations
            (&[1, 1, 1], &[1, 0, 0], [0, 0, 0, 1]), // 7 + 1 = 8 (carry propagates through all bits)
            (&[1, 1, 1, 1], &[1, 0, 0, 0], [0, 0, 0, 0, 1]), // 15 + 1 = 16 (carry propagates through all bits)
            (&[1, 1], &[1, 1], [0, 1, 1]),                   // 3 + 3 = 6 (carry in middle)
            (&[1, 0, 1, 1], &[1, 1, 0, 0], [0, 0, 0, 0, 1])  // 13 + 3 = 16 (mixed carry pattern)
        );
    }
}
