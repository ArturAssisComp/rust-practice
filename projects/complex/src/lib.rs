use std::convert::From;
use std::ops::Add;

#[derive(Default, Debug, PartialEq, Copy, Clone)]
struct Complex<T> {
    real: T,
    imaginary: T,
}

impl<T> Complex<T> {
    fn new(real: T, imaginary: T) -> Self {
        Complex { real, imaginary }
    }
}

impl<T: Add<T, Output = T>> Add for Complex<T> {
    type Output = Complex<T>;
    fn add(self, rhs: Self) -> Self::Output {
        Complex::new(self.real + rhs.real, self.imaginary + rhs.imaginary)
    }
}

impl<T> From<(T, T)> for Complex<T> {
    fn from(value: (T, T)) -> Self {
        Complex {
            real: value.0,
            imaginary: value.1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn complex_basics() {
        let first = Complex::new(3, 5);
        let second: Complex<i32> = Complex::default();
        assert_eq!(first.real, 3);
        assert_eq!(first.imaginary, 5);
        assert!(second.real == second.imaginary);
    }

    #[test]
    fn complex_add() {
        let first = Complex::new(3, 5);
        let second = Complex::new(1, 2);
        let expected = Complex::new(4, 7);
        assert_eq!(
            first + second,
            expected,
            "Check if {}+{}i + {}+{}i = {}+{}i",
            first.real,
            first.imaginary,
            second.real,
            second.imaginary,
            expected.real,
            expected.imaginary
        );
    }

    #[test]
    fn complex_from() {
        let real = 12;
        let imaginary = 35;
        let tuple = (real, imaginary);
        let result = Complex::from(tuple);

        assert_eq!(result.real, real, "Check the real part");
        assert_eq!(result.imaginary, imaginary, "Check the imaginary part");
    }
}
