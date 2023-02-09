mod aliases;
use std::fmt::Display;

pub use aliases::*;

mod flattening;
pub use flattening::*;

mod generics;
use fp_bindgen::prelude::Serializable;
pub use generics::*;

mod inline_docs;
pub use inline_docs::*;

mod options;
pub use options::*;

mod renaming;
pub use renaming::*;

mod tagged_enums;
use serde::{Deserialize, Serialize};
pub use tagged_enums::*;

mod time;
pub use self::time::*;

mod use_statements;
pub use use_statements::*;

#[derive(Serializable, Serialize, Deserialize)]
pub struct NewI32(i32);

#[derive(Serializable, Debug)]
pub struct NewF32(f32);

impl Display for NewF32 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
