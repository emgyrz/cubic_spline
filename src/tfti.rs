///
/// The same as [`TryInto`] from `std/core`, except that it allows generic implementations.
/// Details are in this [issue](https://github.com/rust-lang/rust/issues/50133)
///
/// [Official documentation](https://doc.rust-lang.org/std/convert/trait.TryInto.html)
///
/// [`TryFrom`]: trait.TryFrom.html
/// [`TryInto`]: https://doc.rust-lang.org/std/convert/trait.TryInto.html
/// [`Into`]: https://doc.rust-lang.org/std/convert/trait.Into.html
pub trait TryInto<T>: Sized {
  /// The type returned in the event of a conversion error.
  type Error;

  /// Performs the conversion.
  fn try_into(self) -> Result<T, Self::Error>;
}

/// The same as [`TryFrom`] from `std/core`, except that it allows generic implementations.
/// Details are in this [issue](https://github.com/rust-lang/rust/issues/50133)
///
/// [Official documentation](https://doc.rust-lang.org/std/convert/trait.TryFrom.html)
///
/// Usually you don't need to implement it yourself.
/// This trait here is for safe conversion into [`Points`]
///
/// # Example
/// ```
/// use cubic_spline::{TryFrom, Points, Error};
///
/// let my_points: Vec<(f64,f64)> = vec![(1.0, 1.0)];
/// let prepared_points = Points::try_from(&my_points);
///
/// assert_eq!(prepared_points.unwrap_err(), Error::TooFewPoints);
///
/// let another_try = Points::try_from( &[ [3.0, 5.1], [10.3, 11.9] ] );
/// assert!(another_try.is_ok());
///
/// ```
///
/// [`try_from`]: trait.TryFrom.html#tymethod.try_from
/// [`TryFrom`]: https://doc.rust-lang.org/std/convert/trait.TryFrom.html
/// [`Points`]: struct.Points.html
///
///
pub trait TryFrom<T>: Sized {
  /// The type returned in the event of a conversion error.
  type Error;

  /// Performs the conversion.
  fn try_from(value: T) -> Result<Self, Self::Error>;
}

////////////////////////////////////////////////////////////////////////////////
// GENERIC IMPLS
////////////////////////////////////////////////////////////////////////////////

// TryFrom implies TryInto
impl<T, U> TryInto<U> for T
where
  U: TryFrom<T>,
{
  type Error = U::Error;

  fn try_into(self) -> Result<U, U::Error> {
    U::try_from(self)
  }
}

// // This don't allow to implement a generic TryFrom

// Infallible conversions are semantically equivalent to fallible conversions
// with an uninhabited error type.
// impl<T, U> TryFrom<U> for T
//   where
//     U: Into<T>,
// {
//   type Error = Infallible;
//
//   fn try_from(value: U) -> Result<Self, Self::Error> {
//     Ok(U::into(value))
//   }
// }
