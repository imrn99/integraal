use integraal::{ComputeMethod, DomainDescriptor, FunctionDescriptor, Integraal};

fn main() {
    // describe domain, function & computation method
    let domain = DomainDescriptor::Uniform {
        start: 0.0,
        step: 0.00001,
        n_step: 100_001,
    };
    let function = FunctionDescriptor::Closure {
        closure: Box::new(|x: f64| 2.0 * x),
    };
    let method = ComputeMethod::Rectangle;

    // build the integral
    let integral = Integraal::default()
        .domain(domain)
        .function(function)
        .method(method);

    // compute & print
    let res: f64 = integral.compute().unwrap();
    println!("integral value of f(x) = 2 * x over [0; 1]: {res}");
}
