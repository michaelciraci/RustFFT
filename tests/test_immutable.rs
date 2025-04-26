use num_complex::Complex;
use rustfft::FftPlanner;

#[test]
fn immutable_f32_0_thru_9() {
    for i in 0..5000 {
        dbg!(i);
        let input = vec![Complex::new(7.0, 8.0); i];

        let mut_output = fft_wrapper_mut_f32(&input);
        let immut_output = fft_wrapper_immut_f32(&input);

        assert_eq!(mut_output, immut_output);
    }
}

#[test]
fn immutable_f64_0_thru_9() {
    for i in 0..5000 {
        dbg!(i);
        let input = vec![Complex::new(7.0, 8.0); i];

        let mut_output = fft_wrapper_mut_f64(&input);
        let immut_output = fft_wrapper_immut_f64(&input);

        assert_eq!(mut_output, immut_output);
    }
}

#[test]
fn immut_18() {
    let input = vec![Complex::new(7.0, 8.0); 118];

    let mut_output = fft_wrapper_mut_f64(&input);
    let immut_output = fft_wrapper_immut_f64(&input);

    assert_eq!(mut_output, immut_output);
}

fn fft_wrapper_mut_f32(input: &[Complex<f32>]) -> Vec<Complex<f32>> {
    let mut plan = FftPlanner::<f32>::new();
    let p = plan.plan_fft_forward(input.len());

    let mut scratch = vec![Complex::new(0.0, 0.0); p.get_inplace_scratch_len()];
    let mut output = input.to_vec();

    p.process_with_scratch(&mut output, &mut scratch);
    output
}

fn fft_wrapper_immut_f32(input: &[Complex<f32>]) -> Vec<Complex<f32>> {
    let mut plan = FftPlanner::<f32>::new();
    let p = plan.plan_fft_forward(input.len());

    let mut scratch = vec![Complex::new(0.0, 0.0); p.get_immutable_scratch_len()];
    let mut output = vec![Complex::new(0.0, 0.0); input.len()];

    p.process_immutable_with_scratch(input, &mut output, &mut scratch);
    output
}

fn fft_wrapper_mut_f64(input: &[Complex<f64>]) -> Vec<Complex<f64>> {
    let mut plan = FftPlanner::<f64>::new();
    let p = plan.plan_fft_forward(input.len());

    let mut scratch = vec![Complex::new(0.0, 0.0); p.get_inplace_scratch_len()];
    let mut output = input.to_vec();

    p.process_with_scratch(&mut output, &mut scratch);
    output
}

fn fft_wrapper_immut_f64(input: &[Complex<f64>]) -> Vec<Complex<f64>> {
    let mut plan = FftPlanner::<f64>::new();
    let p = plan.plan_fft_forward(input.len());

    let mut scratch = vec![Complex::new(0.0, 0.0); p.get_immutable_scratch_len()];
    let mut output = vec![Complex::new(0.0, 0.0); input.len()];

    p.process_immutable_with_scratch(input, &mut output, &mut scratch);
    output
}
