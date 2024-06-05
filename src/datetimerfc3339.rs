use std::str::FromStr;
use serde::Deserialize;
use serde::Serialize;


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct DateRfc3339(pub(crate) chrono::Date<chrono::Utc>);

impl DateRfc3339 {
    pub fn now() -> Self {
        Self(chrono::Utc::today())
    }

    pub fn from_naive(naive: chrono::NaiveDate) -> Self {
        Self(chrono::Date::from_utc(naive, chrono::Utc))
    }
}

impl FromStr for DateRfc3339 {
    type Err = chrono::ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d")
            .or_else(|_| chrono::NaiveDate::parse_from_str(s, "%d.%m.%Y"))
            .or_else(|_e| {
                let millis = i64::from_str(s).map_err(|_| _e)?;
                chrono::NaiveDateTime::from_timestamp_millis(millis)
                    .map(|dt| dt.date())
                    .ok_or(_e)
            })?;
        Ok(Self(chrono::Date::from_utc(value, chrono::Utc)))
    }
}

impl ToString for DateRfc3339 {
    fn to_string(&self) -> String {
        let v = self.0.format("%d.%m.%Y").to_string();
        v
    }
}

impl<'de> Deserialize<'de> for DateRfc3339 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(DateVisitorRfc3339)
    }
}

impl Serialize for DateRfc3339 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let v = self.to_string();
        serializer.serialize_str(v.as_str())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct DateTimeRfc3339(pub chrono::DateTime<chrono::Utc>);

impl DateTimeRfc3339 {
    pub fn now() -> Self {
        Self(chrono::Utc::now())
    }

    pub fn from_naive(naive: chrono::NaiveDateTime) -> Self {
        Self(chrono::DateTime::from_utc(naive, chrono::Utc))
    }

    pub fn from_naive_date(naive: chrono::NaiveDate) -> Self {
        let naive = naive
            .and_time(chrono::NaiveTime::from_num_seconds_from_midnight_opt(100000, 0).unwrap());
        Self(chrono::DateTime::from_utc(naive, chrono::Utc))
    }
}

impl FromStr for DateTimeRfc3339 {
    type Err = chrono::ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = chrono::DateTime::parse_from_rfc3339(s)
            .map(|dt| dt.with_timezone(&chrono::Utc))
            .or_else(|_e| {
                let millis = i64::from_str(s).map_err(|_| _e)?;
                let naive =
                    chrono::naive::NaiveDateTime::from_timestamp_millis(millis).ok_or(_e)?;
                Ok(chrono::DateTime::from_local(naive, chrono::Utc))
            })?;
        Ok(Self(value))
    }
}

impl ToString for DateTimeRfc3339 {
    fn to_string(&self) -> String {
        // let v = self.0.to_rfc3339(); // wishful thinking

        self.0.timestamp_millis().to_string()
    }
}

impl<'de> Deserialize<'de> for DateTimeRfc3339 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(DateTimeVisitorRfc3339)
    }
}

impl Serialize for DateTimeRfc3339 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let v = self.to_string();
        serializer.serialize_str(v.as_str())
    }
}

#[derive(Debug, Clone, Copy, Hash)]
struct DateTimeVisitorRfc3339;

impl<'de> serde::de::Visitor<'de> for DateTimeVisitorRfc3339 {
    type Value = DateTimeRfc3339;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            formatter,
            "a string containing an rfc33339 compliant timestamp"
        )
    }

    fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Self::Value::from_str(s).map_err(serde::de::Error::custom)
    }

    fn visit_string<E>(self, s: String) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.visit_str::<E>(s.as_str())
    }
}

#[derive(Debug, Clone, Copy, Hash)]
struct DateVisitorRfc3339;

impl<'de> serde::de::Visitor<'de> for DateVisitorRfc3339 {
    type Value = DateRfc3339;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            formatter,
            "a string containing an rfc33339 compliant timestamp"
        )
    }

    fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Self::Value::from_str(s).map_err(serde::de::Error::custom)
    }

    fn visit_string<E>(self, s: String) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.visit_str::<E>(s.as_str())
    }
}
