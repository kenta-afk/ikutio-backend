use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

include!("authservice.rs");
include!("locationservice.rs");
pub mod authserviceclient;
pub mod locationserviceclient;
