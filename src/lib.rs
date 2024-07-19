#![deny(clippy::all)]

use std::collections::HashMap;

use napi::JsFunction;
use napi_derive::napi;

#[napi(object)]
pub struct Context {}

#[napi(object)]
pub struct CommandMeta {
  pub name: Option<String>,
  pub version: Option<String>,
  pub description: Option<String>,
  pub hidden: Option<bool>,
}

#[napi(object)]
pub struct CommandOption {}

#[napi(object)]
pub struct Command {
  pub meta: CommandMeta,
  pub options: HashMap<String, CommandOption>,
  #[napi(ts_type = "(ctx: Context) => void")]
  pub callback: JsFunction,
}

#[napi]
pub fn define_command(options: Command) -> Command {
  options
}
