type Coordinate = (usize, usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct MatrixSlice {
    top_left: Coordinate,
    bottom_right: Coordinate,
}

impl MatrixSlice {
    fn new(num_of_lines: usize, num_of_columns: usize) -> Self {
        Self {
            top_left: (0, 0),
            bottom_right: (num_of_lines, num_of_columns),
        }
    }

    fn split_4(self) -> (Self, Self, Self, Self) {
        let y0 = self.top_left.0;
        let x0 = self.top_left.1;
        let y1 = self.bottom_right.0;
        let x1 = self.bottom_right.1;

        let half_height = (y1 - y0) / 2;
        let half_width = (x1 - x0) / 2;

        let x_mid = x0 + half_width;
        let y_mid = y0 + half_height;

        (
            // A11
            Self {
                top_left: (y0, x0),
                bottom_right: (y_mid, x_mid),
            },
            // A12
            Self {
                top_left: (y0, x_mid),
                bottom_right: (y_mid, x1),
            },
            // A21
            Self {
                top_left: (y_mid, x0),
                bottom_right: (y1, x_mid),
            },
            // A22
            Self {
                top_left: (y_mid, x_mid),
                bottom_right: (y1, x1),
            },
        )
    }
}

struct Matrix<T> {
    data: Vec<Vec<T>>,
    num_of_lines: usize,
    num_of_columns: usize,
}

impl<T: Copy> Matrix<T> {
    fn new(num_of_lines: usize, num_of_columns: usize, default: T) -> Self {
        let mut data = Vec::with_capacity(num_of_lines);
        for _ in 0..num_of_lines {
            data.push(vec![default; num_of_columns]);
        }
        Self {
            data,
            num_of_columns,
            num_of_lines,
        }
    }
    fn from_data(data: Vec<Vec<T>>) -> Self {
        let num_of_lines = data.len();
        let num_of_columns = if num_of_lines == 0 { 0 } else { data[0].len() };

        debug_assert!(data.iter().all(|e| e.len() == num_of_columns));

        Self {
            data,
            num_of_columns,
            num_of_lines,
        }
    }
}

fn recursive_matrix_mul<T: Copy>(
    m1: &Matrix<T>,
    m2: &Matrix<T>,
    result: &mut Matrix<T>,
    slice: MatrixSlice,
) -> Result<(), &'static str> {
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    mod matrix_slice_tests {
        use super::*;

        mod split_4_tests {
            use super::*;

            #[test]
            fn split_2x2_slice() {
                let slc1 = MatrixSlice {
                    top_left: (0, 0),
                    bottom_right: (2, 2),
                };
                assert_eq!(
                    slc1.split_4(),
                    (
                        // A11
                        MatrixSlice {
                            top_left: (0, 0),
                            bottom_right: (1, 1)
                        },
                        // A12
                        MatrixSlice {
                            top_left: (0, 1),
                            bottom_right: (1, 2)
                        },
                        // A21
                        MatrixSlice {
                            top_left: (1, 0),
                            bottom_right: (2, 1)
                        },
                        // A22
                        MatrixSlice {
                            top_left: (1, 1),
                            bottom_right: (2, 2)
                        },
                    )
                );
            }
        }
    }
}
