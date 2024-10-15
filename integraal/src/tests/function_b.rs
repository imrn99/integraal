// ------ IMPORTS

use super::*;

// ------ TESTS

// integral B (contiguous, non-contiguous derivative)
// y = f(x) = |  x     from 0 to 1
//            |  1     from 1 to 2
//            | -x + 3 from 2 to 3

mod double {
    use super::*;
    const STEP: f64 = 0.001;
    const RES: f64 = 2.0;

    // FIXME: tolerances need to be updated
    const RECTANGLE_TOLERANCE: f64 = 1e-5;
    const TRAPEZOID_TOLERANCE: f64 = 1e-5;
    const SIMPSON_TOLERANCE: f64 = 1e-5;
    const BOOLE_TOLERANCE: f64 = 1e-5;
    const ROMBERG_TOLERANCE: f64 = 1e-5;
    const MONTECARLO_TOLERANCE: f64 = 1e-5;

    all_tests!(
        f64,
        let domain: Vec<f64> = (0..3000)
            .map(|step_id| f64::from(step_id) * STEP)
            .collect(),
        FunctionDescriptor::Closure(
            Box::new(|x|
                if      (0.0..1.0).contains(&x) { x }
                else if (1.0..2.0).contains(&x) { 1.0 }
                else if (2.0..3.0).contains(&x) { 3.0 - x }
                else { 0.0 })
        ),
        FunctionDescriptor::Values(
            (0..3000)
                .map(|step_id| {
                    let x = f64::from(step_id) * STEP;
                    if      (0.0..1.0).contains(&x) { x }
                    else if (1.0..2.0).contains(&x) { 1.0 }
                    else if (2.0..3.0).contains(&x) { 3.0 - x }
                    else { 0.0 }
                })
                .collect()
        ),
        DomainDescriptor::Explicit(&domain),
        DomainDescriptor::Uniform {
            start: 0.,
            step: STEP,
            n_step: 3000,
        },
    );
}

mod simple {
    use super::*;
    const STEP: f32 = 0.001;
    const RES: f32 = 2.0;

    // FIXME: tolerances need to be updated
    const RECTANGLE_TOLERANCE: f32 = 1e-5;
    const TRAPEZOID_TOLERANCE: f32 = 1e-5;
    const SIMPSON_TOLERANCE: f32 = 1e-5;
    const BOOLE_TOLERANCE: f32 = 1e-5;
    const ROMBERG_TOLERANCE: f32 = 1e-5;
    const MONTECARLO_TOLERANCE: f32 = 1e-5;

    all_tests!(
        f32,
        let domain: Vec<f32> = (0..3000)
            .map(|step_id| step_id as f32 * STEP)
            .collect(),
        FunctionDescriptor::Closure(
            Box::new(|x: f32|
                if x < 1.0 { x }
                else if x < 2.0 { 1.0 }
                else if x < 3.0 { 3.0 - x }
                else { 0.0 })
        ),
        FunctionDescriptor::Values(
            (0..3000)
                .map(|step_id| {
                    let x = step_id as f32 * STEP;
                    if x < 1.0 { x }
                    else if x < 2.0 { 1.0 }
                    else if x < 3.0 { 3.0 - x }
                    else { 0.0 }
                })
                .collect()
        ),
        DomainDescriptor::Explicit(&domain),
        DomainDescriptor::Uniform {
            start: 0.,
            step: STEP,
            n_step: 3000,
        },
    );
}
