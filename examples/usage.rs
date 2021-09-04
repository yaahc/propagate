#![feature(try_blocks)]
// Overrides the std library's `Result` type.

use core::fmt;
use std::fs::File;
use std::io;

#[derive(Debug)]
enum MyError {
    Unlucky,
    Io(io::Error),
    TooSmall(u64),
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unlucky => write!(f, "Not this time!"),
            Self::Io(_) => write!(f, "I/O error"),
            Self::TooSmall(size) => write!(f, "File too small: {} bytes", size),
        }
    }
}

impl std::error::Error for MyError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            MyError::Unlucky => None,
            MyError::Io(e) => e.source(),
            MyError::TooSmall(_) => None,
        }
    }
}

impl From<io::Error> for MyError {
    fn from(e: io::Error) -> Self {
        Self::Io(e)
    }
}

fn file_size(path: &str) -> propagate::Result<u64, MyError> {
    try {
        // The `?` operator coerces `std::result::Result<_, io::Error>` into `Result<_, MyError>`.
        let size = File::open(path)?.metadata()?.len();

        if size < 1024 {
            Err(MyError::TooSmall(size))?
        }

        size
    }
}

fn maybe_file_size(path: &str) -> propagate::Result<u64, MyError> {
    let lucky = (path.len() % 2) == 0;

    try {
        if !lucky {
            Err(MyError::Unlucky)?
        }

        file_size(path)?
    }
}

fn main() -> propagate::Result<(), MyError> {
    let size = maybe_file_size("foo.txt")?;
    println!("File size: {} KiB", size / 1024);
    propagate::Ok(())
}