fn main() {
    // describe Xs
    // 100_001 samples
    let step = 0.00001;
    let pre_args: Vec<f64> = (0..100_001).map(|step_id| step * step_id as f64).collect();

    // describe Ys
    let closure = |x: f64| 2.0 * x;

    // compute Ys from Xs
    // 100_001 samples yield 100_000 trapezoids
    let args = (0..100_000).map(|idx| (pre_args[idx], pre_args[idx + 1]));
    let rets = args.map(|(left_x, right_x)| (closure(left_x), closure(right_x)));

    // compute trapezoid areas as rectangle + triangle
    let areas =
        rets.map(|(left_y, right_y)| step * (left_y.min(right_y) + (left_y - right_y).abs() / 2.0));

    // reduce & print result
    let res: f64 = areas.sum();
    println!("integral value of f(x) = 2 * x over [0; 1]: {res}");
}
