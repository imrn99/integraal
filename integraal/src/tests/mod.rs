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

macro_rules! all_tests {
    (
      $ft: ty,           // float type
      $dm: stmt,         // domain
      $fnd_cls: expr,    // function descriptor (closure)
      $fnd_val: expr,    // function descriptor (values)
      $dmd_xpl: expr,    // domain descriptor (explicit)
      $dmd_uni: expr,    // domain descriptor (uniform)
      // $met: expr,     // compute method
      // $res: ident,    // expected result
      // $tol: ident     // tolerance
    ) => {
        #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
        mod rectangle_left {
            use super::*;

            generate_test!(
                $ft,
                ClosureExplicit,
                $dm,
                $fnd_cls,
                $dmd_xpl,
                ComputeMethod::RectangleLeft,
                RES,
                RECTANGLE_TOLERANCE
            );

            generate_test!(
                $ft,
                ClosureUniform,
                $fnd_cls,
                $dmd_uni,
                ComputeMethod::RectangleLeft,
                RES,
                RECTANGLE_TOLERANCE
            );

            generate_test!(
                $ft,
                ValuesExplicit,
                $dm,
                $fnd_val,
                $dmd_xpl,
                ComputeMethod::RectangleLeft,
                RES,
                RECTANGLE_TOLERANCE
            );

            generate_test!(
                $ft,
                ValuesUniform,
                $fnd_val,
                $dmd_uni,
                ComputeMethod::RectangleLeft,
                RES,
                RECTANGLE_TOLERANCE
            );
        }

        #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
        mod rectangle_right {
            use super::*;

            generate_test!(
                $ft,
                ClosureExplicit,
                $dm,
                $fnd_cls,
                $dmd_xpl,
                ComputeMethod::RectangleRight,
                RES,
                RECTANGLE_TOLERANCE
            );

            generate_test!(
                $ft,
                ClosureUniform,
                $fnd_cls,
                $dmd_uni,
                ComputeMethod::RectangleRight,
                RES,
                RECTANGLE_TOLERANCE
            );

            generate_test!(
                $ft,
                ValuesExplicit,
                $dm,
                $fnd_val,
                $dmd_xpl,
                ComputeMethod::RectangleRight,
                RES,
                RECTANGLE_TOLERANCE
            );

            generate_test!(
                $ft,
                ValuesUniform,
                $fnd_val,
                $dmd_uni,
                ComputeMethod::RectangleRight,
                RES,
                RECTANGLE_TOLERANCE
            );
        }

        #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
        mod trapezoid {
            use super::*;

            generate_test!(
                $ft,
                ClosureExplicit,
                $dm,
                $fnd_cls,
                $dmd_xpl,
                ComputeMethod::Trapezoid,
                RES,
                TRAPEZOID_TOLERANCE
            );

            generate_test!(
                $ft,
                ClosureUniform,
                $fnd_cls,
                $dmd_uni,
                ComputeMethod::Trapezoid,
                RES,
                TRAPEZOID_TOLERANCE
            );

            generate_test!(
                $ft,
                ValuesExplicit,
                $dm,
                $fnd_val,
                $dmd_xpl,
                ComputeMethod::Trapezoid,
                RES,
                TRAPEZOID_TOLERANCE
            );

            generate_test!(
                $ft,
                ValuesUniform,
                $fnd_val,
                $dmd_uni,
                ComputeMethod::Trapezoid,
                RES,
                TRAPEZOID_TOLERANCE
            );
        }

        #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
        mod simpson {
            use super::*;

            generate_test!(
                $ft,
                ClosureExplicit,
                $dm,
                $fnd_cls,
                $dmd_xpl,
                ComputeMethod::Simpson,
                RES,
                SIMPSON_TOLERANCE
            );

            generate_test!(
                $ft,
                ClosureUniform,
                $fnd_cls,
                $dmd_uni,
                ComputeMethod::Simpson,
                RES,
                SIMPSON_TOLERANCE
            );

            generate_test!(
                $ft,
                ValuesExplicit,
                $dm,
                $fnd_val,
                $dmd_xpl,
                ComputeMethod::Simpson,
                RES,
                SIMPSON_TOLERANCE
            );

            generate_test!(
                $ft,
                ValuesUniform,
                $fnd_val,
                $dmd_uni,
                ComputeMethod::Simpson,
                RES,
                SIMPSON_TOLERANCE
            );
        }

        #[cfg(feature = "boole")]
        #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
        mod boole {
            use super::*;

            generate_test!(
                $ft,
                ClosureUniform,
                $fnd_cls,
                $dmd_uni,
                ComputeMethod::Boole { force: true },
                RES,
                BOOLE_TOLERANCE
            );

            generate_test!(
                $ft,
                ValuesUniform,
                $fnd_val,
                $dmd_uni,
                ComputeMethod::Boole { force: true },
                RES,
                BOOLE_TOLERANCE
            );
        }

        #[cfg(feature = "romberg")]
        #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
        mod romberg {
            use super::*;

            generate_test!(
                $ft,
                ClosureUniform,
                $fnd_cls,
                $dmd_uni,
                ComputeMethod::Romberg { max_steps: 10 },
                RES,
                ROMBERG_TOLERANCE
            );

            generate_test!(
                $ft,
                ValuesUniform,
                $fnd_val,
                $dmd_uni,
                ComputeMethod::Romberg { max_steps: 10 },
                RES,
                ROMBERG_TOLERANCE
            );
        }

        #[cfg(feature = "montecarlo")]
        #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
        mod montecarlo {
            use super::*;

            generate_test!(
                $ft,
                ClosureExplicit,
                $dm,
                $fnd_cls,
                $dmd_xpl,
                ComputeMethod::MonteCarlo { n_sample: 100 },
                RES,
                MONTECARLO_TOLERANCE
            );

            generate_test!(
                $ft,
                ClosureUniform,
                $fnd_cls,
                $dmd_uni,
                ComputeMethod::MonteCarlo { n_sample: 100 },
                RES,
                MONTECARLO_TOLERANCE
            );

            generate_test!(
                $ft,
                ValuesExplicit,
                $dm,
                $fnd_val,
                $dmd_xpl,
                ComputeMethod::MonteCarlo { n_sample: 100 },
                RES,
                MONTECARLO_TOLERANCE
            );

            generate_test!(
                $ft,
                ValuesUniform,
                $fnd_val,
                $dmd_uni,
                ComputeMethod::MonteCarlo { n_sample: 100 },
                RES,
                MONTECARLO_TOLERANCE
            );
        }
    };
}

pub(crate) use {all_tests, almost_equal, generate_test};
