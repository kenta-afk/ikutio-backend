use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

include!("authservice.rs");
include!("profileservice.rs");
include!("gameservice.rs");
include!("locationservice.rs");
pub mod authserviceclient;
pub mod gameserviceclient;
pub mod locationserviceclient;
pub mod profileserviceclient;
