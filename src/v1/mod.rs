mod calc;
mod err;
mod hlp;
mod points;
mod points_iter;
mod tfti;

pub(crate) use calc::calc_spline;
pub use err::{Error, Result};
pub use points::{Point, Points, DEFAULT_APPROX_EQ_PRECISION};
pub use tfti::{TryFrom, TryInto};
