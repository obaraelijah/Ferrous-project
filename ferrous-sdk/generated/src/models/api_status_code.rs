/*
 * ferrous
 *
 * The core component of ferrous-project
 *
 * The version of the OpenAPI document: 0.1.0
 * Generated by: https://openapi-generator.tech
 */

/// ApiStatusCode : This type holds all possible error types that can be returned by the API.  Numbers between 1000 and 1999 (inclusive) are client errors that can be handled by the client. Numbers between 2000 and 2999 (inclusive) are server errors.

/// This type holds all possible error types that can be returned by the API.  Numbers between 1000 and 1999 (inclusive) are client errors that can be handled by the client. Numbers between 2000 and 2999 (inclusive) are server errors.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ApiStatusCode {
    #[serde(rename = "1000")]
    Variant1000,
    #[serde(rename = "1001")]
    Variant1001,
    #[serde(rename = "1002")]
    Variant1002,
    #[serde(rename = "1003")]
    Variant1003,
    #[serde(rename = "1004")]
    Variant1004,
    #[serde(rename = "1005")]
    Variant1005,
    #[serde(rename = "1006")]
    Variant1006,
    #[serde(rename = "1007")]
    Variant1007,
    #[serde(rename = "1008")]
    Variant1008,
    #[serde(rename = "1009")]
    Variant1009,
    #[serde(rename = "1010")]
    Variant1010,
    #[serde(rename = "1011")]
    Variant1011,
    #[serde(rename = "1012")]
    Variant1012,
    #[serde(rename = "1013")]
    Variant1013,
    #[serde(rename = "1014")]
    Variant1014,
    #[serde(rename = "1015")]
    Variant1015,
    #[serde(rename = "1016")]
    Variant1016,
    #[serde(rename = "1017")]
    Variant1017,
    #[serde(rename = "1018")]
    Variant1018,
    #[serde(rename = "1019")]
    Variant1019,
    #[serde(rename = "1020")]
    Variant1020,
    #[serde(rename = "2000")]
    Variant2000,
    #[serde(rename = "2001")]
    Variant2001,
    #[serde(rename = "2002")]
    Variant2002,
    #[serde(rename = "2003")]
    Variant2003,
    #[serde(rename = "2004")]
    Variant2004,
}

impl ToString for ApiStatusCode {
    fn to_string(&self) -> String {
        match self {
            Self::Variant1000 => String::from("1000"),
            Self::Variant1001 => String::from("1001"),
            Self::Variant1002 => String::from("1002"),
            Self::Variant1003 => String::from("1003"),
            Self::Variant1004 => String::from("1004"),
            Self::Variant1005 => String::from("1005"),
            Self::Variant1006 => String::from("1006"),
            Self::Variant1007 => String::from("1007"),
            Self::Variant1008 => String::from("1008"),
            Self::Variant1009 => String::from("1009"),
            Self::Variant1010 => String::from("1010"),
            Self::Variant1011 => String::from("1011"),
            Self::Variant1012 => String::from("1012"),
            Self::Variant1013 => String::from("1013"),
            Self::Variant1014 => String::from("1014"),
            Self::Variant1015 => String::from("1015"),
            Self::Variant1016 => String::from("1016"),
            Self::Variant1017 => String::from("1017"),
            Self::Variant1018 => String::from("1018"),
            Self::Variant1019 => String::from("1019"),
            Self::Variant1020 => String::from("1020"),
            Self::Variant2000 => String::from("2000"),
            Self::Variant2001 => String::from("2001"),
            Self::Variant2002 => String::from("2002"),
            Self::Variant2003 => String::from("2003"),
            Self::Variant2004 => String::from("2004"),
        }
    }
}

impl Default for ApiStatusCode {
    fn default() -> ApiStatusCode {
        Self::Variant1000
    }
}
