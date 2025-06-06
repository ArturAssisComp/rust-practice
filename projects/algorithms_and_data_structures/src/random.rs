use rand::Rng;
use serial_test::serial;

type TossCoinFunction = fn() -> usize;
type DaryTossCoinFunction = fn(arity: usize) -> usize;

fn toss_coin() -> usize {
    let mut r = rand::rng();
    if r.random_bool(0.5) {
        1
    } else {
        0
    }
}

fn toss_d_ary_coin(n: usize) -> usize {
    let mut r = rand::rng();
    r.random_range(0..n)
}

/// Returns a random number within the range [start, end - 1]. The distribution is uniform
/// and the function used to build the random number is a toss coin function.
///
/// # Contracts
/// - `start` < `end`
/// - `toss_coin_fn` must return only 0 or 1
pub fn toss_coin_random(start: usize, end: usize, toss_coin_fn: TossCoinFunction) -> usize {
    let num_of_possibilitie = end - start;
    if num_of_possibilitie == 1 {
        return start;
    }
    let mut sentinel = num_of_possibilitie - 1;
    let mut answer;
    loop {
        answer = 0;
        while sentinel > 0 {
            sentinel >>= 1;
            answer = (answer << 1) + toss_coin_fn();
        }
        if (0..num_of_possibilitie).contains(&answer) {
            return start + answer;
        }
        sentinel = num_of_possibilitie;
    }
}

/// Returns a random number within the range [start, end - 1]. The distribution is uniform
/// and the function used to build the random number is a d-ary toss coin function.
///
/// # Contracts
/// - `start` < `end`
/// - `toss_coin_fn` must return only 0, 1, ... , arity - 1
pub fn toss_coin_random_d_ary(
    start: usize,
    end: usize,
    toss_coin_fn: DaryTossCoinFunction,
    arity: usize,
) -> usize {
    let num_of_possibilitie = end - start;
    if num_of_possibilitie == 1 {
        return start;
    }
    let mut sentinel = num_of_possibilitie - 1;
    let mut answer;
    loop {
        answer = 0;
        while sentinel > 0 {
            sentinel /= arity;
            answer = (answer * arity) + toss_coin_fn(arity);
        }
        if (0..num_of_possibilitie).contains(&answer) {
            return start + answer;
        }
        sentinel = num_of_possibilitie;
    }
}

#[cfg(test)]
#[serial]
mod test {
    use super::*;
    static mut SEQUENCE: &[usize] = &[];
    static mut I: usize = 0;

    fn build_toss_coin_function(new_sequence: &'static [usize]) -> TossCoinFunction {
        unsafe {
            SEQUENCE = new_sequence;
            I = 0;
        }
        #[allow(static_mut_refs)]
        fn predetermined_toss_coin() -> usize {
            let ans;
            unsafe {
                ans = SEQUENCE[I];
                I += 1;
            }
            assert!(ans <= 1, "invalid predetermined toss coin value");
            ans
        }
        predetermined_toss_coin
    }

    fn build_dary_toss_coin_function(new_sequence: &'static [usize]) -> DaryTossCoinFunction {
        unsafe {
            SEQUENCE = new_sequence;
            I = 0;
        }
        #[allow(static_mut_refs)]
        fn predetermined_toss_coin(arity: usize) -> usize {
            let ans;
            unsafe {
                ans = SEQUENCE[I];
                I += 1;
            }
            assert!(ans <= arity - 1, "invalid predetermined toss coin value");
            ans
        }
        predetermined_toss_coin
    }

    mod test_toss_coin_random {
        use super::*;
        #[test]
        fn should_return_one_deterministic_element() {
            assert_eq!(toss_coin_random(0, 1, toss_coin), 0);
            assert_eq!(toss_coin_random(1, 2, toss_coin), 1);
            assert_eq!(toss_coin_random(2, 3, toss_coin), 2);
            assert_eq!(toss_coin_random(1000023, 1000024, toss_coin), 1000023);
        }

        #[test]
        fn should_return_an_element_from_2_size_range() {
            assert_eq!(toss_coin_random(0, 2, build_toss_coin_function(&[0])), 0);
            assert_eq!(toss_coin_random(0, 2, build_toss_coin_function(&[1])), 1);
            assert_eq!(
                toss_coin_random(100, 102, build_toss_coin_function(&[0])),
                100
            );
            assert_eq!(toss_coin_random(45, 47, build_toss_coin_function(&[1])), 46);
        }

        #[test]
        fn should_return_an_element_from_25_size_range_with_repetition() {
            assert_eq!(
                toss_coin_random(0, 25, build_toss_coin_function(&[1, 0, 0, 1, 0])),
                18
            );
            assert_eq!(
                toss_coin_random(
                    101,
                    126,
                    build_toss_coin_function(&[
                        1, 1, 1, 1, 1, // 63
                        1, 1, 1, 0, 0, // 60
                        1, 1, 1, 1, 0, // 62
                        1, 0, 0, 1, 0
                    ])
                ),
                119
            );
        }

        #[test]
        fn should_return_an_element_from_3_size_range() {
            assert_eq!(toss_coin_random(0, 3, build_toss_coin_function(&[0, 0])), 0);
            assert_eq!(toss_coin_random(0, 3, build_toss_coin_function(&[0, 1])), 1);
            assert_eq!(toss_coin_random(0, 3, build_toss_coin_function(&[1, 0])), 2);
            assert_eq!(
                toss_coin_random(0, 3, build_toss_coin_function(&[1, 1, 0, 1])),
                1
            );

            let start = 10005;
            assert_eq!(
                toss_coin_random(start, start + 3, build_toss_coin_function(&[0, 0])),
                start
            );
            assert_eq!(
                toss_coin_random(start, start + 3, build_toss_coin_function(&[0, 1])),
                start + 1
            );
            assert_eq!(
                toss_coin_random(start, start + 3, build_toss_coin_function(&[1, 0])),
                start + 2
            );
            assert_eq!(
                toss_coin_random(start, start + 3, build_toss_coin_function(&[1, 1, 0, 1])),
                start + 1
            );
        }

        #[test]
        fn should_return_an_element_from_10_size_range() {
            assert_eq!(
                toss_coin_random(0, 10, build_toss_coin_function(&[0, 0, 0, 0])),
                0
            );
            assert_eq!(
                toss_coin_random(0, 10, build_toss_coin_function(&[0, 0, 0, 1])),
                1
            );
            assert_eq!(
                toss_coin_random(0, 10, build_toss_coin_function(&[0, 0, 1, 0])),
                2
            );
            assert_eq!(
                toss_coin_random(0, 10, build_toss_coin_function(&[0, 0, 1, 1])),
                3
            );
            assert_eq!(
                toss_coin_random(0, 10, build_toss_coin_function(&[0, 1, 0, 0])),
                4
            );
            assert_eq!(
                toss_coin_random(0, 10, build_toss_coin_function(&[0, 1, 0, 1])),
                5
            );
            assert_eq!(
                toss_coin_random(0, 10, build_toss_coin_function(&[0, 1, 1, 0])),
                6
            );
            assert_eq!(
                toss_coin_random(0, 10, build_toss_coin_function(&[0, 1, 1, 1])),
                7
            );
            assert_eq!(
                toss_coin_random(0, 10, build_toss_coin_function(&[1, 0, 0, 0])),
                8
            );
            assert_eq!(
                toss_coin_random(0, 10, build_toss_coin_function(&[1, 0, 0, 1])),
                9
            );
            assert_eq!(
                toss_coin_random(0, 10, build_toss_coin_function(&[1, 0, 1, 1, 1, 0, 0, 1])),
                9
            );
            assert_eq!(
                toss_coin_random(0, 11, build_toss_coin_function(&[1, 0, 1, 0])),
                10
            );

            let start = 1234567;
            assert_eq!(
                toss_coin_random(start, start + 10, build_toss_coin_function(&[0, 0, 0, 0])),
                start
            );
            assert_eq!(
                toss_coin_random(start, start + 10, build_toss_coin_function(&[0, 0, 0, 1])),
                start + 1
            );
            assert_eq!(
                toss_coin_random(start, start + 10, build_toss_coin_function(&[0, 0, 1, 0])),
                start + 2
            );
            assert_eq!(
                toss_coin_random(start, start + 10, build_toss_coin_function(&[0, 0, 1, 1])),
                start + 3
            );
            assert_eq!(
                toss_coin_random(start, start + 10, build_toss_coin_function(&[0, 1, 0, 0])),
                start + 4
            );
            assert_eq!(
                toss_coin_random(start, start + 10, build_toss_coin_function(&[0, 1, 0, 1])),
                start + 5
            );
            assert_eq!(
                toss_coin_random(start, start + 10, build_toss_coin_function(&[0, 1, 1, 0])),
                start + 6
            );
            assert_eq!(
                toss_coin_random(start, start + 10, build_toss_coin_function(&[0, 1, 1, 1])),
                start + 7
            );
            assert_eq!(
                toss_coin_random(start, start + 10, build_toss_coin_function(&[1, 0, 0, 0])),
                start + 8
            );
            assert_eq!(
                toss_coin_random(start, start + 10, build_toss_coin_function(&[1, 0, 0, 1])),
                start + 9
            );
            assert_eq!(
                toss_coin_random(
                    start,
                    start + 10,
                    build_toss_coin_function(&[1, 0, 1, 1, 1, 0, 0, 1])
                ),
                start + 9
            );
        }
    }

    mod test_d_ary_toss_coin_random {
        use super::*;
        #[test]
        fn should_return_one_deterministic_element() {
            assert_eq!(toss_coin_random_d_ary(0, 1, toss_d_ary_coin, 10), 0);
            assert_eq!(toss_coin_random_d_ary(1, 2, toss_d_ary_coin, 10), 1);
            assert_eq!(toss_coin_random_d_ary(2, 3, toss_d_ary_coin, 2), 2);
            assert_eq!(
                toss_coin_random_d_ary(1000023, 1000024, toss_d_ary_coin, 3),
                1000023
            );
        }

        #[test]
        fn should_return_an_element_from_2_size_range() {
            assert_eq!(
                toss_coin_random_d_ary(0, 2, build_dary_toss_coin_function(&[0]), 2),
                0
            );
            assert_eq!(
                toss_coin_random_d_ary(0, 2, build_dary_toss_coin_function(&[1]), 2),
                1
            );
            assert_eq!(
                toss_coin_random_d_ary(100, 102, build_dary_toss_coin_function(&[0]), 2),
                100
            );
            assert_eq!(
                toss_coin_random_d_ary(45, 47, build_dary_toss_coin_function(&[1]), 2),
                46
            );
            assert_eq!(
                toss_coin_random_d_ary(45, 47, build_dary_toss_coin_function(&[1]), 3),
                46
            );
            assert_eq!(
                toss_coin_random_d_ary(45, 47, build_dary_toss_coin_function(&[0]), 80),
                45
            );
        }

        #[test]
        fn should_return_an_element_from_25_size_range_with_repetition() {
            assert_eq!(
                toss_coin_random_d_ary(0, 25, build_dary_toss_coin_function(&[1, 0, 0, 1, 0]), 2),
                18
            );
            assert_eq!(
                toss_coin_random_d_ary(0, 25, build_dary_toss_coin_function(&[2, 0, 0]), 3),
                18
            );
            assert_eq!(
                toss_coin_random_d_ary(
                    0,
                    25,
                    build_dary_toss_coin_function(&[2, 2, 2, 2, 0, 0]),
                    3
                ),
                18
            );
            assert_eq!(
                toss_coin_random_d_ary(
                    101,
                    126,
                    build_dary_toss_coin_function(&[
                        1, 1, 1, 1, 1, // 63
                        1, 1, 1, 0, 0, // 60
                        1, 1, 1, 1, 0, // 62
                        1, 0, 0, 1, 0
                    ]),
                    2
                ),
                119
            );
            assert_eq!(
                toss_coin_random_d_ary(
                    101,
                    126,
                    build_dary_toss_coin_function(&[
                        2, 0, 0, // 32
                        2, 0, 3, // 35
                        1, 0, 2
                    ]),
                    4
                ),
                119
            );
        }

        #[test]
        fn should_return_an_element_from_3_size_range() {
            assert_eq!(
                toss_coin_random_d_ary(0, 3, build_dary_toss_coin_function(&[0, 0]), 2),
                0
            );
            assert_eq!(
                toss_coin_random_d_ary(0, 3, build_dary_toss_coin_function(&[0, 1]), 2),
                1
            );
            assert_eq!(
                toss_coin_random_d_ary(0, 3, build_dary_toss_coin_function(&[1, 0]), 2),
                2
            );
            assert_eq!(
                toss_coin_random_d_ary(0, 3, build_dary_toss_coin_function(&[1, 1, 0, 1]), 2),
                1
            );

            let start = 10005;
            assert_eq!(
                toss_coin_random_d_ary(start, start + 3, build_dary_toss_coin_function(&[0, 0]), 2),
                start
            );
            assert_eq!(
                toss_coin_random_d_ary(start, start + 3, build_dary_toss_coin_function(&[0, 1]), 2),
                start + 1
            );
            assert_eq!(
                toss_coin_random_d_ary(start, start + 3, build_dary_toss_coin_function(&[1, 0]), 2),
                start + 2
            );
            assert_eq!(
                toss_coin_random_d_ary(
                    start,
                    start + 3,
                    build_dary_toss_coin_function(&[1, 1, 0, 1]),
                    2
                ),
                start + 1
            );
        }

        #[test]
        fn should_return_an_element_from_10_size_range() {
            assert_eq!(
                toss_coin_random_d_ary(0, 10, build_dary_toss_coin_function(&[0, 0, 0, 0]), 2),
                0
            );
            assert_eq!(
                toss_coin_random_d_ary(0, 10, build_dary_toss_coin_function(&[0, 0, 0, 1]), 2),
                1
            );
            assert_eq!(
                toss_coin_random_d_ary(0, 10, build_dary_toss_coin_function(&[0, 0, 1, 0]), 2),
                2
            );
            assert_eq!(
                toss_coin_random_d_ary(0, 10, build_dary_toss_coin_function(&[0, 0, 1, 1]), 2),
                3
            );
            assert_eq!(
                toss_coin_random_d_ary(0, 10, build_dary_toss_coin_function(&[0, 1, 0, 0]), 2),
                4
            );
            assert_eq!(
                toss_coin_random_d_ary(0, 10, build_dary_toss_coin_function(&[0, 1, 0, 1]), 2),
                5
            );
            assert_eq!(
                toss_coin_random_d_ary(0, 10, build_dary_toss_coin_function(&[0, 1, 1, 0]), 2),
                6
            );
            assert_eq!(
                toss_coin_random_d_ary(0, 10, build_dary_toss_coin_function(&[0, 1, 1, 1]), 2),
                7
            );
            assert_eq!(
                toss_coin_random_d_ary(0, 10, build_dary_toss_coin_function(&[1, 0, 0, 0]), 2),
                8
            );
            assert_eq!(
                toss_coin_random_d_ary(0, 10, build_dary_toss_coin_function(&[1, 0, 0, 1]), 2),
                9
            );
            assert_eq!(
                toss_coin_random_d_ary(
                    0,
                    10,
                    build_dary_toss_coin_function(&[1, 0, 1, 1, 1, 0, 0, 1]),
                    2
                ),
                9
            );
            assert_eq!(
                toss_coin_random_d_ary(0, 11, build_dary_toss_coin_function(&[1, 0, 1, 0]), 2),
                10
            );

            let start = 1234567;
            assert_eq!(
                toss_coin_random_d_ary(
                    start,
                    start + 10,
                    build_dary_toss_coin_function(&[0, 0, 0, 0]),
                    2
                ),
                start
            );
            assert_eq!(
                toss_coin_random_d_ary(
                    start,
                    start + 10,
                    build_dary_toss_coin_function(&[0, 0, 0, 1]),
                    2
                ),
                start + 1
            );
            assert_eq!(
                toss_coin_random_d_ary(
                    start,
                    start + 10,
                    build_dary_toss_coin_function(&[0, 0, 1, 0]),
                    2
                ),
                start + 2
            );
            assert_eq!(
                toss_coin_random_d_ary(
                    start,
                    start + 10,
                    build_dary_toss_coin_function(&[0, 0, 1, 1]),
                    2
                ),
                start + 3
            );
            assert_eq!(
                toss_coin_random_d_ary(
                    start,
                    start + 10,
                    build_dary_toss_coin_function(&[0, 1, 0, 0]),
                    2
                ),
                start + 4
            );
            assert_eq!(
                toss_coin_random_d_ary(
                    start,
                    start + 10,
                    build_dary_toss_coin_function(&[0, 1, 0, 1]),
                    2
                ),
                start + 5
            );
            assert_eq!(
                toss_coin_random_d_ary(
                    start,
                    start + 10,
                    build_dary_toss_coin_function(&[0, 1, 1, 0]),
                    2
                ),
                start + 6
            );
            assert_eq!(
                toss_coin_random_d_ary(
                    start,
                    start + 10,
                    build_dary_toss_coin_function(&[0, 1, 1, 1]),
                    2
                ),
                start + 7
            );
            assert_eq!(
                toss_coin_random_d_ary(
                    start,
                    start + 10,
                    build_dary_toss_coin_function(&[1, 0, 0, 0]),
                    2
                ),
                start + 8
            );
            assert_eq!(
                toss_coin_random_d_ary(
                    start,
                    start + 10,
                    build_dary_toss_coin_function(&[1, 0, 0, 1]),
                    2
                ),
                start + 9
            );
            assert_eq!(
                toss_coin_random_d_ary(
                    start,
                    start + 10,
                    build_dary_toss_coin_function(&[1, 0, 1, 1, 1, 0, 0, 1]),
                    2
                ),
                start + 9
            );
            assert_eq!(
                toss_coin_random_d_ary(
                    start,
                    start + 10,
                    build_dary_toss_coin_function(&[1, 1]),
                    8
                ),
                start + 9
            );
        }
    }
}
