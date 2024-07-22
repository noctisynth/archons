use std::collections::HashMap;

use napi::{JsFunction, JsObject};
use napi_derive::napi;

/// Command context
///
/// This is the context object that is passed to the command callback.
#[napi(object)]
pub struct Context {
  /// Parsed arguments
  ///
  /// This is a js object that contains the parsed arguments.
  /// The keys of the object are the names of the arguments and
  /// the values are the parsed values.
  pub args: JsObject,
  /// Raw arguments
  ///
  /// The raw arguments parsed by command line or manually given.
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
  /// If not provided, the CLI will not display the version and you can't
  /// call the command with the `--version` flag or `-V` option.
  pub version: Option<String>,
  /// Command description
  ///
  /// Command description will be displayed in the help output.
  pub about: Option<String>,
  /// Enable styled mode
  ///
  /// Determines whether the CLI output should be displayed in the styled format.
  pub styled: Option<bool>,
}

#[napi(object)]
#[derive(Clone)]
pub struct CommandOption {
  #[napi(js_name = "type", ts_type = "'positional' | 'flag' | 'option'")]
  pub _type: Option<String>,
  #[napi(ts_type = "'string' | 'number' | 'boolean'")]
  pub parser: Option<String>,
  #[napi(ts_type = "'set' | 'append' | 'count' | 'store' | 'store_false'")]
  pub action: Option<String>,
  #[napi(ts_type = "string & { length: 1 }")]
  pub short: Option<String>,
  pub long: Option<String>,
  pub alias: Option<Vec<String>>,
  pub hidden_alias: Option<Vec<String>>,
  pub help: Option<String>,
  pub required: Option<bool>,
  pub default: Option<String>,
  pub hidden: Option<bool>,
  pub conflicts_with: Option<Vec<String>>,
}

/// Command definition
///
/// This is the object that defines a command.
/// It contains the metadata, options, and callback function.
#[napi(object)]
pub struct Command {
  pub meta: CommandMeta,
  pub options: HashMap<String, CommandOption>,
  #[napi(ts_type = "(ctx: Context) => void")]
  pub callback: Option<JsFunction>,
  pub subcommands: Option<HashMap<String, Command>>,
}
