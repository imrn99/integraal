// ------ IMPORTS

use super::*;

// ------ CONTENT

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
    let mut integral: Integraal<f64> = Integraal::default().method(method);
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
    let domain = DomainDescriptor::Explicit(vec![0.0, 0.1, 0.2, 0.3, 0.4]);

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

    // equivalent to the first domain
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
