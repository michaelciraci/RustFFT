// use num_complex::Complex;
// use rustfft::FftPlanner;

// #[test]
// fn test_oop() {
//     let mut p = FftPlanner::<f32>::new();
//     let plan = p.plan_fft_forward(100);
//     let mut input: Vec<Complex<f32>> = (0..100).map(|i| Complex::new(i as f32, 0.0)).collect();

//     let mut output = vec![Complex::<f32>::ZERO; 100];
//     let mut scratch = output.clone();

//     plan.process_outofplace_with_scratch(&mut input, &mut output, &mut scratch);

//     dbg!(output);
//     panic!();
// }
