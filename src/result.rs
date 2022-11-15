pub trait ResultExt<T, E, EI> {
    /// Exchange the inner err and the outer err.
    ///
    /// # Examples
    ///
    /// ```
    /// use chain_ext::result::ResultExt;
    ///
    /// assert_eq!(
    ///   Result::<Result<i32, bool>, u32>::Ok(Err(false)),
    ///   Result::<Result<i32, u32>, bool>::Err(false)
    /// );
    /// ```
    fn transpose(self) -> Result<Result<T, E>, EI>;

    /// Squeeze the nested result by unionising the inner err and the outer err.
    ///
    /// # Examples
    ///
    /// ```
    /// use chain_ext::result::ResultExt;
    ///
    /// assert_eq!(
    ///     Result::<Result<i32, u32>, bool>::Ok(Ok(10)).squeeze(),
    ///     Result::<i32, either::Either<bool, u32>>::Ok(10)
    /// );
    /// ```
    #[cfg(feature = "either")]
    fn squeeze(self) -> Result<T, either::Either<E, EI>>;
}

impl<T, E, EI> ResultExt<T, E, EI> for Result<Result<T, EI>, E> {
    fn transpose(self) -> Result<Result<T, E>, EI> {
        match self {
            Ok(Ok(value)) => Ok(Ok(value)),
            Ok(Err(err)) => Err(err),
            Err(err) => Ok(Err(err)),
        }
    }

    #[cfg(feature = "either")]
    fn squeeze(self) -> Result<T, either::Either<E, EI>> {
        match self {
            Ok(Ok(value)) => Ok(value),
            Ok(Err(err)) => Err(either::Either::Right(err)),
            Err(err) => Err(either::Either::Left(err)),
        }
    }
}

pub trait ResultWrapExt: Sized {
    fn wrap_ok<E>(self) -> Result<Self, E> {
        Ok(self)
    }
}

impl<T: Sized> ResultWrapExt for T {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transpose() {
        assert_eq!(
            Result::<Result<i32, u32>, bool>::Ok(Ok(10)).transpose(),
            Result::<Result<i32, bool>, u32>::Ok(Ok(10)),
        );
        assert_eq!(
            Result::<Result<i32, u32>, bool>::Ok(Err(10u32)).transpose(),
            Result::<Result<i32, bool>, u32>::Err(10u32),
        );
        assert_eq!(
            Result::<Result<i32, u32>, bool>::Err(false).transpose(),
            Result::<Result<i32, bool>, u32>::Ok(Err(false)),
        );
    }

    #[cfg(feature = "either")]
    #[test]
    fn test_squeeze() {
        assert_eq!(
            Result::<Result<i32, u32>, bool>::Ok(Ok(10)).squeeze(),
            Result::<i32, either::Either<bool, u32>>::Ok(10)
        );
        assert_eq!(
            Result::<Result<i32, u32>, bool>::Ok(Err(10)).squeeze(),
            Result::<i32, either::Either<bool, u32>>::Err(either::Right(10))
        );
        assert_eq!(
            Result::<Result<i32, u32>, bool>::Err(false).squeeze(),
            Result::<i32, either::Either<bool, u32>>::Err(either::Left(false))
        );
    }

    #[test]
    fn test_wrap_ok() {
        assert_eq!(10.wrap_ok::<bool>(), Result::<i32, bool>::Ok(10))
    }
}
