use anyhow::Result;
use serde_json::Value;

use crate::cli::JsonArgs;
use crate::util::input_or_stdin;

pub fn run(args: &JsonArgs) -> Result<()> {
    let (input, pretty) = match &args.command {
        crate::cli::JsonCommand::Pretty { json } => (input_or_stdin(json.as_deref())?, true),
        crate::cli::JsonCommand::Compact { json } => (input_or_stdin(json.as_deref())?, false),
    };
    let value: Value = serde_json::from_str(&input)
        .map_err(|e| anyhow::anyhow!("无效的 JSON: {e}"))?;
    if pretty {
        println!("{}", serde_json::to_string_pretty(&value)?);
    } else {
        println!("{}", serde_json::to_string(&value)?);
    }
    Ok(())
}
