pub mod command;
pub mod inquire;
pub mod progressbar;
pub mod resolver;
pub mod types;
pub mod utils;

pub type HashMap<K, V> = rustc_hash::FxHashMap<K, V>;

#[macro_export]
macro_rules! apply_opt {
  ($arg:ident, $opt:ident, $field:ident => $method:ident) => {
    if let Some(val) = $opt.$field {
      $arg = $arg.$method(val);
    }
  };
  ($arg:ident, $opt:ident, &$field:ident => $method:ident) => {
    if let Some(val) = &$opt.$field {
      $arg = $arg.$method(val);
    }
  };
  ($arg:ident, $opt:ident, $wrapper:ident($field:ident) => $method:ident) => {
    if let Some(val) = $opt.$field {
      $arg = $arg.$method($wrapper(val));
    }
  };
  ($arg:ident, $opt:ident, $wrapper:ident(&$field:ident) => $method:ident) => {
    if let Some(val) = &$opt.$field {
      $arg = $arg.$method($wrapper(val));
    }
  };
  ($arg:ident, $opt:ident, $wrapper:ident!($field:ident) => $method:ident) => {
    if let Some(val) = $opt.$field {
      $arg = $arg.$method($wrapper!(val));
    }
  };
  ($arg:ident, $opt:ident, $wrapper:ident!(&$field:ident) => $method:ident) => {
    if let Some(val) = &$opt.$field {
      $arg = $arg.$method($wrapper!(val));
    }
  };
  ($arg:ident, $opt:ident, $field:ident) => {
    apply_opt!($arg, $opt, $field => $field);
  };
  ($arg:ident, $opt:ident, &$field:ident) => {
    apply_opt!($arg, $opt, &$field => $field);
  };
}

#[macro_export]
macro_rules! to_char_vec {
  ($vec:ident) => {
    $vec
      .into_iter()
      .map(|c| c.chars().next().unwrap())
      .collect::<Vec<char>>()
  };
}
