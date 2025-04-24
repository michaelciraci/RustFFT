use num_complex::Complex;
use rustfft::FftPlanner;

#[test]
fn test_oop() {
    let mut p = FftPlanner::<f32>::new();
    let s = 50;
    let forward = p.plan_fft_forward(s);

    let mut buffer: Vec<Complex<f32>> = (0..s).map(|i| Complex::new(i as f32, 0.0)).collect();
    let mut scratch = vec![Complex::ZERO; forward.get_inplace_scratch_len()];

    forward.process_with_scratch(&mut buffer, &mut scratch);
}
