use crate::guard;
use crate::params::{ListArgs, PackArgs, UnpackArgs};
use crate::reply::{blocking, failure};
use crate::server::Prybar;
use prybar_service::ops::{self, PackRequest};
use rmcp::handler::server::wrapper::Parameters;
use rmcp::model::CallToolResult;
use rmcp::{tool, tool_router};

const DEFAULT_LIMIT: usize = 200;
const MAX_LIMIT: usize = 5000;

#[tool_router(router = package_router, vis = "pub(crate)")]
impl Prybar {
    #[tool(
        description = "List the files in a VPK, GMA, APK or HFS package, sorted by path, with sizes. Use `filter` to narrow big game VPKs; `total` and `total_bytes` count every match even when `truncated`.",
        annotations(read_only_hint = true, open_world_hint = false)
    )]
    async fn list_package(&self, Parameters(a): Parameters<ListArgs>) -> CallToolResult {
        let package = match guard::absolute("package", &a.package) {
            Ok(p) => p,
            Err(e) => return failure(&e),
        };
        let limit = guard::limit(a.limit, DEFAULT_LIMIT, MAX_LIMIT);
        blocking(move || ops::list(&package, a.filter.as_deref(), Some(limit))).await
    }

    #[tool(
        description = "Extract every file of a VPK, GMA, APK or HFS package into a folder. Entry paths that would escape the folder are refused. Returns the file count, not the file list (use list_package for that).",
        annotations(
            destructive_hint = true,
            idempotent_hint = true,
            open_world_hint = false
        )
    )]
    async fn unpack_package(&self, Parameters(a): Parameters<UnpackArgs>) -> CallToolResult {
        let paths = guard::absolute("package", &a.package)
            .and_then(|p| Ok((p, guard::absolute("out", &a.out)?)));
        match paths {
            Ok((package, out)) => blocking(move || ops::unpack(&package, &out)).await,
            Err(e) => failure(&e),
        }
    }

    #[tool(
        description = "Pack a folder into a new VPK (v1 or v2, optionally split into numbered archives) or GMA package (the folder must then hold an `addon.json` with `title` and `type`). Overwrites `out`.",
        annotations(
            destructive_hint = true,
            idempotent_hint = true,
            open_world_hint = false
        )
    )]
    async fn pack_package(&self, Parameters(a): Parameters<PackArgs>) -> CallToolResult {
        let req = guard::absolute("src", &a.src).and_then(|src| {
            Ok(PackRequest {
                src,
                out: guard::absolute("out", &a.out)?,
                vpk_version: a.vpk_version.unwrap_or(2),
                chunk_size: a.chunk_size,
            })
        });
        match req {
            Ok(req) => blocking(move || ops::pack(&req)).await,
            Err(e) => failure(&e),
        }
    }
}
