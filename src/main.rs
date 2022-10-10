use std::f64::consts::PI;
use num::complex::Complex;


fn fft(xs: Vec<f64>) -> Vec<Complex<f64>> {
    // Following the Python version from https://pythonnumericalmethods.berkeley.edu/notebooks/chapter24.03-Fast-Fourier-Transform.html
    let n = xs.len();

    if n == 1 {
        return vec![Complex::new(xs[0], 0.0)]
    }

    let x_even: Vec<f64> = xs.iter().step_by(2).copied().collect();
    let x_odd: Vec<f64> = xs.iter().skip(1).step_by(2).copied().collect();
    
    let x_even_res = fft(x_even);
    let x_odd_res = fft(x_odd);

    let factor_mul: Complex<f64> = Complex::new(0.0, -2.0 * PI) / Complex::new(n as f64, 0.0);

    let c_p1: Vec<Complex<f64>> = (0..(n/2)).map(
        |i| x_even_res[i] + 
        (factor_mul * Complex::new(i as f64, 0.0)).exp() *
        x_odd_res[i]
    ).collect();

    let c_p2: Vec<Complex<f64>> = (0..(n/2)).map(
        |i| x_even_res[i] + 
        (factor_mul * Complex::new((i + (n/2)) as f64, 0.0)).exp() *
        x_odd_res[i]
    ).collect();

    return [c_p1, c_p2].concat();
}

fn main() {
    let xs: Vec<f64> = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];

    let res = fft(xs);

    for item in res.iter() {
        println!("{:>7.2} + {:>7.2}j", item.re, item.im);
    }
}
