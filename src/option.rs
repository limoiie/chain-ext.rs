/// Extend [core::option::Option] with more chain methods.
pub trait OptionExt<T> {
    /// Return the option if it contains value, otherwise return the result as [core::option::Option].
    ///
    /// # Examples
    ///
    /// ```
    /// use chain_ext::option::OptionExt;
    ///
    /// assert_eq!(Some(10).or_ok::<u8>(Ok(20)), Some(10));
    /// assert_eq!(Some(10).or_ok(Err(0)), Some(10));
    /// assert_eq!(None::<i32>.or_ok::<u8>(Ok(20)), Some(20));
    /// assert_eq!(None::<i32>.or_ok(Err(0)), None);
    /// ```
    fn or_ok<E>(self, result: Result<T, E>) -> Self;

    /// Return the option if it contains value, otherwise return default as [core::option::Option].
    ///
    /// # Exmaples
    ///
    /// ```
    /// use chain_ext::option::OptionExt;
    ///
    /// assert_eq!(Some(10).or_wrap(20), Some(10));
    /// assert_eq!(None.or_wrap(20), Some(20));
    /// ```
    fn or_wrap(self, default: T) -> Self;

    /// Return the option of pair of inner values of self and the given option if both them contain
    /// a value, otherwise return [core::option::None].
    ///
    /// # Examples
    ///
    /// ```
    /// use chain_ext::option::OptionExt;
    ///
    /// assert_eq!(Some(10).pair(Some(20u32)), Some((10, 20u32)));
    /// assert_eq!(None::<i32>.pair(Some(20)), None);
    /// assert_eq!(Some(10).pair(None::<u32>), None);
    /// assert_eq!(None::<i32>.pair(None::<u32>), None);
    /// ```
    fn pair<S>(self, other: Option<S>) -> Option<(T, S)>;
}

impl<T> OptionExt<T> for Option<T> {
    fn or_ok<E>(self, result: Result<T, E>) -> Self {
        match self {
            Some(_) => self,
            _ => result.ok(),
        }
    }

    fn or_wrap(self, default: T) -> Self {
        match self {
            Some(_) => self,
            _ => Some(default),
        }
    }

    fn pair<S>(self, other: Option<S>) -> Option<(T, S)> {
        match (self, other) {
            (Some(this), Some(that)) => Some((this, that)),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_or_ok() {
        assert_eq!(None::<i32>.or_ok::<i32>(Ok(10)), Some(10));
        assert_eq!(None::<i32>.or_ok(Err(10)), None);
        assert_eq!(Some(9).or_ok::<i32>(Ok(10)), Some(9));
        assert_eq!(Some(9).or_ok(Err(1)), Some(9));
    }

    #[test]
    fn test_or_wrap() {
        assert_eq!(None::<i32>.or_wrap(10), Some(10));
        assert_eq!(Some(9).or_wrap(10), Some(9));
    }

    #[test]
    fn test_pair() {
        assert_eq!(Some(10).pair(Some(20)), Some((10, 20)));
        assert_eq!(Some(10).pair(None::<i32>), None);
        assert_eq!(None::<i32>.pair(Some(10)), None);
        assert_eq!(None::<i32>.pair(None::<i32>), None);
    }
}
