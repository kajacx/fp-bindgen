#![allow(unused_imports)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct FlattenedStruct {
    pub foo: String,
    pub bar: i64,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct FpFlatten {
    #[serde(flatten)]
    pub flattened: FlattenedStruct,
}

/// Our struct for passing date time instances.
///
/// We wrap the `OffsetDateTime` type in a new struct so that the Serde
/// attributes can be inserted. These are necessary to enable RFC3339
/// formatting. Without a wrapper type like this, we would not be able to pass
/// date time instances directly to function arguments and we might run into
/// trouble embedding them into certain generic types.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct MyDateTime(
    #[serde(with = "time::serde::rfc3339")]
    pub time::OffsetDateTime,
);

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct SerdeFlatten {
    #[serde(flatten)]
    pub flattened: FlattenedStruct,
}
