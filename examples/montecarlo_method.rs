use rand::Rng;

fn main() {
    // describe Xs
    // 100_001 samples
    const N_SAMPLE: usize = 100_000;
    const STEP: f64 = 0.00001;
    let args = (0..=N_SAMPLE).map(|step_id| STEP * step_id as f64);

    // describe Ys
    let closure = |x: f64| 2.0 * x;

    // describe the Monte Carlo process
    const N_MONTECARLO_SAMPLE: usize = 100;
    let mut rng = rand::thread_rng();

    // compute Ys from Xs
    let rets = args.map(closure);

    // compute area from the number of sample under the curve
    let count: usize = rets
        .map(|y| {
            let iter = (&mut rng)
                .sample_iter::<f64, _>(rand::distributions::Uniform::new(0.0, 2.0).unwrap())
                .take(N_MONTECARLO_SAMPLE);
            iter.filter(|sample| *sample <= y).count()
        })
        .sum::<usize>();

    // reduce & print result
    let res = (count as f64 / (N_SAMPLE * N_MONTECARLO_SAMPLE) as f64) * 1.0 * 2.0;
    println!("integral value of f(x) = 2 * x over [0; 1]: {res}");
}
