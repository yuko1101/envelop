use std::path::PathBuf;

use anyhow::Context;
use base64::{Engine, prelude::BASE64_STANDARD};

use crate::{
    OutputFormat,
    env::{Env, EvelopContext},
};

pub fn make_wrapper(
    target: PathBuf,
    output: PathBuf,
    env: Env,
    format: OutputFormat,
) -> anyhow::Result<()> {
    match format {
        OutputFormat::Nushell => make_nushell_wrapper(target, output, env),
    }
}

const NUSHELL_WRAPPER_TEMPLATE: &str = include_str!("../assets/nushell_wrapper_template.nu");
fn make_nushell_wrapper(target: PathBuf, output: PathBuf, env: Env) -> anyhow::Result<()> {
    let nu = std::env::var("NU_PATH").context("NU_PATH environment variable is not set")?;
    let expanded_env = env.into_expanded();

    let context = EvelopContext {
        target,
        env: expanded_env,
    };
    let context = serde_json::to_string(&context)
        .context("Failed to serialize the envelop context for the wrapper")?;

    let script = NUSHELL_WRAPPER_TEMPLATE
        .replace("%NU_PATH%", &nu)
        .replace("%CONTEXT%", &BASE64_STANDARD.encode(context.as_bytes()));

    std::fs::write(&output, script)
        .with_context(|| format!("Failed to write the wrapper to {}", output.display()))?;

    Ok(())
}
