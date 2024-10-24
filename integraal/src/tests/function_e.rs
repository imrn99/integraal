// ------ IMPORTS

use super::*;

// ------ TESTS

// integral E (odd)
// y = f(x) = x^3 from -4 to 4
// expected value = 0

mod double {
    use super::*;
    const STEP: f64 = 0.001;
    const RES: f64 = 0.0;

    // FIXME: tolerances need to be updated
    const RECTANGLE_TOLERANCE: f64 = 1e-5;
    const TRAPEZOID_TOLERANCE: f64 = 1e-5;
    const SIMPSON_TOLERANCE: f64 = 1e-5;
    const BOOLE_TOLERANCE: f64 = 1e-5;
    const ROMBERG_TOLERANCE: f64 = 1e-5;
    const MONTECARLO_TOLERANCE: f64 = 1e-5;

    all_tests!(
        f64,
        FunctionDescriptor::Closure(Box::new(|x: f64| x.powi(2))),
        FunctionDescriptor::Values(
            (-4000..4000)
                .map(|step_id| (f64::from(step_id) * STEP).powi(2))
                .collect()
        ),
        DomainDescriptor::Explicit(
            (-4000..4000)
                .map(|step_id| f64::from(step_id) * STEP)
                .collect()
        ),
        DomainDescriptor::Uniform {
            start: -4.,
            step: STEP,
            n_step: 8000,
        },
    );
}

mod simple {
    use super::*;
    const STEP: f32 = 0.001;
    const RES: f32 = 0.0;

    // FIXME: tolerances need to be updated
    const RECTANGLE_TOLERANCE: f32 = 1e-5;
    const TRAPEZOID_TOLERANCE: f32 = 1e-5;
    const SIMPSON_TOLERANCE: f32 = 1e-5;
    const BOOLE_TOLERANCE: f32 = 1e-5;
    const ROMBERG_TOLERANCE: f32 = 1e-5;
    const MONTECARLO_TOLERANCE: f32 = 1e-5;

    all_tests!(
        f32,
        FunctionDescriptor::Closure(Box::new(|x: f32| x.powi(2))),
        FunctionDescriptor::Values(
            (-4000..4000)
                .map(|step_id| (step_id as f32 * STEP).powi(2))
                .collect()
        ),
        DomainDescriptor::Explicit((-4000..4000).map(|step_id| step_id as f32 * STEP).collect()),
        DomainDescriptor::Uniform {
            start: -4.,
            step: STEP,
            n_step: 8000,
        },
    );
}
