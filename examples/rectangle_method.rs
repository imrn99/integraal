fn main() {
    let closure = |x: f64| 2.0 * x;
    let step = 0.00001;
    // compute integral over the [0.0; 1.0] range using the right endpoint for rectangles
    // compute Xs
    let args = (1..100001).map(|step_id| step * step_id as f64);
    // compute Ys from Xs
    let rets = args.map(closure);
    // compute rectangle areas
    let rectangles = rets.map(|ret| ret * step);
    // reduce & print result
    let res: f64 = rectangles.sum();
    println!("integral value of f(x) = 2 * x over [0; 1]: {res}");
}
