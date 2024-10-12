// ------ IMPORTS
//
use crate::{ComputeMethod, DomainDescriptor, FunctionDescriptor, Integraal, IntegraalError};

// ------ MODULE DECLARATIONS

// test incorrect usages of the struct
mod incorrect;

// test are groups per module according to the integral & the computation method
// test names follow this pattern:
// <FunctionDescriptorEnum><DomainDescriptorEnum>

// contiguous function
mod function_a;

// contiguous function, non-contiguous derivative
mod function_b;

// piecewise contiguous function
mod function_c;

// even function
mod function_d;

// odd function
mod function_e;

// ------ CONTENT

// test utils

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

pub(crate) use {generate_missing, generate_sample_descriptors};

// works for $ft: Float
macro_rules! almost_equal {
    ($ft: ty, $v1: expr, $v2: expr) => {
        ($v1 - $v2).abs() < ($v1.abs() + $v2.abs()).min(<$ft as num_traits::Float>::max_value())
    };
    ($ft: ty, $v1: expr, $v2: expr, $tol: ident) => {
        ($v1 - $v2).abs()
            < ($v1.abs() + $v2.abs()).min(<$ft as num_traits::Float>::max_value()) + $tol
    };
}

macro_rules! generate_test {
    ($ft: ty, $name: ident, $dm: stmt, $fnd: expr, $dmd: expr, $met: expr, $res: ident, $tol: ident) => {
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
                almost_equal!($ft, res.unwrap(), $res, $tol),
                "left: {} \nright: {}",
                res.unwrap(),
                $res,
            );
        }
    };
    ($ft: ty, $name: ident, $fnd: expr, $dmd: expr, $met: expr, $res: ident, $tol: ident) => {
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
            let res = integraal.compute();
            assert!(res.is_ok());
            assert!(
                almost_equal!($ft, res.unwrap(), $res, $tol),
                "computed value: {res:?}\nexpected value: 2.0\ntolerance: {:?}",
                $tol
            );
        }
    };
}

pub(crate) use {almost_equal, generate_test};
