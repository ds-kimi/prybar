//! End-to-end runs of the `prybar` binary against synthetic fixtures.

mod agent;
mod batch;
mod fixture;
mod model;
mod package;
mod packed;
mod tools;
mod update;
mod workshop;

#[path = "../../../prybar-mdl/tests/common/mod.rs"]
mod mdl_common;
#[path = "../../../prybar-vtf/tests/support/mod.rs"]
mod vtf_support;
