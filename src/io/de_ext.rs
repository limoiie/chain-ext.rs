use serde::de::DeserializeOwned;

pub trait DeExt {
    #[cfg(feature = "serde_yaml")]
    fn de_yaml<T>(self) -> Result<T, serde_yaml::Error>
    where
        T: DeserializeOwned;
}

impl<R: std::io::Read> DeExt for R {
    #[cfg(feature = "serde_yaml")]
    fn de_yaml<T>(self) -> Result<T, serde_yaml::Error>
    where
        T: DeserializeOwned,
    {
        serde_yaml::from_reader(self)
    }
}
