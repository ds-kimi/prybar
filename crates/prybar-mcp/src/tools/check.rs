use crate::guard;
use crate::params::{BbcodeArgs, PreviewArgs, PrivacyArgs, TextureArgs};
use crate::reply::{blocking, failure};
use crate::server::Prybar;
use prybar_service::{privacy, texture, workshop, ServiceError};
use rmcp::handler::server::wrapper::Parameters;
use rmcp::model::CallToolResult;
use rmcp::{tool, tool_router};

#[tool_router(router = check_router, vis = "pub(crate)")]
impl Prybar {
    #[tool(
        description = "Check a Workshop preview image without uploading: width, height, bytes, `resized` (Steam gets a 512x512 center crop unless it is already 512x512) and `problem` when it would be refused (over 32 MB; a GIF must be under 1 MB since it is sent as is to stay animated).",
        annotations(read_only_hint = true, open_world_hint = false)
    )]
    async fn check_preview(&self, Parameters(a): Parameters<PreviewArgs>) -> CallToolResult {
        match guard::absolute("image", &a.image) {
            Ok(p) => blocking(move || workshop::check_preview(&p)).await,
            Err(e) => failure(&e),
        }
    }

    #[tool(
        description = "Check a Workshop description written in Steam BBCode: `chars` against `max_chars` (7999), `too_long`, the `tags` that parse (unknown or unclosed tags show as text on Steam, as here) and `plain`, the text a reader sees with list bullets and link targets kept.",
        annotations(read_only_hint = true, open_world_hint = false)
    )]
    async fn check_bbcode(&self, Parameters(a): Parameters<BbcodeArgs>) -> CallToolResult {
        blocking(move || Ok(workshop::check_bbcode(&a.text))).await
    }

    #[tool(
        description = "Decode a VTF texture to a PNG file: a `.vtf` on disk, or a model's material found like the engine does (model tree, its package, configured games). Returns path, location, format (e.g. DXT1), mips, width, height and the PNG path.",
        annotations(
            destructive_hint = true,
            idempotent_hint = true,
            open_world_hint = false
        )
    )]
    async fn decode_texture(&self, Parameters(a): Parameters<TextureArgs>) -> CallToolResult {
        let out = match guard::absolute("out", &a.out) {
            Ok(o) => o,
            Err(e) => return failure(&e),
        };
        blocking(move || match a.material {
            Some(m) => {
                texture::export_material(&guard::model_arg("input", &a.input)?, &m, a.level, &out)
            }
            None => texture::export_file(&guard::absolute("input", &a.input)?, a.level, &out),
        })
        .await
    }

    #[tool(
        description = "Read the desktop app's anonymous analytics choice (`on`, `blocked_by` an env switch such as DO_NOT_TRACK) and its local log of every payload queued or sent. `set` turns them on or off, `clear_log` empties the log; both need `confirm: true`. The CLI and MCP server never send analytics.",
        annotations(
            destructive_hint = true,
            idempotent_hint = true,
            open_world_hint = false
        )
    )]
    async fn privacy(&self, Parameters(a): Parameters<PrivacyArgs>) -> CallToolResult {
        if (a.set.is_some() || a.clear_log) && !a.confirm {
            return failure(&ServiceError::Usage(
                "changing privacy settings needs `confirm: true`; ask the user first".into(),
            ));
        }
        blocking(move || privacy::privacy(a.set, a.clear_log)).await
    }
}
