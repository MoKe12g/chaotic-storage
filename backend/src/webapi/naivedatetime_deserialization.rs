use chrono::NaiveDateTime;
use serde::{self, Deserialize, Deserializer};

const FORMATS: [&str; 2] = [
    "%Y-%m-%dT%H:%M",      // without seconds
    "%Y-%m-%dT%H:%M:%S",   // with seconds
];

pub fn deserialize_datetime<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    for format in FORMATS.iter() {
        if let Ok(dt) = NaiveDateTime::parse_from_str(&s, format) {
            return Ok(dt);
        }
    }
    Err(serde::de::Error::custom("invalid datetime format"))
}
