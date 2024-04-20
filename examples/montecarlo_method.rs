use rand::Rng;

fn main() {
    let closure = |x: f64| 2.0 * x;
    const N_STEP: usize = 100_000;
    const N_SAMPLE: usize = 100;
    const STEP: f64 = 0.00001;

    let args = (0..=N_STEP).map(|step_id| STEP * step_id as f64);
    let rets = args.map(closure);
    let count: usize = rets
        .map(|y| {
            let mut rng = rand::thread_rng();
            let iter = (&mut rng)
                .sample_iter::<f64, _>(rand::distributions::Uniform::new(0.0, 2.0))
                .take(N_SAMPLE);
            iter.filter(|sample| *sample <= y).count()
        })
        .sum::<usize>();
    // reduce & print result
    let res = (count as f64 / (N_STEP * N_SAMPLE) as f64) * 1.0 * 2.0;
    println!("integral value of f(x) = 2 * x over [0; 1]: {res}");
}
