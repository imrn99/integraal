fn main() {
    let closure = |x: f64| 2.0 * x;
    let step = 0.00001;
    // compute integral over the [0.0; 1.0] range using the right endpoint for rectangles
    // compute Xs
    let pre_args: Vec<f64> = (0..100001).map(|step_id| step * step_id as f64).collect();
    let args = (0..100000).map(|idx| (pre_args[idx], pre_args[idx + 1]));
    // compute Ys from Xs
    let rets = args.map(|(left_x, right_x)| (closure(left_x), closure(right_x)));
    // compute trapezoid areas as rectangle + triangle
    let rectangles =
        rets.map(|(left_y, right_y)| step * (left_y.min(right_y) + (left_y - right_y).abs() / 2.0));
    // reduce & print result
    let res: f64 = rectangles.sum();
    println!("integral value of f(x) = 2 * x over [0; 1]: {res}");
}
