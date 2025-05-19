/// Exchange the elements of index `i1` and `i2` from mutable slice `v`.
///
/// # Patterns
/// - ($v:expr, $i1:expr, $i2:expr)=> ()
#[macro_export]
macro_rules! exchange {
    ($v:expr, $i1:expr, $i2:expr) => {{
        let tmp = $v[$i1];
        $v[$i1] = $v[$i2];
        $v[$i2] = tmp;
    }};
}
