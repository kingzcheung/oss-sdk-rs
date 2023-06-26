//! Copyright The iFREEGROUP/oss-sdk-rs Authors

use serde::{Deserialize};

pub mod error;
pub mod object;

#[derive(Default,Deserialize)]
pub struct Empty;
