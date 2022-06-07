//! Error utilities
use std::{
    error::Error,
    sync::{LockResult, MutexGuard},
};

pub trait NotPoison<'a, T> {
    fn not_poison(self) -> Result<MutexGuard<'a, Vec<T>>, Box<dyn Error>>;
}

impl<'a, T> NotPoison<'a, T> for LockResult<MutexGuard<'a, Vec<T>>> {
    fn not_poison(self) -> Result<MutexGuard<'a, Vec<T>>, Box<dyn Error>> {
        match self {
            Ok(v) => Ok(v),
            Err(e) => Err(e.to_string().into()),
        }
    }
}
