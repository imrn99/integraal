// we compute the work W associated to the force exerted on a charged particle by an electric field
//
// electric field E(x) = E0 / x
// force exerted  F(x) = q * E0 / x
//
// we compute the work done by this force over the displacement from x=a to x=b with:
//
// - E0 = 100    V/m
// - a  = 0.1    m
// - b  = 0.5    m
// - q  = 1.0e-6 C

use integraal::{ComputeMethod, DomainDescriptor, FunctionDescriptor, Integraal};

const E0: f64 = 100.0;
const Q: f64 = 1.0e-6;
const EXPECTED: f64 = 1.6094379124341003e-4;

fn main() {
    let domain = DomainDescriptor::Uniform {
        start: 0.1,
        step: 1.0e-5,
        n_step: 40_001,
    };
    let function = FunctionDescriptor::Closure(Box::new(|x| Q * E0 / x));
    let method = ComputeMethod::Trapezoid;

    let mut integraal = Integraal::default()
        .domain(domain)
        .function(function)
        .method(method);

    let Ok(res) = integraal.compute() else {
        unreachable!()
    };

    let abs_diff = (res - EXPECTED).abs();
    assert!(abs_diff < 10e-5);

    println!("expected: {:.18}", EXPECTED);
    println!("computed: {:.18}", res);
    println!("diff:    >{:.18}", abs_diff);
}
