use num_traits::Float;

pub fn vector_add<T: Float>(a: &[T], b: &[T], c: &mut [T], n: usize) {
    for i in 0..n {
        c[i] = a[i] + b[i];
    }
}

pub fn vector_sub<T: Float>(a: &[T], b: &[T], c: &mut [T], n: usize) {
    for i in 0..n {
        c[i] = a[i] - b[i];
    }
}

pub fn vector_mul<T: Float>(a: &[T], b: &[T], c: &mut [T], n: usize) {
    for i in 0..n {
        c[i] = a[i] * b[i];
    }
}

pub fn vector_div<T: Float>(a: &[T], b: &[T], c: &mut [T], n: usize) {
    for i in 0..n {
        c[i] = a[i] / b[i];
    }
}

pub fn vector_neg<T: Float>(a: &[T], c: &mut [T], n: usize) {
    for i in 0..n {
        c[i] = -a[i];
    }
}

pub fn vector_abs<T: Float>(a: &[T], c: &mut [T], n: usize) {
    for i in 0..n {
        c[i] = a[i].abs();
    }
}

pub fn vector_sqrt<T: Float>(a: &[T], c: &mut [T], n: usize) {
    for i in 0..n {
        c[i] = a[i].sqrt();
    }
}

pub fn vector_dot<T: Float>(a: &[T], b: &[T], n: usize) -> T {
    let mut sum = T::zero();
    for i in 0..n {
        sum = sum + a[i] * b[i];
    }
    sum
}

pub fn vector_sum<T: Float>(a: &[T], n: usize) -> T {
    let mut sum = T::zero();
    for i in 0..n {
        sum = sum + a[i];
    }
    sum
}

pub fn vector_pow<T: Float>(a: &[T], b: &[T], c: &mut [T], n: usize) {
    for i in 0..n {
        c[i] = a[i].powf(b[i]);
    }
}
