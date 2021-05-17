// Copyright 2021 Conveen

//! Crate-level error and result types.

/// Crate-level error type.
///
/// All possible error types should be handled by this enum so [Result](type.Result.html) can be
/// used consistently across the crate.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// IO errors.
    #[error(transparent)]
    IO(#[from] std::io::Error),
    /// Error from creating an `&str` from `&[u8]`.
    #[error(transparent)]
    Utf8(#[from] std::str::Utf8Error),
}

/// Crate-level result type that wraps [Error](enum.Error.html).
pub type Result<T> = std::result::Result<T, Error>;
