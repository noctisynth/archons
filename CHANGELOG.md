# archons

## 0.2.2

### Patch Changes

- 4133af7: Support `exclusive` field for arguments
- 4133af7: Support set `value_hint` for arguments

## 0.2.1

### Patch Changes

- 2c3ce3f: Wrap `parse_arguments` to inner function
- 2c3ce3f: Fix global option can't be downcast

## 0.2.0

### Minor Changes

- bbfb42c: Refactor arguments parsing policy and support global options

### Patch Changes

- bbfb42c: Fix context args annotations to `Record<string, any>`
- bbfb42c: Refactor merge arguments matches policy
- bbfb42c: Remove `{ length: 1}` annotation
- bbfb42c: Refactor `Vec<String>` to `Vec<&str>`
- 1b5d6b7: Fix args resolver in musl systems
- bbfb42c: Improve parser resolver and determine default parser by action

## 0.1.7

### Patch Changes

- fa1b5c6: Fix prepublish package names
- be34ebc: Refactor codes to extract resolvers to single module
- f4e7ff9: Support more options for command option
- f4e7ff9: Support action option and resolve by str
- f4e7ff9: Support `conflicts_with` in command option definition
- f4e7ff9: Improvements of type annotations for command option
- f4e7ff9: Add `default_missing` arg for option

## 0.1.6

### Patch Changes

- d565ad4: Release v0.1.6
- df6f8bf: Remove `VERSION` const from `Cargo.toml`

## 0.1.5

### Patch Changes

- 3382c3a: Add `--js-package-name` option in build script

## 0.1.4

### Patch Changes

- 8f34ec5: Support pass help option for args

## 0.1.3

### Patch Changes

- 0968fe3: Fix napi package failed to release

## 0.1.2

### Patch Changes

- bcee4ef: Support styled option for command meta

## 0.1.1

### Patch Changes

- c3e676e: Set color choice to always
- c3e676e: Implement utils functions to create clap command instance
- c3e676e: Call callback functions with context object after merged parsed args
- c3e676e: Merge parsed args by `clap-rs` to context args object
- c3e676e: Remove features for clap-rs
- c3e676e: Add `defineCommand` interface
- c3e676e: Implement command definition types
