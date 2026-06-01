use num_traits::Float;

pub fn matrix_gemm<T: Float>(
    a: &[T],
    b: &[T],
    c: &mut [T],
    m: usize,
    n: usize,
    k: usize,
    alpha: T,
    beta: T,
) {
    for i in 0..m {
        for j in 0..n {
            let mut sum = T::zero();
            for p in 0..k {
                sum = sum + a[i * k + p] * b[p * n + j];
            }
            c[i * n + j] = alpha * sum + beta * c[i * n + j];
        }
    }
}

pub fn matrix_gemv<T: Float>(
    a: &[T],
    x: &[T],
    y: &mut [T],
    m: usize,
    n: usize,
    alpha: T,
    beta: T,
) {
    for i in 0..m {
        let mut sum = T::zero();
        for j in 0..n {
            sum = sum + a[i * n + j] * x[j];
        }
        y[i] = alpha * sum + beta * y[i];
    }
}
