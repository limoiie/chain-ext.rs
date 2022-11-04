use serde::de::DeserializeOwned;

pub trait DeExt {
    #[cfg(feature = "serde_json")]
    fn de_json<T>(self) -> Result<T, serde_json::Error>
    where
        T: DeserializeOwned;

    #[cfg(feature = "serde_yaml")]
    fn de_yaml<T>(self) -> Result<T, serde_yaml::Error>
    where
        T: DeserializeOwned;
}

impl<R: std::io::Read> DeExt for R {
    #[cfg(feature = "serde_json")]
    fn de_json<T>(self) -> Result<T, serde_json::Error>
    where
        T: DeserializeOwned,
    {
        serde_json::from_reader(self)
    }

    #[cfg(feature = "serde_yaml")]
    fn de_yaml<T>(self) -> Result<T, serde_yaml::Error>
    where
        T: DeserializeOwned,
    {
        serde_yaml::from_reader(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_de_json() {
        let json_string = r###"{name: limo, type: real}"###;
        let m: HashMap<String, String> = json_string.as_bytes().de_json().unwrap();

        assert_eq!(
            m,
            HashMap::from([
                ("name".into(), "fake".into()),
                ("type".into(), "real".into())
            ])
        )
    }

    #[test]
    fn test_de_yaml() {
        let yaml_string = r###"name: fake
type: real"###;
        let m: HashMap<String, String> = yaml_string.as_bytes().de_yaml().unwrap();

        assert_eq!(
            m,
            HashMap::from([
                ("name".into(), "fake".into()),
                ("type".into(), "real".into())
            ])
        )
    }
}
