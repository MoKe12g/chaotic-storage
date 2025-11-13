use chrono::NaiveDateTime;
use serde::{self, Deserialize, Deserializer};


// because typescript is not ok
const FORMATS: [&str; 3] = [
    "%Y-%m-%dT%H:%M:%S%.f",   // with fractional seconds
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
    Err(serde::de::Error::custom(format!("invalid datetime format. Expected one of: {:?}", FORMATS)))
}
