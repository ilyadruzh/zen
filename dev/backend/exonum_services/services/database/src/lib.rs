extern crate bodyparser;
#[macro_use]
extern crate display_derive;
#[macro_use]
extern crate enum_primitive_derive;
#[macro_use]
extern crate exonum;
extern crate failure;
extern crate iron;
extern crate num_traits;
extern crate router;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate toml;

/// Persistent data.
pub mod schema;

/// Transactions.
pub mod transactions;

/// Contracts.
pub mod contracts;

/// REST API.
pub mod api;

/// Service declaration.
pub mod service;

/// Service configuration.
pub mod cmd;