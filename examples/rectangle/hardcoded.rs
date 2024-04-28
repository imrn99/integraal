fn main() {
    // describe Xs
    // 100_001 samples
    let step = 0.00001;
    let args = (1..100001).map(|step_id| step * step_id as f64);

    // describe Ys
    let closure = |x: f64| 2.0 * x;

    // compute Ys from Xs
    // 100_001 samples yields 100_001 rectangle
    let rets = args.map(closure);

    // compute rectangle areas
    let areas = rets.map(|ret| ret * step);

    // reduce & print result
    let res: f64 = areas.sum();
    println!("integral value of f(x) = 2 * x over [0; 1]: {res}");
}
