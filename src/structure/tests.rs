//! main structure tests

// test utils

macro_rules! almost_equal {
    ($v1: ident, $v2: ident) => {
        ($v1 - $v2).abs() < f64::EPSILON
    };
}

// incorrect usages

#[test]
#[should_panic(expected = "a")]
fn basic() {
    assert_eq!(1 + 1, 3);
}

// correct usages
// test names follow this pattern:
// <integral>_<FunctionDescriptorEnum>_<DomainDescriptorEnum>_<ComputeMethodEnum>

// integral A
// y = f(x) = sin(x) from 0 to PI

#[allow(non_snake_case)]
#[test]
fn A_Closure_Explicit_Rectangle() {
    assert_eq!(1 + 1, 2);
}

// integral B
// y = f(x) = x from -1 to 1

#[allow(non_snake_case)]
#[test]
fn B_Closure_Explicit_Rectangle() {
    assert_eq!(1 + 1, 2);
}
