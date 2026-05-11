use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use crate::components::*;

#[cfg(target_arch = "wasm32")]
use gloo_storage::{LocalStorage, Storage};

#[path = "schema.rs"]
pub mod schema;
pub use schema::*;

#[path = "io.rs"]
pub mod io;
pub use io::*;

#[path = "systems.rs"]
pub mod systems;
pub use systems::*;

pub const SAVE_VERSION: u32 = 7;
