// ------ IMPORTS

use super::*;

// ------ TESTS

// integral A
// y = f(x) = sin(x) from 0 to PI

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
        let domain: Vec<f64> = (0..(std::f64::consts::PI * 1000.) as usize)
            .map(|step_id| step_id as f64 * STEP)
            .collect(),
        FunctionDescriptor::Closure(Box::new(f64::sin)),
        FunctionDescriptor::Values(
            (0..(1000. * std::f64::consts::PI) as usize)
                .map(|step_id| (step_id as f64 * STEP).sin())
                .collect()
        ),
        DomainDescriptor::Explicit(&domain),
        DomainDescriptor::Uniform {
            start: 0.,
            step: STEP,
            n_step: (1000. * std::f64::consts::PI) as usize,
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
        let domain: Vec<f32> = (0..(std::f32::consts::PI * 1000.) as usize)
            .map(|step_id| step_id as f32 * STEP)
            .collect(),
        FunctionDescriptor::Closure(Box::new(f32::sin)),
        FunctionDescriptor::Values(
            (0..(1000. * std::f32::consts::PI) as usize)
                .map(|step_id| (step_id as f32 * STEP).sin())
                .collect()
        ),
        DomainDescriptor::Explicit(&domain),
        DomainDescriptor::Uniform {
            start: 0.,
            step: STEP,
            n_step: (1000. * std::f32::consts::PI) as usize,
        },
    );
}
