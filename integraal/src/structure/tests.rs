//! main structure tests

// ------ IMPORTS

use crate::{ComputeMethod, DomainDescriptor, FunctionDescriptor, Integraal, IntegraalError};

// ------ CONTENT

// test utils

const RECTANGLE_TOLERANCE: f64 = 1e-5;
const TRAPEZOID_TOLERANCE: f64 = 1e-5;
const STEP: f64 = 0.001;

macro_rules! generate_sample_descriptors {
    ($f: ident, $d: ident, $c: ident) => {
        let $f: FunctionDescriptor<f64> = FunctionDescriptor::Closure(Box::new(|x| x));
        let $d: DomainDescriptor<'_, f64> = DomainDescriptor::Explicit(&[]);
        let $c: ComputeMethod = ComputeMethod::RectangleLeft;
    };
}

macro_rules! generate_missing {
    ($a: ident, $b: ident) => {
        let mut integral: Integraal<'_, f64> = Integraal::default().$a($a).$b($b);
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
    let mut integral: Integraal<'_, f64> = Integraal::default().method(method);
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

    let mut integral = Integraal::default()
        .method(method)
        .function(function)
        .domain(domain);
    assert_eq!(
        integral.compute(),
        Err(IntegraalError::InconsistentParameters(
            "function and domain value slices have different lengthes"
        ))
    );

    // this is equivalent to the first domain
    let domain = DomainDescriptor::Uniform {
        start: 0.,
        step: 0.1,
        n_step: 5,
    };
    let function = FunctionDescriptor::Values(vec![1., 1., 1., 1., 1., 1.]);

    let mut integral = Integraal::default()
        .method(method)
        .function(function)
        .domain(domain);
    assert_eq!(
        integral.compute(),
        Err(IntegraalError::InconsistentParameters(
            "provided function and domain value slices have different lengthes"
        ))
    );
}

// correct usages

// works for F: Float
macro_rules! almost_equal {
    ($ft: ty, $v1: expr, $v2: expr) => {
        ($v1 - $v2).abs() < ($v1.abs() + $v2.abs()).min(<$ft as num_traits::Float>::max_value())
    };
    ($ft: ty,$v1: expr, $v2: expr, $tol: ident) => {
        ($v1 - $v2).abs()
            < ($v1.abs() + $v2.abs()).min(<$ft as num_traits::Float>::max_value()) + $tol
    };
}

// test are groups per module according to the integral & the computation method
// test names follow this pattern:
// <FunctionDescriptorEnum><DomainDescriptorEnum>

// integral A
// y = f(x) = sin(x) from 0 to PI
// expected value = 2

macro_rules! generate_test {
    ($name: ident, $dm: stmt, $fnd: expr, $dmd: expr, $met: expr, $tol: ident) => {
        #[allow(non_snake_case)]
        #[test]
        fn $name() {
            $dm

            let functiond = $fnd;
            let domaind = $dmd;
            let computem = $met;
            let mut integraal = Integraal::default()
                .function(functiond)
                .domain(domaind)
                .method(computem);
            let res = integraal.compute();
            assert!(res.is_ok());
            assert!(
                almost_equal!(f64, res.unwrap(), 2.0_f64, $tol),
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
            let mut integraal = Integraal::default()
                .function(functiond)
                .domain(domaind)
                .method(computem);
            let res = integraal.compute().unwrap();
            assert!(
                almost_equal!(f64, res, 2.0_f64, $tol),
                "computed value: {res:?}\nexpected value: 2.0\ntolerance: {:?}",
                $tol
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

#[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
mod a_simpson {
    use super::*;

    generate_test!(
        ClosureExplicit,
        let domain: Vec<f64> = (0..(std::f64::consts::PI * 1000.) as usize)
            .map(|step_id| step_id as f64 * STEP)
            .collect(),
        FunctionDescriptor::Closure(Box::new(f64::sin)),
        DomainDescriptor::Explicit(&domain),
        ComputeMethod::Simpson,
        TRAPEZOID_TOLERANCE // FIXME: update tol
    );

    generate_test!(
        ClosureUniform,
        FunctionDescriptor::Closure(Box::new(f64::sin)),
        DomainDescriptor::Uniform {
            start: 0.,
            step: STEP,
            n_step: (1000. * std::f64::consts::PI) as usize,
        },
        ComputeMethod::Simpson,
        TRAPEZOID_TOLERANCE // FIXME: update tol
    );

    generate_test!(
        ValuesExplicit,
        let domain: Vec<f64> = (0..(std::f64::consts::PI * 1000.) as usize)
            .map(|step_id| step_id as f64 * STEP)
            .collect(),
        FunctionDescriptor::Values(domain.iter().copied().map(f64::sin).collect()),
        DomainDescriptor::Explicit(&domain),
        ComputeMethod::Simpson,
        TRAPEZOID_TOLERANCE // FIXME: update tol
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
        ComputeMethod::Simpson,
        TRAPEZOID_TOLERANCE // FIXME: update tol
    );
}

#[cfg(feature = "boole")]
#[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
mod a_boole {
    use super::*;

    generate_test!(
        ClosureUniform,
        FunctionDescriptor::Closure(Box::new(f64::sin)),
        DomainDescriptor::Uniform {
            start: 0.,
            step: STEP,
            n_step: (1000. * std::f64::consts::PI) as usize - 1,
        },
        ComputeMethod::Boole,
        TRAPEZOID_TOLERANCE // FIXME: update tol
    );

    generate_test!(
        ValuesUniform,
        FunctionDescriptor::Values(
            (0..(1000. * std::f64::consts::PI) as usize - 1)
                .map(|step_id| (step_id as f64 * STEP).sin())
                .collect()
        ),
        DomainDescriptor::Uniform {
            start: 0.,
            step: STEP,
            n_step: (1000. * std::f64::consts::PI) as usize - 1,
        },
        ComputeMethod::Boole,
        TRAPEZOID_TOLERANCE // FIXME: update tol
    );
}

#[cfg(feature = "romberg")]
#[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
mod a_romberg {
    use super::*;

    generate_test!(
        ClosureUniform,
        FunctionDescriptor::Closure(Box::new(f64::sin)),
        DomainDescriptor::Uniform {
            start: 0.,
            step: STEP,
            n_step: (1000. * std::f64::consts::PI) as usize,
        },
        ComputeMethod::Romberg { max_steps: 10 },
        TRAPEZOID_TOLERANCE // FIXME: update tol
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
        ComputeMethod::Romberg { max_steps: 10 },
        TRAPEZOID_TOLERANCE // FIXME: update tol
    );
}

// integral B (contiguous, non-contiguous derivative)
// y = f(x) = |  x     from 0 to 1
//            |  1     from 1 to 2
//            | -x + 3 from 2 to 3
// expected value = 2

mod b_rectangleleft {}

mod b_rectangleright {}

mod b_trapezoid {}

mod b_simpson {}

#[cfg(feature = "boole")]
mod b_boole {}

#[cfg(feature = "romberg")]
mod b_romberg {}

// integral C (piece-wise contiguous)
// y = f(x) = | 0 from 0 to 1
//            | 1 from 1 to 1.5
//            | 0 from 1.5 to 2
// expected value = 0.5

mod c_rectangleleft {}

mod c_rectangleright {}

mod c_trapezoid {}

mod c_simpson {}

#[cfg(feature = "boole")]
mod c_boole {}

#[cfg(feature = "romberg")]
mod c_romberg {}

// integral D (even)
// y = f(x) = x^2 from -4 to 4
// expected value = 128/3

mod d_rectangleleft {}

mod d_rectangleright {}

mod d_trapezoid {}

mod d_simpson {}

#[cfg(feature = "boole")]
mod d_boole {}

#[cfg(feature = "romberg")]
mod d_romberg {}

// integral E (odd)
// y = f(x) = x^3 from -4 to 4
// expected value = 0

mod e_rectangleleft {}

mod e_rectangleright {}

mod e_trapezoid {}

mod e_simpson {}

#[cfg(feature = "boole")]
mod e_boole {}

#[cfg(feature = "romberg")]
mod e_romberg {}
