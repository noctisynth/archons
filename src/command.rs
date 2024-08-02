use napi::{Env, Result};
use napi_derive::napi;

use crate::resolver::{resolve_command, resolve_option_args};
use crate::types::Command;
use crate::utils::parse_arguments;

#[napi]
pub fn define_command(options: Command) -> Command {
  options
}

/// Run command
///
/// **NOTE**: If the given `args` is empty, it will use `process.argv` instead.
///
/// **NOTE**: The given `args` should include the nodejs executable and script name.
/// For example, if you are running a script `index.js` in the current directory with
/// a flag `--foo`, you should pass `["node", "index.js", "--foo"]` as `args`.
///
/// @param cmd Command object
/// @param args Run with given arguments
/// @returns {void}
#[napi]
pub fn run(env: Env, cmd: Command, args: Option<Vec<String>>) -> Result<()> {
  let raw_args = resolve_option_args(env, args)?;
  let clap = resolve_command(clap::Command::default(), Default::default(), &cmd);
  let matches = clap.clone().get_matches_from(&raw_args);

  parse_arguments(env, &clap, cmd, &matches, raw_args)
}
