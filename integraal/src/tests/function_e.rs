// integral E (odd)
// y = f(x) = x^3 from -4 to 4
// expected value = 0

mod rectangle_left {}

mod rectangle_right {}

mod trapezoid {}

mod simpson {}

#[cfg(feature = "boole")]
mod boole {}

#[cfg(feature = "romberg")]
mod romberg {}
