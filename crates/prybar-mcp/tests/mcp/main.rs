//! The MCP server driven the way an agent drives it: an rmcp client over
//! an in-memory pipe for every tool, plus the real binary over stdio.

mod client;
mod config;
mod external;
mod model;
mod package;
mod stdio;
mod update;
mod workshop;

#[path = "../../../prybar-mdl/tests/common/mod.rs"]
mod mdl_common;
