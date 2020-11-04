use std::fmt::{self, Debug, Display, Formatter};

///
/// This type represents all possible errors that can occur when constructing new spline points.
#[derive(Eq, PartialEq)]
pub enum Error {
  ///
  /// Thrown when you passing flatten points and missing last `y` value.
  MissingY,

  ///
  /// Thrown when there are less than **two** points passed.
  TooFewPoints,
}

///
/// Alias for a `Result` with the error type [`cubic_spline::Error`].
///
/// [`cubic_spline::Error`]: enum.Error.html
pub type Result<T, E = Error> = std::result::Result<T, E>;

impl Error {
  pub fn msg(&self) -> &'static str {
    match self {
      Error::MissingY => "Passed values is not even. Last `y` is missing",
      Error::TooFewPoints => "Too few points. There should be more than one",
    }
  }
}

impl Display for Error {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    f.write_str(self.msg())
  }
}

impl Debug for Error {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    f.write_str(self.msg())
  }
}

impl std::error::Error for Error {}
