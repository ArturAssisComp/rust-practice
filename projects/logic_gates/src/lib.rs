#![doc(
    html_logo_url = "https://th.bing.com/th/id/OIP.nXQrAEVILEZM61I9eJL1KgHaF9?rs=1&pid=ImgDetMain"
)]

//! This is a simple logic gate simulation project to practice implementation of
//! unit and integrated tests and also rust in general.

/// This function represents the logic gate `and`. It receives two bits and
/// returns a bit as result. The `and` gate returns 1 if and only if both inputs
/// are 1. Otherwise, it returns 0.
pub fn and(a: u8, b: u8) -> u8 {
    (a & b) % 2
}

/// This function represents the logic gate `xor`. It receives two bits and
/// returns a bit as result. The `xor` gate returns 1 if and only if exactly one
/// of the inputs is 1. Otherwise, it returns 0.
pub fn xor(a: u8, b: u8) -> u8 {
    (a ^ b) % 2
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Template that will be used by the
    struct TestTemplate<'a> {
        input: (u8, u8),
        expected: u8,
        description: &'a str,
    }

    #[test]
    fn test_and() {
        let test_cases: [TestTemplate; 4] = [
            TestTemplate {
                input: (0, 0),
                expected: 0,
                description: "0 ^ 0 = 0",
            },
            TestTemplate {
                input: (0, 1),
                expected: 0,
                description: "0 ^ 1 = 0",
            },
            TestTemplate {
                input: (1, 0),
                expected: 0,
                description: "1 ^ 0 = 0",
            },
            TestTemplate {
                input: (1, 1),
                expected: 1,
                description: "1 ^ 1 = 1",
            },
        ];
        for TestTemplate {
            input: (a, b),
            expected,
            description,
        } in test_cases
        {
            assert_eq!(and(a, b), expected, "{}", description);
        }
    }

    #[test]
    fn test_xor() {
        let test_cases: [TestTemplate; 4] = [
            TestTemplate {
                input: (0, 0),
                expected: 0,
                description: "0 ^ 0 = 0",
            },
            TestTemplate {
                input: (0, 1),
                expected: 1,
                description: "0 ^ 1 = 1",
            },
            TestTemplate {
                input: (1, 0),
                expected: 1,
                description: "1 ^ 0 = 1",
            },
            TestTemplate {
                input: (1, 1),
                expected: 0,
                description: "1 ^ 1 = 0",
            },
        ];
        for TestTemplate {
            input: (a, b),
            expected,
            description,
        } in test_cases
        {
            assert_eq!(xor(a, b), expected, "{}", description);
        }
    }
}
