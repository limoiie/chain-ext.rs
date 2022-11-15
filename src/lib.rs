#![feature(const_trait_impl)]

use std::marker::Destruct;

pub mod io;
pub mod option;
pub mod path;
pub mod result;

#[cfg(feature = "mongodb")]
pub mod mongodb;

#[cfg(feature = "mongodb-gridfs")]
pub mod mongodb_gridfs;

pub trait Apply {
    fn apply<F, R>(self, f: F) -> R
    where
        F: ~const FnOnce(Self) -> R,
        F: ~const Destruct,
        Self: Sized;
}

impl<T: Sized> Apply for T {
    fn apply<F, R>(self, f: F) -> R
    where
        F: ~const FnOnce(Self) -> R,
        F: ~const Destruct,
    {
        f(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apply_move() {
        let v = 10;
        let n = v.apply(|n| n + 10);
        assert_eq!(n, 10 + 10);
    }

    #[test]
    fn test_apply_ref() {
        let v = 10;
        let n = (&v).apply(|n| n + 20);
        assert_eq!(n, 10 + 20);
    }

    #[test]
    fn test_apply_mut_ref() {
        let mut v = 10;
        (&mut v).apply(|v| *v = *v + 10);
        assert_eq!(v, 10 + 10);
    }
}
