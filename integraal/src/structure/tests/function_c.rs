// integral C (piece-wise contiguous)
// y = f(x) = | 0 from 0 to 1
//            | 1 from 1 to 1.5
//            | 0 from 1.5 to 2
// expected value = 0.5

mod rectangle_left {}

mod rectangle_right {}

mod trapezoid {}

mod simpson {}

#[cfg(feature = "boole")]
mod boole {}

#[cfg(feature = "romberg")]
mod romberg {}
