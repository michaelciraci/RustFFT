use num_complex::Complex;
use rustfft::FftPlanner;

#[test]
fn immutable_0_thru_9() {
    for i in 0..1400 {
        dbg!(i);
        let input = vec![Complex::new(7.0, 8.0); i];

        let mut_output = fft_wrapper_mut(&input);
        let immut_output = fft_wrapper_immut(&input);

        assert_eq!(mut_output, immut_output);
    }
}

#[test]
fn immut_10() {
    let input = vec![Complex::new(7.0, 8.0); 10];

    let mut_output = fft_wrapper_mut(&input);
    let immut_output = fft_wrapper_immut(&input);

    assert_eq!(mut_output, immut_output);
}

fn fft_wrapper_mut(input: &[Complex<f32>]) -> Vec<Complex<f32>> {
    let mut plan = FftPlanner::<f32>::new();
    let p = plan.plan_fft_forward(input.len());

    let mut scratch = vec![Complex::<f32>::ZERO; p.get_inplace_scratch_len()];
    let mut output = input.to_vec();

    p.process_with_scratch(&mut output, &mut scratch);
    output
}

fn fft_wrapper_immut(input: &[Complex<f32>]) -> Vec<Complex<f32>> {
    let mut plan = FftPlanner::<f32>::new();
    let p = plan.plan_fft_forward(input.len());

    let mut scratch = vec![Complex::<f32>::ZERO; p.get_immutable_scratch_len()];
    let mut output = vec![Complex::<f32>::ZERO; input.len()];

    p.process_immutable_with_scratch(input, &mut output, &mut scratch);
    output
}