//! The prybar MCP server: every prybar operation as a tool with a JSON
//! schema, over the shared `prybar-service` layer the CLI also uses.

mod guard;
mod params;
mod reply;
mod server;
mod tools;

pub use server::Prybar;
