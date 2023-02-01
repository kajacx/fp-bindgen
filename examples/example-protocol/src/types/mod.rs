mod aliases;
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
