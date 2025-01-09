use napi::{JsFunction, JsObject};
use napi_derive::napi;

use crate::HashMap;

/// Command context
///
/// This is the context object that is passed to the command callback.
#[napi]
pub struct Context {
  /// Parsed arguments
  ///
  /// This is a js object that contains the parsed arguments.
  /// The keys of the object are the names of the arguments and
  /// the values are the parsed values.
  args: JsObject,
  /// Raw arguments
  ///
  /// The raw arguments parsed by command line or manually given.
  #[napi(ts_type = "string[]")]
  pub raw_args: Vec<String>,
}

#[napi]
impl Context {
  /// Create a new command context
  ///
  /// This method is used to create a new command context,
  /// and is not intended to be used directly.
  ///
  /// @param args - Parsed arguments
  /// @param raw_args - Raw arguments
  #[napi(
    constructor,
    ts_args_type = "args: Record<string, any>, raw_args: string[]"
  )]
  pub fn new(args: JsObject, raw_args: Vec<String>) -> Self {
    Self { args, raw_args }
  }

  /// Get the parsed arguments
  #[napi(getter, ts_return_type = "Record<string, any>")]
  pub fn args(&self) -> &JsObject {
    &self.args
  }

  /// Get the raw arguments
  #[napi(ts_return_type = "string[]")]
  pub fn get_raw_args(&self) -> &Vec<String> {
    &self.raw_args
  }

  /// Get the argument value by specified key
  #[napi(ts_return_type = "any")]
  pub fn get(&self, key: String) -> napi::Result<JsObject> {
    self.args.get_named_property(&key)
  }
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
  /// Option type for argument
  ///
  /// `type` option and `action` option are used to specify how to parse the argument.
  ///
  /// - `option` and (`store` or `store_false`): Boolean flag
  /// - `option` and  `count`: Counter flag
  /// - `option` and `set`: Option flag
  /// - `option` and `append`: Multiple option flag
  /// - `positional` and `set`: Positional argument
  /// - `positional` and `append`: Multiple positional argument
  ///
  /// Defaults to `option` if not specified.
  #[napi(js_name = "type", ts_type = "'positional' | 'option'")]
  pub type_: Option<String>,
  /// Specify the value type for the argument.
  #[napi(ts_type = "'string' | 'number' | 'boolean'")]
  pub parser: Option<String>,
  /// Specify how to react to an argument when parsing it.
  ///
  /// - `set`: Overwrite previous values with new ones
  /// - `append`: Append new values to all previous ones
  /// - `count`: Count how many times a flag occurs
  /// - `store`: Store the value as a boolean flag
  /// - `store_false`: Store the value as a boolean flag with opposite meaning
  ///
  /// Defaults to `set` if not specified.
  #[napi(ts_type = "'set' | 'append' | 'count' | 'store' | 'store_false'")]
  pub action: Option<String>,
  /// Short option name
  ///
  /// This is a single character that can be used to represent the option
  /// in the command line. For example, `-v` for the `--verbose` option.
  /// Panics if the length of the string is empty. If the size of string
  /// is greater than 1, the first character will be used as the short option.
  ///
  /// This option will be ignored if option `type` is not `option`.
  ///
  /// Defaults to the first character of the long option name.
  pub short: Option<String>,
  /// Long option name
  ///
  /// This is the name of the option that will be used to represent the option,
  /// preceded by two dashes. For example, `--verbose` option.
  ///
  /// This option will be ignored if option `type` is not `option`.
  ///
  /// Defaults to the name of the argument.
  pub long: Option<String>,
  /// Option aliases
  pub alias: Option<Vec<&'static str>>,
  /// Hidden option aliases
  pub hidden_alias: Option<Vec<&'static str>>,
  /// Short option aliases
  pub short_alias: Option<Vec<String>>,
  /// Hidden short option aliases
  pub hidden_short_alias: Option<Vec<String>>,
  /// Value hint for shell completion
  ///
  /// Provide the shell a hint about how to complete this argument.
  ///
  /// **Warning**: this will implicitly set `action` to `set`.
  #[napi(ts_type = r#"
    | 'any_path'
    | 'file'
    | 'dir'
    | 'executable'
    | 'cmd_name'
    | 'cmd'
    | 'cmd_with_args'
    | 'url'
    | 'username'
    | 'hostname'
    | 'email'"#)]
  pub value_hint: Option<String>,
  /// Option description
  pub help: Option<&'static str>,
  /// Required argument
  ///
  /// If true, the argument is required and the command will fail without it.
  pub required: Option<bool>,
  /// Value for the argument when not present
  pub default: Option<&'static str>,
  /// Value for the argument when the flag is present but no value is specified.
  ///
  /// This configuration option is often used to give the user a shortcut and
  /// allow them to efficiently specify an option argument without requiring an
  /// explicitly value. The `--color` argument is a common example. By supplying
  /// a default, such as `default_missing_value("always")`, the user can quickly
  /// just add `--color` to the command line to produce the desired color output.
  pub default_missing: Option<&'static str>,
  /// Limit the count of values for the argument
  ///
  /// If the expected number of parameters required is a fixed value, pass in the
  /// number directly. If you want to limit the number of values to a range, for
  /// example, pass `1..5` or `1..=4` to specify a range from 1 to 4 inclusive.
  pub num_args: Option<&'static str>,
  /// Requires that options use the `--option=val` syntax
  pub required_equals: Option<bool>,
  /// Hide argument in help output
  ///
  /// Do not display the argument in the help message.
  pub hidden: Option<bool>,
  /// Global argument
  ///
  /// Specifies that an argument can be matched to all child subcommands
  pub global: Option<bool>,
  /// Options that conflict with this argument
  ///
  /// This argument is mutually exclusive with the specified arguments.
  pub conflicts_with: Option<Vec<&'static str>>,
  /// Exclusive argument
  ///
  /// This argument must be passed alone; it conflicts with all other arguments.
  pub exclusive: Option<bool>,
  /// Hide default value in help output
  ///
  /// Do not display the default value of the argument in the help message.
  ///
  /// This is useful when default behavior of an arg is explained elsewhere
  /// in the help text.
  pub hide_default_value: Option<bool>,
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
