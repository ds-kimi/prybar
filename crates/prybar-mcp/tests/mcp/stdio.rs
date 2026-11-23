use serde_json::Value;
use std::io::{BufRead, BufReader, Write};
use std::process::{Command, Stdio};

const TOOLS: [&str; 19] = [
    "inspect_model",
    "decompile_model",
    "list_package",
    "unpack_package",
    "pack_package",
    "compile_model",
    "view_model",
    "publish_workshop",
    "games_list",
    "config_get",
    "config_set",
    "check_update",
    "workshop_status",
    "workshop_items",
    "workshop_item",
    "check_preview",
    "check_bbcode",
    "decode_texture",
    "privacy",
];

/// Replays `scripts/mcp/handshake.jsonl` (the same file a person pipes
/// into the binary by hand) and reads the two answers it asks for.
#[test]
fn stdio_handshake_lists_every_tool() {
    let script = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../scripts/mcp/handshake.jsonl"
    );
    let mut child = Command::new(env!("CARGO_BIN_EXE_prybar-mcp"))
        .env("PRYBAR_NO_UPDATE_CHECK", "1")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let mut stdin = child.stdin.take().unwrap();
    stdin.write_all(&std::fs::read(script).unwrap()).unwrap();
    let mut lines = BufReader::new(child.stdout.take().unwrap()).lines();
    let mut next = || serde_json::from_str::<Value>(&lines.next().unwrap().unwrap()).unwrap();
    let init = next();
    assert_eq!(init["result"]["serverInfo"]["name"], "prybar");
    let list = next();
    drop(stdin);
    let _ = child.wait();
    check_tools(&list["result"]["tools"]);
}

fn check_tools(tools: &Value) {
    let tools = tools.as_array().expect("tools array");
    assert_eq!(tools.len(), TOOLS.len());
    for name in TOOLS {
        let t = tools
            .iter()
            .find(|t| t["name"] == name)
            .unwrap_or_else(|| panic!("{name}"));
        assert!(
            t["description"].as_str().is_some_and(|d| d.len() > 40),
            "{name}"
        );
        assert_eq!(t["inputSchema"]["type"], "object", "{name}");
    }
}
