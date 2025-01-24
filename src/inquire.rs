use std::rc::Rc;

use inquire::validator::{ErrorMessage, StringValidator, Validation};
use napi::JsFunction;
use napi_derive::napi;

use crate::types::{Context, Error};

#[napi(object)]
#[derive(Default)]
pub struct SelectConfig {
  pub help_message: Option<String>,
  pub page_size: Option<u32>,
  pub reset_cursor: Option<bool>,
  pub starting_cursor: Option<u32>,
  pub starting_filter_input: Option<String>,
  pub vim_mode: Option<bool>,
  pub filtering: Option<bool>,
  pub help_message_disabled: Option<bool>,
}

#[napi]
pub fn select(
  prompt: String,
  choices: Vec<String>,
  config: Option<SelectConfig>,
) -> napi::Result<String> {
  let mut inquire = inquire::Select::new(&prompt, choices);
  let config = config.unwrap_or_default();
  if let Some(help_message) = config.help_message {
    inquire = inquire.with_help_message(help_message.leak());
  }
  if let Some(page_size) = config.page_size {
    inquire = inquire.with_page_size(page_size as usize);
  }
  if let Some(reset_cursor) = config.reset_cursor {
    inquire = inquire.with_reset_cursor(reset_cursor);
  }
  if let Some(starting_cursor) = config.starting_cursor {
    inquire = inquire.with_starting_cursor(starting_cursor as usize);
  }
  if let Some(starting_filter_input) = config.starting_filter_input {
    inquire = inquire.with_starting_filter_input(starting_filter_input.leak());
  }
  if let Some(vim_mode) = config.vim_mode {
    inquire = inquire.with_vim_mode(vim_mode);
  }
  if let Some(false) = config.filtering {
    inquire = inquire.without_filtering();
  }
  if let Some(true) = config.help_message_disabled {
    inquire = inquire.without_help_message();
  }
  Ok(inquire.prompt().map_err(Error::InquireError)?)
}

#[napi(object)]
#[derive(Default)]
pub struct CheckboxConfig {
  pub all_selected_by_default: Option<bool>,
  pub default: Option<Vec<u32>>,
  pub help_message: Option<String>,
  pub keep_filter: Option<bool>,
  pub page_size: Option<u32>,
  pub reset_cursor: Option<bool>,
  pub starting_cursor: Option<u32>,
  pub starting_filter_input: Option<String>,
  pub vim_mode: Option<bool>,
  pub filtering: Option<bool>,
  pub help_message_disabled: Option<bool>,
}

#[napi]
pub fn checkbox(
  prompt: String,
  choices: Vec<String>,
  config: Option<CheckboxConfig>,
) -> napi::Result<Vec<String>> {
  let mut inquire = inquire::MultiSelect::new(&prompt, choices);
  let config = config.unwrap_or_default();
  if let Some(true) = config.all_selected_by_default {
    inquire = inquire.with_all_selected_by_default();
  }
  if let Some(default) = config.default {
    let default = Box::new(
      default
        .into_iter()
        .map(|i| i as usize)
        .collect::<Vec<usize>>(),
    )
    .leak();
    inquire = inquire.with_default(default);
  }
  // TODO: formatter
  // if let Some(formatter) = config.formatter {
  // }
  if let Some(help_message) = config.help_message {
    inquire = inquire.with_help_message(help_message.leak());
  }
  if let Some(keep_filter) = config.keep_filter {
    inquire = inquire.with_keep_filter(keep_filter);
  }
  if let Some(page_size) = config.page_size {
    inquire = inquire.with_page_size(page_size as usize);
  }
  if let Some(reset_cursor) = config.reset_cursor {
    inquire = inquire.with_reset_cursor(reset_cursor);
  }
  // TODO: scorer
  // if let Some(scorer) = config.scorer {
  // }
  if let Some(starting_cursor) = config.starting_cursor {
    inquire = inquire.with_starting_cursor(starting_cursor as usize);
  }
  if let Some(starting_filter_input) = config.starting_filter_input {
    inquire = inquire.with_starting_filter_input(starting_filter_input.leak());
  }
  // TODO: validator
  // if let Some(validator) = config.validator {
  // }
  if let Some(vim_mode) = config.vim_mode {
    inquire = inquire.with_vim_mode(vim_mode);
  }
  if let Some(false) = config.filtering {
    inquire = inquire.without_filtering();
  }
  if let Some(true) = config.help_message_disabled {
    inquire = inquire.without_help_message();
  }
  Ok(inquire.prompt().map_err(Error::InquireError)?)
}

#[napi(object)]
#[derive(Default)]
pub struct InputConfig {
  pub default: Option<String>,
  pub formatter: Option<JsFunction>,
  pub help_message: Option<String>,
  pub initial_value: Option<String>,
  pub page_size: Option<u32>,
  pub placeholder: Option<String>,
  pub validators: Option<Vec<JsFunction>>,
}

#[napi]
pub fn input(prompt: String, config: Option<InputConfig>) -> napi::Result<String> {
  let mut inquire = inquire::Text::new(&prompt);
  let config = config.unwrap_or_default();
  if let Some(default) = config.default {
    inquire = inquire.with_default(default.leak());
  }
  if let Some(formatter) = config.formatter {
    let formatter = Box::new(move |value: &str| {
      let res: String = formatter.call1(value).unwrap_or(value.to_string());
      res
    });
    inquire = inquire.with_formatter(Box::leak(formatter));
  }
  if let Some(help_message) = config.help_message {
    inquire = inquire.with_help_message(help_message.leak());
  }
  if let Some(initial_value) = config.initial_value {
    inquire = inquire.with_initial_value(initial_value.leak());
  }
  if let Some(page_size) = config.page_size {
    inquire = inquire.with_page_size(page_size as usize);
  }
  if let Some(placeholder) = config.placeholder {
    inquire = inquire.with_placeholder(placeholder.leak());
  }
  if let Some(validators) = config.validators {
    let validators: Vec<Box<dyn StringValidator>> = validators
      .into_iter()
      .map(|f| validator(Rc::new(f)))
      .collect();
    inquire = inquire.with_validators(&validators);
  }
  Ok(inquire.prompt().map_err(Error::InquireError)?)
}

#[napi(object)]
#[derive(Default)]
pub struct ConfirmConfig {
  pub default: Option<bool>,
  pub default_value_formatter: Option<JsFunction>,
  pub error_message: Option<String>,
  pub formatter: Option<JsFunction>,
  pub help_message: Option<String>,
  pub parser: Option<JsFunction>,
  pub placeholder: Option<String>,
  pub starting_input: Option<String>,
}

#[napi]
pub fn confirm(prompt: String, config: Option<ConfirmConfig>) -> napi::Result<bool> {
  let mut inquire = inquire::Confirm::new(&prompt);
  let config = config.unwrap_or_default();
  if let Some(default) = config.default {
    inquire = inquire.with_default(default);
  }
  if let Some(default_value_formatter) = config.default_value_formatter {
    let default_value_formatter = Box::new(move |value: bool| {
      let res: String = default_value_formatter
        .call1(value)
        .unwrap_or(value.to_string());
      res
    });
    inquire = inquire.with_default_value_formatter(Box::leak(default_value_formatter));
  }
  if let Some(error_message) = config.error_message {
    inquire = inquire.with_error_message(error_message.leak());
  }
  if let Some(formatter) = config.formatter {
    let formatter = Box::new(move |value: bool| {
      let res: String = formatter.call1(value).unwrap_or(value.to_string());
      res
    });
    inquire = inquire.with_formatter(Box::leak(formatter));
  }
  if let Some(help_message) = config.help_message {
    inquire = inquire.with_help_message(help_message.leak());
  }
  if let Some(parser) = config.parser {
    let parser = Box::new(move |value: &str| {
      let res: bool = parser.call1(value).unwrap_or(false);
      Ok(res)
    });
    inquire = inquire.with_parser(Box::leak(parser));
  }
  if let Some(placeholder) = config.placeholder {
    inquire = inquire.with_placeholder(placeholder.leak());
  }
  if let Some(starting_input) = config.starting_input {
    inquire = inquire.with_starting_input(starting_input.leak());
  }
  Ok(inquire.prompt().map_err(Error::InquireError)?)
}

#[napi(object)]
#[derive(Default)]
pub struct PasswordConfig {
  pub custom_confirmation_error_message: Option<String>,
  pub custom_confirmation_message: Option<String>,
  #[napi(ts_type = "'hidden' | 'masked' | 'full'")]
  pub display_mode: Option<String>,
  pub display_toggle: Option<bool>,
  pub help_message: Option<String>,
  #[napi(ts_type = "(text: string) => string")]
  pub formatter: Option<JsFunction>,
  #[napi(ts_type = "(text: string) => StringValidatorResult")]
  pub validators: Option<Vec<JsFunction>>,
  pub confirmation: Option<bool>,
}

#[napi(object)]
pub struct StringValidatorResult {
  #[napi(ts_type = "'valid' | 'invalid'")]
  pub validation: String,
  pub err_msg: Option<String>,
}

fn validator(f: Rc<JsFunction>) -> Box<dyn StringValidator> {
  Box::new(move |text: &str| {
    let res: String = f.call1(text)?;
    Ok(match res.as_str() {
      "valid" => Validation::Valid,
      "invalid" => Validation::Invalid(ErrorMessage::Custom("invalid".to_string())),
      _ => Validation::Invalid(ErrorMessage::Custom(res)),
    })
  })
}

#[napi]
pub fn password(prompt: String, config: Option<PasswordConfig>) -> napi::Result<String> {
  let mut inquire = inquire::Password::new(&prompt);
  let config = config.unwrap_or_default();
  if let Some(custom_confirmation_error_message) = config.custom_confirmation_error_message {
    inquire =
      inquire.with_custom_confirmation_error_message(custom_confirmation_error_message.leak());
  }
  if let Some(custom_confirmation_message) = config.custom_confirmation_message {
    inquire = inquire.with_custom_confirmation_message(custom_confirmation_message.leak());
  }
  if let Some(display_mode) = config.display_mode {
    inquire = inquire.with_display_mode(match display_mode.as_str() {
      "hidden" => inquire::PasswordDisplayMode::Full,
      "masked" => inquire::PasswordDisplayMode::Masked,
      "full" => inquire::PasswordDisplayMode::Hidden,
      _ => {
        return Err(
          Error::InquireError(inquire::InquireError::InvalidConfiguration(
            "invalid password display mode".to_string(),
          ))
          .into(),
        );
      }
    });
  }
  if let Some(true) = config.display_toggle {
    inquire = inquire.with_display_toggle_enabled();
  }
  if let Some(help_message) = config.help_message {
    inquire = inquire.with_help_message(help_message.leak());
  }
  if let Some(formatter) = config.formatter {
    let formatter = Box::new(move |text: &str| {
      let res: String = formatter.call1(text).unwrap_or(text.to_string());
      res
    });
    inquire = inquire.with_formatter(Box::leak(formatter));
  }
  if let Some(false) = config.confirmation {
    inquire = inquire.without_confirmation();
  }
  if let Some(validators) = config.validators {
    let validators: Vec<Box<dyn StringValidator>> = validators
      .into_iter()
      .map(|f| validator(Rc::new(f)))
      .collect();
    inquire = inquire.with_validators(&validators);
  }
  Ok(inquire.prompt().map_err(Error::InquireError)?)
}

#[napi]
impl Context {
  #[napi]
  pub fn ask(&self, prompt: String, config: Option<InputConfig>) -> napi::Result<String> {
    input(prompt, config)
  }

  #[napi]
  pub fn confirm(&self, prompt: String, config: Option<ConfirmConfig>) -> napi::Result<bool> {
    confirm(prompt, config)
  }
}
