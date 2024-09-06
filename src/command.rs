use napi::{Env, Result};
use napi_derive::napi;

use crate::resolver::{resolve_command, resolve_option_args};
use crate::types::Command;
use crate::utils::parse_arguments;

/// Define a command functionally
///
/// @param options Command options
/// @returns {Command}
#[napi]
pub fn define_command(options: Command) -> Command {
  options
}

#[cfg(test)]
mod tests {
  use crate::command::{define_command, Command};
  use crate::types::CommandMeta;

  #[test]
  fn test_define_command() {
    let meta = CommandMeta {
      name: Some("test".to_string()),
      version: Some("1.0.0".to_string()),
      about: Some("test case".to_string()),
      styled: Some(false),
    };
    let cmd = Command {
      meta: meta.clone(),
      options: Default::default(),
      callback: None,
      subcommands: None,
    };

    let defined_cmd = define_command(cmd);

    assert_eq!(defined_cmd.meta, meta);
  }
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
