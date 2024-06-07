use std::{fmt, str::FromStr};
use thiserror::Error;

/// An error that can occur when parsing a [`File`].
#[derive(Error, Debug)]
pub enum FileError {
    /// The file is not valid.
    #[error("invalid file (expected a-h, got {0})")]
    Invalid(String),
}

/// A `File` on a chessboard.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum File {
    /// The File A.
    A,
    /// The File B.
    B,
    /// The File C.
    C,
    /// The File D.
    D,
    /// The File E.
    E,
    /// The File F.
    F,
    /// The File G.
    G,
    /// The File H.
    H,
}

impl File {
    /// Creates a new `File` from a `u8`.
    ///
    /// # Panics
    ///
    /// Panics if the file is not 0-7.
    ///
    /// # Examples
    ///
    /// ```
    /// # use chess_engine::File;
    /// let file = File::new(0);
    /// assert_eq!(file, File::A);
    /// ```
    pub fn new(file: u8) -> Self {
        match file {
            0 => File::A,
            1 => File::B,
            2 => File::C,
            3 => File::D,
            4 => File::E,
            5 => File::F,
            6 => File::G,
            7 => File::H,
            _ => unreachable!(),
        }
    }
}

/// Parses a `File` from a string.
///
/// # Errors
///
/// Returns a [`FileError`] if the string is not a valid file.
///
/// # Examples
///
/// ```
/// # use chess_engine::File;
/// let file: File = "a".parse().unwrap();
/// assert_eq!(file, File::A);
/// ```
impl FromStr for File {
    type Err = FileError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "a" => Ok(File::A),
            "b" => Ok(File::B),
            "c" => Ok(File::C),
            "d" => Ok(File::D),
            "e" => Ok(File::E),
            "f" => Ok(File::F),
            "g" => Ok(File::G),
            "h" => Ok(File::H),
            _ => Err(FileError::Invalid(s.to_string())),
        }
    }
}

/// Formats a `File` as a string.
///
/// # Examples
///
/// ```
/// # use chess_engine::File;
/// let file = File::A;
/// assert_eq!(file.to_string(), "a");
/// ```
impl fmt::Display for File {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            File::A => "a",
            File::B => "b",
            File::C => "c",
            File::D => "d",
            File::E => "e",
            File::F => "f",
            File::G => "g",
            File::H => "h",
        };

        write!(f, "{}", s)
    }
}
