use crate::guard;
use crate::params::{DecompileArgs, InspectArgs};
use crate::reply::{blocking, failure};
use crate::server::Prybar;
use prybar_service::model::{self, DecompileOptions, DecompileReport};
use prybar_service::ServiceError;
use rmcp::handler::server::wrapper::Parameters;
use rmcp::model::CallToolResult;
use rmcp::{tool, tool_router};
use serde_json::{json, Value};

const MAX_INPUTS: usize = 64;
/// Folder and package inputs can expand to thousands of models; the
/// agent gets counts plus a sample, the files are on disk anyway.
const SHOWN: usize = 50;

#[tool_router(router = model_router, vis = "pub(crate)")]
impl Prybar {
    #[tool(
        description = "Read a model's header without writing files: version, name, bone/mesh/sequence/material counts and which companion files (.vvd, .vtx, .phy, .ani) were found.",
        annotations(read_only_hint = true, open_world_hint = false)
    )]
    async fn inspect_model(&self, Parameters(a): Parameters<InspectArgs>) -> CallToolResult {
        match guard::model_arg("model", &a.model) {
            Ok(m) => blocking(move || model::inspect(&m)).await,
            Err(e) => failure(&e),
        }
    }

    #[tool(
        description = "Decompile models to QC + SMD (+ VTA flexes, VRD procedural bones) source files; existing files of the same name in `out` are overwritten. Models are processed in parallel; one bad model does not stop the others. Companions (.vvd, .vtx, .phy, .ani) must sit next to each .mdl; Vampire: Bloodlines MDL 2531 has no .vvd and needs only its .dx80.vtx. The output mirrors the input tree (`flat` for one folder per model name) and models named by `$includemodel` are decompiled too (`with_includes`, default true; unresolved ones become warnings). Returns counts, the first 50 results (included ones carry `included_from`), failures and warnings.",
        annotations(
            destructive_hint = false,
            idempotent_hint = true,
            open_world_hint = false
        )
    )]
    async fn decompile_model(&self, Parameters(a): Parameters<DecompileArgs>) -> CallToolResult {
        let opts = match decompile_options(&a) {
            Ok(o) => o,
            Err(e) => return failure(&e),
        };
        let inputs = a.inputs;
        blocking(move || model::decompile(&inputs, &opts, &|_| {}).map(summary)).await
    }
}

fn decompile_options(a: &DecompileArgs) -> prybar_service::Result<DecompileOptions> {
    if a.inputs.is_empty() || a.inputs.len() > MAX_INPUTS {
        return Err(ServiceError::Usage(format!(
            "inputs: expected 1 to {MAX_INPUTS} items"
        )));
    }
    for (i, input) in a.inputs.iter().enumerate() {
        guard::model_arg(&format!("inputs[{i}]"), input)?;
    }
    Ok(DecompileOptions {
        out: guard::absolute("out", &a.out)?,
        recursive: a.recursive,
        split_qc: a.split_qc,
        flat: a.flat,
        with_includes: a.with_includes,
    })
}

fn summary(r: DecompileReport) -> Value {
    json!({
        "decompiled": r.models.len(),
        "failed": r.failures.len(),
        "truncated": r.models.len() > SHOWN || r.failures.len() > SHOWN,
        "models": &r.models[..r.models.len().min(SHOWN)],
        "failures": &r.failures[..r.failures.len().min(SHOWN)],
        "warnings": &r.warnings[..r.warnings.len().min(SHOWN)],
    })
}
