use logic_gates::{and, xor};

pub type Sum = u8;
pub type Carry = u8;

pub fn half_adder_test_cases() -> Vec<((u8, u8), (Sum, Carry))> {
    vec![
        ((0, 0), (0, 0)),
        ((0, 1), (1, 0)),
        ((1, 0), (1, 0)),
        ((1, 1), (0, 1)),
    ]
}

fn half_adder(a: u8, b: u8) -> (Sum, Carry) {
    (xor(a, b), and(a, b))
}

#[test]
fn one_bit_adder() {
    for (input, output) in half_adder_test_cases() {
        let (a, b) = input;
        let (sum, carry) = output;
        println!("Testing: {} + {} = {} (Carry: {})", a, b, sum, carry);
        assert_eq!(half_adder(a, b), output);
    }
}
