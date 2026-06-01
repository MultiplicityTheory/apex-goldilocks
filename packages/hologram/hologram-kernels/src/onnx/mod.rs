use num_traits::Float;

pub fn conv2d<T: Float>(
    x: &[T],
    w: &[T],
    y: &mut [T],
    h_in: usize,
    w_in: usize,
    k_h: usize,
    k_w: usize,
) {
    let h_out = h_in - k_h + 1;
    let w_out = w_in - k_w + 1;

    for i in 0..h_out {
        for j in 0..w_out {
            let mut sum = T::zero();
            for ki in 0..k_h {
                for kj in 0..k_w {
                    let in_i = i + ki;
                    let in_j = j + kj;
                    let x_val = x[in_i * w_in + in_j];
                    let w_val = w[ki * k_w + kj];
                    sum = sum + x_val * w_val;
                }
            }
            y[i * w_out + j] = sum;
        }
    }
}

pub fn layer_norm<T: Float>(
    x: &[T],
    gamma: &[T],
    beta: &[T],
    y: &mut [T],
    n: usize,
    d: usize,
    epsilon: T,
) {
    for i in 0..n {
        let offset = i * d;
        let mut mean = T::zero();
        for j in 0..d {
            mean = mean + x[offset + j];
        }
        mean = mean / T::from(d).unwrap();

        let mut var = T::zero();
        for j in 0..d {
            let diff = x[offset + j] - mean;
            var = var + diff * diff;
        }
        var = var / T::from(d).unwrap();
        let std_inv = T::one() / (var + epsilon).sqrt();

        for j in 0..d {
            y[offset + j] = (x[offset + j] - mean) * std_inv * gamma[j] + beta[j];
        }
    }
}

pub fn gelu<T: Float>(x: &[T], y: &mut [T], n: usize) {
    let sqrt_2_over_pi = T::from((2.0 / std::f64::consts::PI).sqrt()).unwrap();
    let coeff = T::from(0.044715).unwrap();
    let zero_five = T::from(0.5).unwrap();
    let one = T::one();

    for i in 0..n {
        let val = x[i];
        let cube = val * val * val;
        let inner = sqrt_2_over_pi * (val + coeff * cube);
        y[i] = zero_five * val * (one + inner.tanh());
    }
}

pub fn softmax<T: Float>(x: &[T], y: &mut [T], n: usize) {
    let mut max_val = T::neg_infinity();
    for i in 0..n {
        if x[i] > max_val {
            max_val = x[i];
        }
    }

    let mut sum_exp = T::zero();
    for i in 0..n {
        y[i] = (x[i] - max_val).exp();
        sum_exp = sum_exp + y[i];
    }

    for i in 0..n {
        y[i] = y[i] / sum_exp;
    }
}
