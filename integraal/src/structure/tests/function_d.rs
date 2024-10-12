// integral D (even)
// y = f(x) = x^2 from -4 to 4
// expected value = 128/3

mod rectangle_left {}

mod rectangle_right {}

mod trapezoid {}

mod simpson {}

#[cfg(feature = "boole")]
mod boole {}

#[cfg(feature = "romberg")]
mod romberg {}
