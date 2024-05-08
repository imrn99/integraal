//! main structure tests

// ------ IMPORTS

use crate::{ComputeMethod, DomainDescriptor, FunctionDescriptor, Integraal, IntegraalError};

// ------ CONTENT

// test utils

const RECTANGLE_TOLERANCE: f64 = 1e-5;
const STEP: f64 = 0.001;

macro_rules! almost_equal {
    ($v1: expr, $v2: expr, $tol: ident) => {
        ($v1 - $v2).abs() < $tol
    };
}

macro_rules! generate_sample_descriptors {
    ($f: ident, $d: ident, $c: ident) => {
        let $f = FunctionDescriptor::Closure(Box::new(|x| x));
        let $d = DomainDescriptor::Explicit(&[]);
        let $c = ComputeMethod::Rectangle;
    };
}

macro_rules! generate_missing {
    ($a: ident, $b: ident) => {
        let mut integral = Integraal::default();
        integral.$a($a).$b($b);
        assert_eq!(
            integral.compute(),
            Err(IntegraalError::MissingParameters(
                "one or more parameter is missing"
            ))
        );
    };
}

// incorrect usages

#[allow(unused_variables)]
#[test]
fn missing_parameters() {
    // missing function descriptor
    generate_sample_descriptors!(function, domain, method);
    generate_missing!(domain, method);

    // missing domain descriptor
    generate_sample_descriptors!(function, domain, method);
    generate_missing!(function, method);

    // missing compute method
    generate_sample_descriptors!(function, domain, method);
    generate_missing!(function, domain);

    // missing all but one
    let mut integral = Integraal::default();
    integral.method(method);
    assert_eq!(
        integral.compute(),
        Err(IntegraalError::MissingParameters(
            "one or more parameter is missing"
        ))
    );
}

// correct usages
// test names follow this pattern:
// <integral>_<FunctionDescriptorEnum>_<DomainDescriptorEnum>_<ComputeMethodEnum>

// integral A
// y = f(x) = sin(x) from 0 to PI

#[allow(non_snake_case)]
#[test]
fn A_Closure_Explicit_Rectangle() {
    let functiond = FunctionDescriptor::Closure(Box::new(f64::sin));
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
    // FIXME
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