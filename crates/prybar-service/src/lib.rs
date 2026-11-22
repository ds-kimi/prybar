//! The operations behind every `prybar` command and MCP tool. Front ends
//! only parse arguments and print; everything here returns serializable
//! reports so both can hand agents the same JSON.

mod error;
pub mod model;
pub mod ops;
pub mod package;
pub mod source;
pub mod tools;

pub use error::{ExitCode, ServiceError};

pub type Result<T> = std::result::Result<T, ServiceError>;
