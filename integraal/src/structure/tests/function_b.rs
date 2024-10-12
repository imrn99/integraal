// integral B (contiguous, non-contiguous derivative)
// y = f(x) = |  x     from 0 to 1
//            |  1     from 1 to 2
//            | -x + 3 from 2 to 3
// expected value = 2

mod rectangle_left {}

mod rectangle_right {}

mod trapezoid {}

mod simpson {}

#[cfg(feature = "boole")]
mod boole {}

#[cfg(feature = "romberg")]
mod romberg {}
