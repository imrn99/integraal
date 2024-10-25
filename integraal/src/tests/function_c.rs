// ------ IMPORTS

use super::*;

// ------ TESTS

// integral C (piece-wise contiguous)
// y = f(x) = | 0 from 0 to 1
//            | 1 from 1 to 1.5
//            | 0 from 1.5 to 2

mod double {
    use super::*;
    const STEP: f64 = 0.001;
    const RES: f64 = 0.5;

    // FIXME: tolerances need to be updated
    const RECTANGLE_TOLERANCE: f64 = 1e-5;
    const TRAPEZOID_TOLERANCE: f64 = 1e-5;
    const SIMPSON_TOLERANCE: f64 = 1e-5;
    const BOOLE_TOLERANCE: f64 = 1e-5;
    const ROMBERG_TOLERANCE: f64 = 1e-5;
    const MONTECARLO_TOLERANCE: f64 = 1e-5;

    all_tests!(
        f64,
        FunctionDescriptor::Closure(Box::new(|x| if x < 1.0 {
            0.0
        } else if x < 1.5 {
            1.0
        } else {
            0.0
        })),
        FunctionDescriptor::Values(
            (0..2000)
                .map(|step_id| {
                    let x = f64::from(step_id) * STEP;
                    if x < 1.0 {
                        0.0
                    } else if x < 1.5 {
                        1.0
                    } else {
                        0.0
                    }
                })
                .collect()
        ),
        DomainDescriptor::Explicit((0..2000).map(|step_id| f64::from(step_id) * STEP).collect()),
        DomainDescriptor::Uniform {
            start: 0.,
            step: STEP,
            n_step: 2000,
        },
    );
}

mod simple {
    use super::*;
    const STEP: f32 = 0.001;
    const RES: f32 = 0.5;

    // FIXME: tolerances need to be updated
    const RECTANGLE_TOLERANCE: f32 = 1e-5;
    const TRAPEZOID_TOLERANCE: f32 = 1e-5;
    const SIMPSON_TOLERANCE: f32 = 1e-5;
    const BOOLE_TOLERANCE: f32 = 1e-5;
    const ROMBERG_TOLERANCE: f32 = 1e-5;
    const MONTECARLO_TOLERANCE: f32 = 1e-5;

    all_tests!(
        f32,
        FunctionDescriptor::Closure(Box::new(|x| if x < 1.0 {
            0.0
        } else if x < 1.5 {
            1.0
        } else {
            0.0
        })),
        FunctionDescriptor::Values(
            (0..2000)
                .map(|step_id| {
                    let x = step_id as f32 * STEP;
                    if x < 1.0 {
                        0.0
                    } else if x < 1.5 {
                        1.0
                    } else {
                        0.0
                    }
                })
                .collect()
        ),
        DomainDescriptor::Explicit((0..2000).map(|step_id| step_id as f32 * STEP).collect()),
        DomainDescriptor::Uniform {
            start: 0.,
            step: STEP,
            n_step: 2000,
        },
    );
}
