// ------ IMPORTS

use super::*;

// ------ TESTS

// integral A
// y = f(x) = sin(x) from 0 to PI

const STEP: f64 = 0.001;
const A_RES: f64 = 2.0;

const RECTANGLE_TOLERANCE: f64 = 1e-5;
const TRAPEZOID_TOLERANCE: f64 = 1e-5;

#[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
mod rectangle_left {
    use super::*;

    generate_test!(
        f64,
        ClosureExplicit,
        let domain: Vec<f64> = (0..(std::f64::consts::PI * 1000.) as usize)
            .map(|step_id| step_id as f64 * STEP)
            .collect(),
        FunctionDescriptor::Closure(Box::new(f64::sin)),
        DomainDescriptor::Explicit(&domain),
        ComputeMethod::RectangleLeft,
        A_RES,
        RECTANGLE_TOLERANCE
    );

    generate_test!(
        f64,
        ClosureUniform,
        FunctionDescriptor::Closure(Box::new(f64::sin)),
        DomainDescriptor::Uniform {
            start: 0.,
            step: STEP,
            n_step: (1000. * std::f64::consts::PI) as usize,
        },
        ComputeMethod::RectangleLeft,
        A_RES,
        RECTANGLE_TOLERANCE
    );

    generate_test!(
        f64,
        ValuesExplicit,
        let domain: Vec<f64> = (0..(std::f64::consts::PI * 1000.) as usize)
            .map(|step_id| step_id as f64 * STEP)
            .collect(),
        FunctionDescriptor::Values(domain.iter().copied().map(f64::sin).collect()),
        DomainDescriptor::Explicit(&domain),
        ComputeMethod::RectangleLeft,
        A_RES,
        RECTANGLE_TOLERANCE
    );

    generate_test!(
        f64,
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
        A_RES,
        RECTANGLE_TOLERANCE
    );
}

#[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
mod rectangle_right {
    use super::*;

    generate_test!(
        f64,
        ClosureExplicit,
        let domain: Vec<f64> = (0..(std::f64::consts::PI * 1000.) as usize)
            .map(|step_id| step_id as f64 * STEP)
            .collect(),
        FunctionDescriptor::Closure(Box::new(f64::sin)),
        DomainDescriptor::Explicit(&domain),
        ComputeMethod::RectangleRight,
        A_RES,
        RECTANGLE_TOLERANCE
    );

    generate_test!(
        f64,
        ClosureUniform,
        FunctionDescriptor::Closure(Box::new(f64::sin)),
        DomainDescriptor::Uniform {
            start: 0.,
            step: STEP,
            n_step: (1000. * std::f64::consts::PI) as usize,
        },
        ComputeMethod::RectangleRight,
        A_RES,
        RECTANGLE_TOLERANCE
    );

    generate_test!(
        f64,
        ValuesExplicit,
        let domain: Vec<f64> = (0..(std::f64::consts::PI * 1000.) as usize)
            .map(|step_id| step_id as f64 * STEP)
            .collect(),
        FunctionDescriptor::Values(domain.iter().copied().map(f64::sin).collect()),
        DomainDescriptor::Explicit(&domain),
        ComputeMethod::RectangleRight,
        A_RES,
        RECTANGLE_TOLERANCE
    );

    generate_test!(
        f64,
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
        A_RES,
        RECTANGLE_TOLERANCE
    );
}

#[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
mod trapezoid {
    use super::*;

    generate_test!(
        f64,
        ClosureExplicit,
        let domain: Vec<f64> = (0..(std::f64::consts::PI * 1000.) as usize)
            .map(|step_id| step_id as f64 * STEP)
            .collect(),
        FunctionDescriptor::Closure(Box::new(f64::sin)),
        DomainDescriptor::Explicit(&domain),
        ComputeMethod::Trapezoid,
        A_RES,
        TRAPEZOID_TOLERANCE
    );

    generate_test!(
        f64,
        ClosureUniform,
        FunctionDescriptor::Closure(Box::new(f64::sin)),
        DomainDescriptor::Uniform {
            start: 0.,
            step: STEP,
            n_step: (1000. * std::f64::consts::PI) as usize,
        },
        ComputeMethod::Trapezoid,
        A_RES,
        TRAPEZOID_TOLERANCE
    );

    generate_test!(
        f64,
        ValuesExplicit,
        let domain: Vec<f64> = (0..(std::f64::consts::PI * 1000.) as usize)
            .map(|step_id| step_id as f64 * STEP)
            .collect(),
        FunctionDescriptor::Values(domain.iter().copied().map(f64::sin).collect()),
        DomainDescriptor::Explicit(&domain),
        ComputeMethod::Trapezoid,
        A_RES,
        TRAPEZOID_TOLERANCE
    );

    generate_test!(
        f64,
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
        A_RES,
        TRAPEZOID_TOLERANCE
    );
}

#[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
mod simpson {
    use super::*;

    generate_test!(
        f64,
        ClosureExplicit,
        let domain: Vec<f64> = (0..(std::f64::consts::PI * 1000.) as usize)
            .map(|step_id| step_id as f64 * STEP)
            .collect(),
        FunctionDescriptor::Closure(Box::new(f64::sin)),
        DomainDescriptor::Explicit(&domain),
        ComputeMethod::Simpson,
        A_RES,
        TRAPEZOID_TOLERANCE // FIXME: update tol
    );

    generate_test!(
        f64,
        ClosureUniform,
        FunctionDescriptor::Closure(Box::new(f64::sin)),
        DomainDescriptor::Uniform {
            start: 0.,
            step: STEP,
            n_step: (1000. * std::f64::consts::PI) as usize,
        },
        ComputeMethod::Simpson,
        A_RES,
        TRAPEZOID_TOLERANCE // FIXME: update tol
    );

    generate_test!(
        f64,
        ValuesExplicit,
        let domain: Vec<f64> = (0..(std::f64::consts::PI * 1000.) as usize)
            .map(|step_id| step_id as f64 * STEP)
            .collect(),
        FunctionDescriptor::Values(domain.iter().copied().map(f64::sin).collect()),
        DomainDescriptor::Explicit(&domain),
        ComputeMethod::Simpson,
        A_RES,
        TRAPEZOID_TOLERANCE // FIXME: update tol
    );

    generate_test!(
        f64,
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
        A_RES,
        TRAPEZOID_TOLERANCE // FIXME: update tol
    );
}

#[cfg(feature = "boole")]
#[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
mod boole {
    use super::*;

    generate_test!(
        f64,
        ClosureUniform,
        FunctionDescriptor::Closure(Box::new(f64::sin)),
        DomainDescriptor::Uniform {
            start: 0.,
            step: STEP,
            n_step: (1000. * std::f64::consts::PI) as usize - 1,
        },
        ComputeMethod::Boole,
        A_RES,
        TRAPEZOID_TOLERANCE // FIXME: update tol
    );

    generate_test!(
        f64,
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
        A_RES,
        TRAPEZOID_TOLERANCE // FIXME: update tol
    );
}

#[cfg(feature = "romberg")]
#[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
mod romberg {
    use super::*;

    generate_test!(
        f64,
        ClosureUniform,
        FunctionDescriptor::Closure(Box::new(f64::sin)),
        DomainDescriptor::Uniform {
            start: 0.,
            step: STEP,
            n_step: (1000. * std::f64::consts::PI) as usize,
        },
        ComputeMethod::Romberg { max_steps: 10 },
        A_RES,
        TRAPEZOID_TOLERANCE // FIXME: update tol
    );

    generate_test!(
        f64,
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
        A_RES,
        TRAPEZOID_TOLERANCE // FIXME: update tol
    );
}
