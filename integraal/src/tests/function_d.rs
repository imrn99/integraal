// ------ IMPORTS

use super::*;

// ------ TESTS

// integral D (even)
// y = f(x) = x^2 from -4 to 4
// expected value = 128/3

mod double {
    use super::*;
    const STEP: f64 = 0.001;
    const RES: f64 = 128.0 / 3.0;

    const RECTANGLE_TOLERANCE: f64 = 1e-5;
    const TRAPEZOID_TOLERANCE: f64 = 1e-5;

    all_tests!(
        f64,
        let domain: Vec<f64> = (-4000..4000)
            .map(|step_id| f64::from(step_id) * STEP)
            .collect(),
        FunctionDescriptor::Closure(Box::new(|x: f64| x.powi(2))),
        FunctionDescriptor::Values(
            (-4000..4000)
                .map(|step_id| (f64::from(step_id) * STEP).powi(2))
                .collect()
        ),
        DomainDescriptor::Explicit(&domain),
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
    const RES: f32 = 128.0 / 3.0;

    const RECTANGLE_TOLERANCE: f32 = 1e-5;
    const TRAPEZOID_TOLERANCE: f32 = 1e-5;

    all_tests!(
        f32,
        let domain: Vec<f32> = (-4000..4000)
            .map(|step_id| step_id as f32 * STEP)
            .collect(),
        FunctionDescriptor::Closure(Box::new(|x: f32| x.powi(2))),
        FunctionDescriptor::Values(
            (-4000..4000)
                .map(|step_id| (step_id as f32 * STEP).powi(2))
                .collect()
        ),
        DomainDescriptor::Explicit(&domain),
        DomainDescriptor::Uniform {
            start: -4.,
            step: STEP,
            n_step: 8000,
        },
    );
}
