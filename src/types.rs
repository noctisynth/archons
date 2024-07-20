use std::collections::HashMap;

use napi::{JsFunction, JsObject};
use napi_derive::napi;

#[napi]
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[napi(object)]
pub struct Context {
  pub args: JsObject,
  pub raw_args: Vec<String>,
}

/// Command metadata
#[napi(object)]
#[derive(Clone)]
pub struct CommandMeta {
  /// Command name
  ///
  /// This is the name of the command that will be used to call it from the CLI.
  /// If the command is the main command, the name will be the name of the binary.
  /// If the command is a subcommand, the name will be the name of the subcommand.
  pub name: Option<String>,
  /// CLI version
  ///
  /// This is optional and can be used to display the version of the CLI
  /// when the command is called with the `--version` flag or `-V` option.
  ///
  /// This option will be ignored if the command is subcommand.
  pub version: Option<String>,
  /// Command description
  ///
  /// Command description will be displayed in the help output.
  pub about: Option<String>,
}

#[napi(object)]
#[derive(Clone)]
pub struct CommandOption {
  #[napi(js_name = "type", ts_type = "'positional' | 'flag' | 'option'")]
  pub _type: Option<String>,
  #[napi(ts_type = r#"
    | 'string'
    | 'str'
    | 'string[]'
    | 'str[]'
    | 'number'
    | 'boolean'
    | 'bool'"#)]
  pub parser: Option<String>,
  pub short: Option<String>,
  pub long: Option<String>,
  pub alias: Option<Vec<String>>,
  pub hidden_alias: Option<Vec<String>>,
  pub required: Option<bool>,
  pub default: Option<String>,
  pub hidden: Option<bool>,
}

#[napi(object)]
pub struct Command {
  pub meta: CommandMeta,
  pub options: HashMap<String, CommandOption>,
  #[napi(ts_type = "(ctx: Context) => void")]
  pub callback: Option<JsFunction>,
  pub subcommands: Option<HashMap<String, Command>>,
}
