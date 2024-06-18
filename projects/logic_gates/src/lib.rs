//! This is a simple logic gate simulation project to practice implementation of
//! unit and integrated tests and also rust in general.

pub fn and(a: u8, b: u8) -> u8 {
    (a & b) % 2
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
}
