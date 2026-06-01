use num_traits::Float;

pub fn relu<T: Float>(a: &[T], c: &mut [T], n: usize) {
    for i in 0..n {
        if a[i] > T::zero() {
            c[i] = a[i];
        } else {
            c[i] = T::zero();
        }
    }
}

pub fn sigmoid<T: Float>(a: &[T], c: &mut [T], n: usize) {
    for i in 0..n {
        c[i] = T::one() / (T::one() + (-a[i]).exp());
    }
}

pub fn tanh<T: Float>(a: &[T], c: &mut [T], n: usize) {
    for i in 0..n {
        c[i] = a[i].tanh();
    }
}
