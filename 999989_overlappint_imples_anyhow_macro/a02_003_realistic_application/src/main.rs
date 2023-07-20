use std::error::Error as StdError;
use std::fmt::Display;

pub struct Error(/* ... */);

// Our two constructors. The first is more general.
impl Error {
    pub(crate) fn from_fmt<T: Display>(error: T) -> Self {
        println!("called Error::from_fmt");
        Error {}
    }
    pub(crate) fn from_std_error<T: StdError>(error: T) -> Self {
        _ = error.source(); // it works!
        println!("called Error::from_std_error");
        Error {}
    }
}

macro_rules! anyhow {
    ($err:expr) => {{
        #[allow(unused_imports)]
        use $crate::{DisplayKind, StdErrorKind};
        match $err {
            error => (&error).anyhow_kind().new(error),
        }
    }};
}

// If the arg implements Display but not StdError, anyhow_kind() will
// return this tag.
struct DisplayTag;

trait DisplayKind {
    #[inline]
    fn anyhow_kind(&self) -> DisplayTag {
        DisplayTag
    }
}

// Requires one extra autoref to call! Lower priority than StdErrorKind.
impl<T: Display> DisplayKind for &T {}

impl DisplayTag {
    #[inline]
    fn new<M: Display>(self, message: M) -> Error {
        Error::from_fmt(message)
    }
}

// If the arg implements StdError (and thus also Display), anyhow_kind()
// will return this tag.
struct StdErrorTag;

trait StdErrorKind {
    #[inline]
    fn anyhow_kind(&self) -> StdErrorTag {
        StdErrorTag
    }
}

// Does not require any autoref if called as (&error).anyhow_kind().
impl<T: StdError> StdErrorKind for T {}

impl StdErrorTag {
    #[inline]
    fn new<E: StdError>(self, error: E) -> Error {
        Error::from_std_error(error)
    }
}

fn main() {
    // Turn a &str into an error.
    // &str implements Display but not std::error::Error.
    let _err = anyhow!("oh no!");

    // Turn an existing std::error::Error value into our error without
    // losing its source() and backtrace() if there is one.
    let io_error = std::fs::read("/tmp/nonexist").unwrap_err();
    let _err = anyhow!(io_error);
}
