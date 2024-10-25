//! main method implementations

// ------ IMPORTS

use crate::{
    ComputeMethod, DomainDescriptor, FunctionDescriptor, Integraal, IntegraalError, Scalar,
};
#[cfg(feature = "montecarlo")]
use rand::Rng;

// ------ CONTENT

impl<X: Scalar> Integraal<X> {
    /// Set the domain descriptor.
    #[must_use = "unused builder struct - please remove this call"]
    pub fn domain(mut self, domain_descriptor: DomainDescriptor<X>) -> Self {
        self.domain = Some(domain_descriptor);
        self
    }

    /// Set the function descriptor.
    #[must_use = "unused builder struct - please remove this call"]
    pub fn function(mut self, function_descriptor: FunctionDescriptor<X>) -> Self {
        self.function = Some(function_descriptor);
        self
    }

    /// Set the numerical integration method.
    #[must_use = "unused builder struct - please remove this call"]
    pub fn method(mut self, compute_method: ComputeMethod) -> Self {
        self.method = Some(compute_method);
        self
    }

    #[allow(clippy::missing_errors_doc, clippy::missing_panics_doc)]
    /// This method attempts to compute the integral. If it is successful, it will clear the
    /// internal [`FunctionDescriptor`] object before returning the result.
    ///
    /// # Return / Errors
    ///
    /// This method returns a `Result` taking the following values:
    /// - `Ok(X: Scalar)` -- The computation succeeded.
    /// - `Err(IntegraalError)` -- The computation failed for the reason specified by the enum.
    pub fn compute(&mut self) -> Result<X, IntegraalError> {
        // ensure all data is defined; evaluate function first because it is reset after all computations
        if self.function.is_none() | self.domain.is_none() | self.method.is_none() {
            return Err(IntegraalError::MissingParameters(
                "one or more parameter is missing",
            ));
        }

        let Some(method) = &self.method else {
            unreachable!()
        };

        let Some(domain) = &self.domain else {
            unreachable!()
        };

        #[rustfmt::skip]
        let res = match (&self.function, &self.domain) {
            // function descriptor -- values
            // domain descriptor   -- explicit
            (
                Some(FunctionDescriptor::Values(vals)),
                Some(DomainDescriptor::Explicit(args))
            ) => values_explicit_arm(vals, args, method)?,
            // function descriptor -- values
            // domain descriptor   -- uniform
            (
                Some(FunctionDescriptor::Values(vals)),
                Some(DomainDescriptor::Uniform { .. })
            ) => values_uniform_arm(vals, domain, method)?,
            // function descriptor -- closure
            // domain descriptor   -- explicit
            (
                Some(FunctionDescriptor::Closure(closure)),
                Some(DomainDescriptor::Explicit(args)),
            ) => closure_explicit_arm(closure, args, method)?,
            // function descriptor -- closure
            // domain descriptor   -- uniform
            (
                Some(FunctionDescriptor::Closure(closure)),
                Some(DomainDescriptor::Uniform { .. }),
            ) => closure_uniform_arm(closure, domain, method)?,
            (_, _) => unreachable!(),
        };

        self.function = None; // is this really useful? we could directly return if this wasn't here
        Ok(res)
    }
}

// --- internals

// function descriptor -- values
// domain descriptor   -- explicit
fn values_explicit_arm<X: Scalar>(
    vals: &[X],
    args: &[X],
    method: &ComputeMethod,
) -> Result<X, IntegraalError> {
    if args.len() != vals.len() {
        return Err(IntegraalError::InconsistentParameters(
            "function and domain value slices have different lengthes",
        ));
    }
    let n_sample = args.len();

    // because the domain may be not uniform, we have to compute step values
    let res = match method {
        ComputeMethod::RectangleLeft => (1..n_sample)
            .map(|idx| {
                let step = args[idx] - args[idx - 1];
                vals[idx - 1] * step
            })
            .sum(),
        ComputeMethod::RectangleRight => (1..n_sample)
            .map(|idx| {
                let step = args[idx] - args[idx - 1];
                vals[idx] * step
            })
            .sum(),
        ComputeMethod::Trapezoid => (1..n_sample)
            .map(|idx| {
                let step = args[idx] - args[idx - 1];
                let y1 = vals[idx - 1];
                let y2 = vals[idx];
                (y1.min(y2) + num_traits::abs(y1 - y2) / X::from_f32(2.0).unwrap()) * step
            })
            .sum(),
        ComputeMethod::Simpson => {
            // using the formula for irregularly spaced data:
            // https://en.wikipedia.org/wiki/Simpson%27s_rule#Composite_Simpson's_rule_for_irregularly_spaced_data
            // the formula is a sum from 0 to N-2, N the number of subintervals; so N = n_sample-1
            let indices: Vec<_> = (0..n_sample - 4).collect();
            indices
                .windows(3)
                .map(|is| {
                    let [i, ip1, ip2] = is else {
                        unreachable!();
                    };
                    let (h_i, h_ip1) = (args[*ip1] - args[*i], args[*ip2] - args[*ip1]);
                    let c_i = X::from(2.0).unwrap() - h_ip1 / h_i;
                    let c_ip1 = (h_i + h_ip1).powi(2) / (h_i * h_ip1);
                    let c_ip2 = X::from(2.0).unwrap() - h_i / h_ip1;
                    (h_i + h_ip1) / X::from(6.0).unwrap()
                        * (c_i * vals[*i] + c_ip1 * vals[*ip1] + c_ip2 * vals[*ip2])
                })
                .sum()
        }
        #[cfg(feature = "boole")]
        ComputeMethod::Boole { .. } => {
            return Err(IntegraalError::Unimplemented(
                "Boole's method isn't implemented for non-uniform domains",
            ));
        }
        #[cfg(feature = "romberg")]
        ComputeMethod::Romberg { .. } => {
            return Err(IntegraalError::Unimplemented(
                "Romberg's method isn't implemented for non-uniform domains",
            ));
        }
        #[cfg(feature = "montecarlo")]
        ComputeMethod::MonteCarlo { n_sample } => {
            let (mut min, mut max) = (vals[0], vals[0]);
            for v in vals {
                min = min.min(*v);
                max = max.max(*v);
            }
            let height: X = max - min;
            let widths = args.windows(2).enumerate().map(|(i, slice)| {
                let [a, b] = slice else { unreachable!() };
                (vals[i].min(X::zero())..vals[i].max(X::zero()), (*b - *a))
            });
            let mut rng = rand::thread_rng();
            let random_numbers: Vec<X> = (&mut rng)
                .sample_iter(
                    rand::distr::Uniform::new(min.to_f64().unwrap(), max.to_f64().unwrap())
                        .unwrap(),
                )
                .take(*n_sample * (args.len() - 1))
                .map(|s| X::from(s).unwrap())
                .collect();
            let total_in: X = random_numbers
                .chunks_exact(*n_sample)
                .zip(widths)
                .map(|(samples, (range, width))| {
                    X::from(samples.iter().filter(|s| range.contains(s)).count()).unwrap() * width
                })
                .sum();
            height * total_in / X::from(*n_sample * (args.len() - 1)).unwrap()
        }
    };

    Ok(res)
}

// function descriptor -- values
// domain descriptor   -- uniform
#[allow(clippy::cast_possible_truncation, clippy::too_many_lines)]
fn values_uniform_arm<X: Scalar>(
    vals: &[X],
    domain: &DomainDescriptor<X>,
    method: &ComputeMethod,
) -> Result<X, IntegraalError> {
    let DomainDescriptor::Uniform {
        start,
        step,
        n_step,
    } = domain
    else {
        unreachable!()
    };

    if *n_step != vals.len() {
        return Err(IntegraalError::InconsistentParameters(
            "provided function and domain value slices have different lengthes",
        ));
    }

    // we can use the uniform domain's step & number of step to compute areas
    let res = match method {
        ComputeMethod::RectangleLeft => {
            // ignore the last value since its a left rule
            (0..*n_step - 1).map(|step_id| vals[step_id] * *step).sum()
        }
        ComputeMethod::RectangleRight => {
            // ignore the last value since its a left rule
            (1..*n_step).map(|step_id| vals[step_id] * *step).sum()
        }
        ComputeMethod::Trapezoid => (1..*n_step)
            .map(|step_id| {
                let y1 = vals[step_id - 1];
                let y2 = vals[step_id];
                (y1.min(y2) + (y1 - y2).abs() / X::from_f32(2.0).unwrap()) * *step
            })
            .sum(),
        ComputeMethod::Simpson => {
            let indices: Vec<_> = (0..*n_step - 4).collect();
            (*step / X::from(3.0).unwrap())
                * indices
                    .windows(3)
                    .map(|is| {
                        let [i, ip1, ip2] = is else {
                            unreachable!();
                        };
                        vals[*i] + X::from(4.0).unwrap() * vals[*ip1] + vals[*ip2]
                    })
                    .sum()
        }
        #[cfg(feature = "boole")]
        ComputeMethod::Boole { force } => {
            let n_step = if *force {
                *n_step - *n_step % 4
            } else {
                return Err(IntegraalError::BadParameters(
                    "domain should be divided into a multiple of 4 segments for Boole's method",
                ));
            };

            let c = X::from(2.0 / 45.0).unwrap() * *step;

            let m1 = X::from(7.0).unwrap() * (vals[0] + vals[n_step - 1]);

            let c1 = X::from(14.0).unwrap();
            let c2 = X::from(12.0).unwrap();
            let c3 = X::from(32.0).unwrap();
            let m2: X = (1..n_step - 1)
                .map(|id| match id % 4 {
                    0 => c1 * vals[id],     // multiple of 4
                    2 => c2 * vals[id],     // pair, non-multiple of 4
                    1 | 3 => c3 * vals[id], // odd
                    _ => unreachable!(),
                })
                .sum();
            c * (m1 + m2)
        }
        #[cfg(feature = "romberg")]
        ComputeMethod::Romberg { max_steps } => {
            let (mut r1, mut r2) = (vec![X::zero(); *max_steps], vec![X::zero(); *max_steps]);
            let end = *start + X::from(*n_step).unwrap() * *step;
            let half = X::from(0.5).unwrap();
            let mut h = end - *start;

            r1[0] = half * h * (vals[0] + vals[*n_step - 1]);

            // since we dont take desired accuracy as an arg, we don't need early returns
            // => the loop is written with an iterator instead of a regular for
            (1..*max_steps).for_each(|i| {
                let (rp, rc) = if i % 2 == 0 {
                    (&mut r2, &mut r1)
                } else {
                    (&mut r1, &mut r2)
                };

                h *= half;
                let ep = 2_usize.pow(i as u32 - 1);
                let c = (1..=ep).map(|j| vals[2 * j - 1]).sum::<X>();
                rc[0] = h * c + half * rp[0];

                (1..=i).for_each(|j| {
                    let n_k = X::from(4_u32.pow(j as u32)).unwrap();
                    rc[j] = (n_k * rc[j - 1] - rp[j - 1]) / (n_k - X::one());
                });
            });
            if *n_step % 2 == 0 {
                r1[*max_steps - 1]
            } else {
                r2[*max_steps - 1]
            }
        }
        #[cfg(feature = "montecarlo")]
        ComputeMethod::MonteCarlo { n_sample } => {
            let (mut min, mut max) = (vals[0], vals[0]);
            for v in vals {
                min = min.min(*v);
                max = max.max(*v);
            }
            let intervals = vals.iter().map(|v| v.min(X::zero())..v.max(X::zero()));
            let volume: X = (max - min) * (X::from(*n_step).unwrap() * *step);
            let mut rng = rand::thread_rng();
            let random_numbers: Vec<X> = (&mut rng)
                .sample_iter(
                    rand::distr::Uniform::new(min.to_f64().unwrap(), max.to_f64().unwrap())
                        .unwrap(),
                )
                .take(*n_sample * *n_step)
                .map(|s| X::from(s).unwrap())
                .collect();
            let total_in: usize = random_numbers
                .chunks_exact(*n_sample)
                .zip(intervals)
                .map(|(samples, range)| samples.iter().filter(|s| range.contains(s)).count())
                .sum();
            volume * X::from(total_in as f64 / (*n_sample * *n_step) as f64).unwrap()
        }
    };

    Ok(res)
}

// function descriptor -- closure
// domain descriptor   -- explicit
#[allow(clippy::unnecessary_wraps)]
fn closure_explicit_arm<X: Scalar>(
    closure: impl Fn(X) -> X,
    args: &[X],
    method: &ComputeMethod,
) -> Result<X, IntegraalError> {
    let res = match method {
        ComputeMethod::RectangleLeft => (1..args.len())
            .map(|idx| {
                let step = args[idx] - args[idx - 1];
                closure(args[idx - 1]) * step
            })
            .sum(),
        ComputeMethod::RectangleRight => (1..args.len())
            .map(|idx| {
                let step = args[idx] - args[idx - 1];
                closure(args[idx]) * step
            })
            .sum(),
        ComputeMethod::Trapezoid => (1..args.len())
            .map(|idx| {
                let step = args[idx] - args[idx - 1];
                let y1 = closure(args[idx - 1]);
                let y2 = closure(args[idx]);
                (y1.min(y2) + (y1 - y2).abs() / X::from_f32(2.0).unwrap()) * step
            })
            .sum(),
        ComputeMethod::Simpson => {
            let indices: Vec<_> = (0..args.len() - 4).collect();
            indices
                .windows(3)
                .map(|is| {
                    let [i, ip1, ip2] = is else {
                        unreachable!();
                    };
                    let (h_i, h_ip1) = (args[*ip1] - args[*i], args[*ip2] - args[*ip1]);
                    let c_i = X::from(2.0).unwrap() - h_ip1 / h_i;
                    let c_ip1 = (h_i + h_ip1).powi(2) / (h_i * h_ip1);
                    let c_ip2 = X::from(2.0).unwrap() - h_i / h_ip1;
                    (h_i + h_ip1) / X::from(6.0).unwrap()
                        * (c_i * closure(args[*i])
                            + c_ip1 * closure(args[*ip1])
                            + c_ip2 * closure(args[*ip2]))
                })
                .sum()
        }
        #[cfg(feature = "boole")]
        ComputeMethod::Boole { .. } => {
            return Err(IntegraalError::Unimplemented(
                "Boole's method isn't implemented for non-uniform domains",
            ));
        }
        #[cfg(feature = "romberg")]
        ComputeMethod::Romberg { .. } => {
            return Err(IntegraalError::Unimplemented(
                "Romberg's method isn't implemented for non-uniform domains",
            ));
        }
        #[cfg(feature = "montecarlo")]
        ComputeMethod::MonteCarlo { n_sample } => {
            let (mut min, mut max) = (closure(args[0]), closure(args[0]));
            for a in args {
                min = min.min(closure(*a));
                max = max.max(closure(*a));
            }
            let height: X = max - min;
            let widths = args.windows(2).map(|slice| {
                let [a, b] = slice else { unreachable!() };
                (
                    closure(*a).min(X::zero())..closure(*a).max(X::zero()),
                    (*b - *a),
                )
            });
            let mut rng = rand::thread_rng();
            let random_numbers: Vec<X> = (&mut rng)
                .sample_iter(
                    rand::distr::Uniform::new(min.to_f64().unwrap(), max.to_f64().unwrap())
                        .unwrap(),
                )
                .take(*n_sample * (args.len() - 1))
                .map(|s| X::from(s).unwrap())
                .collect();
            let total_in: X = random_numbers
                .chunks_exact(*n_sample)
                .zip(widths)
                .map(|(samples, (range, width))| {
                    X::from(samples.iter().filter(|s| range.contains(s)).count()).unwrap() * width
                })
                .sum();
            height * total_in / X::from(*n_sample * (args.len() - 1)).unwrap()
        }
    };
    Ok(res)
}

// function descriptor -- closure
// domain descriptor   -- uniform
#[allow(
    clippy::unnecessary_wraps,
    clippy::cast_possible_truncation,
    clippy::too_many_lines
)]
fn closure_uniform_arm<X: Scalar>(
    closure: impl Fn(X) -> X,
    domain: &DomainDescriptor<X>,
    method: &ComputeMethod,
) -> Result<X, IntegraalError> {
    let DomainDescriptor::Uniform {
        start,
        step,
        n_step,
    } = domain
    else {
        unreachable!()
    };

    // compute args
    let res = match method {
        ComputeMethod::RectangleLeft => (0..*n_step - 1)
            .map(|step_id| {
                let x = *start + *step * X::from_usize(step_id).unwrap();
                closure(x) * *step
            })
            .sum(),
        ComputeMethod::RectangleRight => (1..*n_step)
            .map(|step_id| {
                let x = *start + *step * X::from_usize(step_id).unwrap();
                closure(x) * *step
            })
            .sum(),
        ComputeMethod::Trapezoid => (1..*n_step)
            .map(|step_id| {
                let x1 = *start + *step * X::from_usize(step_id - 1).unwrap();
                let x2 = *start + *step * X::from_usize(step_id).unwrap();
                let y1 = closure(x1);
                let y2 = closure(x2);
                (y1.min(y2) + (y1 - y2).abs() / X::from_f32(2.0).unwrap()) * *step
            })
            .sum(),
        ComputeMethod::Simpson => {
            let indices: Vec<_> = (0..*n_step - 4).collect();
            (*step / X::from(3.0).unwrap())
                * indices
                    .windows(3)
                    .map(|is| {
                        let [i, ip1, ip2] = is else {
                            unreachable!();
                        };
                        closure(*start + *step * X::from(*i).unwrap())
                            + X::from(4.0).unwrap()
                                * closure(*start + *step * X::from(*ip1).unwrap())
                            + closure(*start + *step * X::from(*ip2).unwrap())
                    })
                    .sum()
        }
        #[cfg(feature = "boole")]
        ComputeMethod::Boole { force } => {
            let n_step = if *force {
                *n_step - *n_step % 4
            } else {
                return Err(IntegraalError::BadParameters(
                    "domain should be divided into a multiple of 4 segments for Boole's method",
                ));
            };

            let c = X::from(2.0 / 45.0).unwrap() * *step;

            let m1 = X::from(7.0).unwrap()
                * (closure(*start) + closure(*start + X::from(n_step - 1).unwrap() * *step));

            let c1 = X::from(14.0).unwrap();
            let c2 = X::from(12.0).unwrap();
            let c3 = X::from(32.0).unwrap();
            let m2: X = (1..n_step - 1)
                .map(|id| match id % 4 {
                    0 => c1 * closure(*start + X::from(id).unwrap() * *step), // multiple of 4
                    2 => c2 * closure(*start + X::from(id).unwrap() * *step), // pair, non-multiple of 4
                    1 | 3 => c3 * closure(*start + X::from(id).unwrap() * *step), // odd
                    _ => unreachable!(),
                })
                .sum();
            c * (m1 + m2)
        }
        #[cfg(feature = "romberg")]
        ComputeMethod::Romberg { max_steps } => {
            let (mut r1, mut r2) = (vec![X::zero(); *max_steps], vec![X::zero(); *max_steps]);
            let end = *start + X::from(*n_step).unwrap() * *step;
            let half = X::from(0.5).unwrap();
            let mut h = end - *start;

            r1[0] = half
                * h
                * (closure(*start) + closure(*start + X::from(*n_step - 1).unwrap() * *step));

            // since we dont take desired accuracy as an arg, we don't need early returns
            // => the loop is written with an iterator instead of a regular for
            (1..*max_steps).for_each(|i| {
                let (rp, rc) = if i % 2 == 0 {
                    (&mut r2, &mut r1)
                } else {
                    (&mut r1, &mut r2)
                };

                h *= half;
                let ep = 2_usize.pow(i as u32 - 1);
                let c = (1..=ep)
                    .map(|j| closure(*start + X::from(2 * j - 1).unwrap() * *step))
                    .sum::<X>();
                rc[0] = h * c + half * rp[0];

                (1..=i).for_each(|j| {
                    let n_k = X::from(4_u32.pow(j as u32)).unwrap();
                    rc[j] = (n_k * rc[j - 1] - rp[j - 1]) / (n_k - X::one());
                });
            });
            if *max_steps % 2 == 0 {
                r1[*max_steps - 1]
            } else {
                r2[*max_steps - 1]
            }
        }
        #[cfg(feature = "montecarlo")]
        ComputeMethod::MonteCarlo { n_sample } => {
            // FIXME: nuke this temp allocation
            let vals: Vec<_> = (0..*n_step)
                .map(|i| closure(*start + X::from(i).unwrap() * *step))
                .collect();
            let (mut min, mut max) = (vals[0], vals[0]);
            for v in &vals {
                min = min.min(*v);
                max = max.max(*v);
            }
            let intervals = vals.iter().map(|v| v.min(X::zero())..v.max(X::zero()));
            let volume: X = (max - min) * (X::from(*n_step).unwrap() * *step);
            let mut rng = rand::thread_rng();
            let random_numbers: Vec<X> = (&mut rng)
                .sample_iter(
                    rand::distr::Uniform::new(min.to_f64().unwrap(), max.to_f64().unwrap())
                        .unwrap(),
                )
                .take(*n_sample * *n_step)
                .map(|s| X::from(s).unwrap())
                .collect();
            let total_in: usize = random_numbers
                .chunks_exact(*n_sample)
                .zip(intervals)
                .map(|(samples, range)| samples.iter().filter(|s| range.contains(s)).count())
                .sum();
            volume * X::from(total_in as f64 / (*n_sample * *n_step) as f64).unwrap()
        }
    };

    Ok(res)
}
