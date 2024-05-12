//! main structure tests

// ------ IMPORTS

use crate::{ComputeMethod, DomainDescriptor, FunctionDescriptor, Integraal, IntegraalError};

// ------ CONTENT

// test utils

const RECTANGLE_TOLERANCE: f64 = 1e-5;
const TRAPEZOID_TOLERANCE: f64 = 1e-5;
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
        let $c = ComputeMethod::RectangleLeft;
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

#[test]
fn inconsistent_parameters() {
    let method = ComputeMethod::RectangleLeft;
    let function = FunctionDescriptor::Values(vec![1., 1., 1., 1., 1., 1.]);
    let domain = vec![0.0, 0.1, 0.2, 0.3, 0.4]; // missing the last x value
    let domain = DomainDescriptor::Explicit(&domain);

    let mut integral = Integraal::default();
    integral.method(method).function(function).domain(domain);
    assert_eq!(
        integral.compute(),
        Err(IntegraalError::InconsistentParameters(
            "provided function and domain value slices have different lengthes"
        ))
    );

    // this is equivalent to the first domain
    let domain = DomainDescriptor::Uniform {
        start: 0.,
        step: 0.1,
        n_step: 5,
    };
    let function = FunctionDescriptor::Values(vec![1., 1., 1., 1., 1., 1.]);

    let mut integral = Integraal::default();
    integral.method(method).function(function).domain(domain);
    assert_eq!(
        integral.compute(),
        Err(IntegraalError::InconsistentParameters(
            "provided function and domain value slices have different lengthes"
        ))
    );
}

// correct usages

// test are groups per module according to the integral & the computation method
// test names follow this pattern:
// <FunctionDescriptorEnum><DomainDescriptorEnum>

// integral A
// y = f(x) = sin(x) from 0 to PI

macro_rules! generate_test {
    ($name: ident, $dm: stmt, $fnd: expr, $dmd: expr, $met: expr, $tol: ident) => {
        #[allow(non_snake_case)]
        #[test]
        fn $name() {
            $dm

            let functiond = $fnd;
            let domaind = $dmd;
            let computem = $met;
            let mut integraal = Integraal::default();
            let res = integraal
                .function(functiond)
                .domain(domaind)
                .method(computem)
                .compute();
            assert!(res.is_ok());
            assert!(
                almost_equal!(res.unwrap(), 2.0, $tol),
                "left: {} \nright: 2.0",
                res.unwrap()
            );
        }
    };
    ($name: ident, $fnd: expr, $dmd: expr, $met: expr, $tol: ident) => {
        #[allow(non_snake_case)]
        #[test]
        fn $name() {
            let functiond = $fnd;
            let domaind = $dmd;
            let computem = $met;
            let mut integraal = Integraal::default();
            let res = integraal
                .function(functiond)
                .domain(domaind)
                .method(computem)
                .compute();
            assert!(res.is_ok());
            assert!(
                almost_equal!(res.unwrap(), 2.0, $tol),
                "left: {} \nright: 2.0",
                res.unwrap()
            );
        }
    };
}

#[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
mod a_rectangleleft {
    use super::*;

    generate_test!(
        ClosureExplicit,
        let domain: Vec<f64> = (0..(std::f64::consts::PI * 1000.) as usize)
            .map(|step_id| step_id as f64 * STEP)
            .collect(),
        FunctionDescriptor::Closure(Box::new(f64::sin)),
        DomainDescriptor::Explicit(&domain),
        ComputeMethod::RectangleLeft,
        RECTANGLE_TOLERANCE
    );

    generate_test!(
        ClosureUniform,
        FunctionDescriptor::Closure(Box::new(f64::sin)),
        DomainDescriptor::Uniform {
            start: 0.,
            step: STEP,
            n_step: (1000. * std::f64::consts::PI) as usize,
        },
        ComputeMethod::RectangleLeft,
        RECTANGLE_TOLERANCE
    );

    generate_test!(
        ValuesExplicit,
        let domain: Vec<f64> = (0..(std::f64::consts::PI * 1000.) as usize)
            .map(|step_id| step_id as f64 * STEP)
            .collect(),
        FunctionDescriptor::Values(domain.iter().copied().map(f64::sin).collect()),
        DomainDescriptor::Explicit(&domain),
        ComputeMethod::RectangleLeft,
        RECTANGLE_TOLERANCE
    );

    generate_test!(
        ValuesUniform,
        FunctionDescriptor::Values(
            (0..(1000. * std::f64::consts::PI) as usize)
                .map(|step_id| (step_id as f64 * STEP).sin())
                .collect()
        ),
        DomainDescriptor::Uniform {
            start: 0.,
            step: STEP,
            n_step: (1000. * std::f64::consts::PI) as usize,
        },
        ComputeMethod::RectangleLeft,
        RECTANGLE_TOLERANCE
    );
}

#[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
mod a_rectangleright {
    use super::*;

    generate_test!(
        ClosureExplicit,
        let domain: Vec<f64> = (0..(std::f64::consts::PI * 1000.) as usize)
            .map(|step_id| step_id as f64 * STEP)
            .collect(),
        FunctionDescriptor::Closure(Box::new(f64::sin)),
        DomainDescriptor::Explicit(&domain),
        ComputeMethod::RectangleRight,
        RECTANGLE_TOLERANCE
    );

    generate_test!(
        ClosureUniform,
        FunctionDescriptor::Closure(Box::new(f64::sin)),
        DomainDescriptor::Uniform {
            start: 0.,
            step: STEP,
            n_step: (1000. * std::f64::consts::PI) as usize,
        },
        ComputeMethod::RectangleRight,
        RECTANGLE_TOLERANCE
    );

    generate_test!(
        ValuesExplicit,
        let domain: Vec<f64> = (0..(std::f64::consts::PI * 1000.) as usize)
            .map(|step_id| step_id as f64 * STEP)
            .collect(),
        FunctionDescriptor::Values(domain.iter().copied().map(f64::sin).collect()),
        DomainDescriptor::Explicit(&domain),
        ComputeMethod::RectangleRight,
        RECTANGLE_TOLERANCE
    );

    generate_test!(
        ValuesUniform,
        FunctionDescriptor::Values(
            (0..(1000. * std::f64::consts::PI) as usize)
                .map(|step_id| (step_id as f64 * STEP).sin())
                .collect()
        ),
        DomainDescriptor::Uniform {
            start: 0.,
            step: STEP,
            n_step: (1000. * std::f64::consts::PI) as usize,
        },
        ComputeMethod::RectangleRight,
        RECTANGLE_TOLERANCE
    );
}

#[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
mod a_trapezoid {
    use super::*;

    generate_test!(
        ClosureExplicit,
        let domain: Vec<f64> = (0..(std::f64::consts::PI * 1000.) as usize)
            .map(|step_id| step_id as f64 * STEP)
            .collect(),
        FunctionDescriptor::Closure(Box::new(f64::sin)),
        DomainDescriptor::Explicit(&domain),
        ComputeMethod::Trapezoid,
        TRAPEZOID_TOLERANCE
    );

    generate_test!(
        ClosureUniform,
        FunctionDescriptor::Closure(Box::new(f64::sin)),
        DomainDescriptor::Uniform {
            start: 0.,
            step: STEP,
            n_step: (1000. * std::f64::consts::PI) as usize,
        },
        ComputeMethod::Trapezoid,
        TRAPEZOID_TOLERANCE
    );

    generate_test!(
        ValuesExplicit,
        let domain: Vec<f64> = (0..(std::f64::consts::PI * 1000.) as usize)
            .map(|step_id| step_id as f64 * STEP)
            .collect(),
        FunctionDescriptor::Values(domain.iter().copied().map(f64::sin).collect()),
        DomainDescriptor::Explicit(&domain),
        ComputeMethod::Trapezoid,
        TRAPEZOID_TOLERANCE
    );

    generate_test!(
        ValuesUniform,
        FunctionDescriptor::Values(
            (0..(1000. * std::f64::consts::PI) as usize)
                .map(|step_id| (step_id as f64 * STEP).sin())
                .collect()
        ),
        DomainDescriptor::Uniform {
            start: 0.,
            step: STEP,
            n_step: (1000. * std::f64::consts::PI) as usize,
        },
        ComputeMethod::Trapezoid,
        TRAPEZOID_TOLERANCE
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
        .map(|step_id| -1. + f64::from(step_id) * STEP)
        .collect();
    let domaind = DomainDescriptor::Explicit(&domain);
    let computem = ComputeMethod::RectangleLeft;
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
