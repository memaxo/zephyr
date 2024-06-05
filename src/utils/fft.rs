use num::Complex;
use rayon::prelude::*;

type BaseElement = f64;

fn parallel_fft(input: &[BaseElement], len: usize) -> Result<Vec<Complex<BaseElement>>, String> {
    if len.count_ones() != 1 {
        return Err("Input length must be a power of 2".to_string());
    }

    if input.len() <= 1 {
        return Ok(input.iter().map(|&x| Complex::new(x, 0.0)).collect());
    }

    let even: Vec<_> = input.iter().step_by(2).cloned().collect();
    let odd: Vec<_> = input.iter().skip(1).step_by(2).cloned().collect();

    let (even_fft, odd_fft) = rayon::join(
        || parallel_fft(&even, len / 2),
        || parallel_fft(&odd, len / 2),
    );

    let even_fft = even_fft?;
    let odd_fft = odd_fft?;

    let mut result = vec![Complex::new(0.0, 0.0); len];
    let half = len / 2;

    for k in 0..half {
        let angle = -2.0 * std::f64::consts::PI * (k as BaseElement) / (len as BaseElement);
        let twiddle_factor = Complex::new(angle.cos(), angle.sin());
        let t = odd_fft[k] * twiddle_factor;
        result[k] = even_fft[k] + t;
        result[k + half] = even_fft[k] - t;
    }

    Ok(result)
}