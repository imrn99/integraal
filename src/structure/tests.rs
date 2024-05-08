//! main structure tests

// ------ IMPORTS

use crate::{ComputeMethod, DomainDescriptor, FunctionDescriptor, Integraal};

// ------ CONTENT

// test utils

const RECTANGLE_TOLERANCE: f64 = 1e-5;
const STEP: f64 = 0.001;

macro_rules! almost_equal {
    ($v1: expr, $v2: expr, $tol: ident) => {
        ($v1 - $v2).abs() < $tol
    };
}

// incorrect usages

#[test]
#[should_panic(expected = "a")]
fn basic() {
    assert_eq!(1 + 1, 3);
}

// correct usages
// test names follow this pattern:
// <integral>_<FunctionDescriptorEnum>_<DomainDescriptorEnum>_<ComputeMethodEnum>

// integral A
// y = f(x) = sin(x) from 0 to PI

#[allow(non_snake_case)]
#[test]
fn A_Closure_Explicit_Rectangle() {
    let functiond = FunctionDescriptor::Closure(Box::new(|x| x.sin()));
    let domain: Vec<f64> = (0..(std::f64::consts::PI * 1000.) as usize)
        .map(|step_id| step_id as f64 * STEP)
        .collect();
    let domaind = DomainDescriptor::Explicit(&domain);
    let computem = ComputeMethod::Rectangle;
    let mut integraal = Integraal::default();
    let res = integraal
        .function(functiond)
        .domain(domaind)
        .method(computem)
        .compute();
    assert!(res.is_ok());
    assert!(
        almost_equal!(res.unwrap(), 2.0, RECTANGLE_TOLERANCE),
        "left: {} \nright: 2.0",
        res.unwrap()
    );
}

// integral B
// y = f(x) = x from -1 to 1

#[allow(non_snake_case)]
#[test]
fn B_Closure_Explicit_Rectangle() {
    let functiond = FunctionDescriptor::Closure(Box::new(|x| x));
    // -1 to 1, with .001 steps
    // currently requires one more value because of
    // the inconsistent sampling policy
    let domain: Vec<f64> = (0..=2001)
        .map(|step_id| -1. + step_id as f64 * STEP)
        .collect();
    let domaind = DomainDescriptor::Explicit(&domain);
    let computem = ComputeMethod::Rectangle;
    let mut integraal = Integraal::default();
    let res = integraal
        .function(functiond)
        .domain(domaind)
        .method(computem)
        .compute();
    assert!(res.is_ok());
    assert!(
        almost_equal!(res.unwrap(), 0.0, RECTANGLE_TOLERANCE),
        "left: {} \nright: 0.0",
        res.unwrap()
    );
}
