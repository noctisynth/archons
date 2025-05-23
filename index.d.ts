/* tslint:disable */
/* eslint-disable */

/* auto-generated by NAPI-RS */

/**
 * Define a command functionally
 *
 * @param options Command options
 * @returns {Command}
 */
export declare function defineCommand(options: Command): Command
/**
 * Run command
 *
 * **NOTE**: If the given `args` is empty, it will use `process.argv`
 * (or `Deno.args` in Deno.js environment) instead.
 *
 * **NOTE**: The given `args` should include the nodejs executable and script name.
 * For example, if you are running a script `index.js` in the current directory with
 * a flag `--foo`, you should pass `["node", "index.js", "--foo"]` as `args`.
 *
 * @param cmd Command object
 * @param args Run with given arguments
 * @returns {void}
 */
export declare function run(cmd: Command, args?: string[]): void
export interface SelectConfig {
  helpMessage?: string
  pageSize?: number
  resetCursor?: boolean
  startingCursor?: number
  startingFilterInput?: string
  vimMode?: boolean
  filtering?: boolean
  helpMessageDisabled?: boolean
}
export declare function select(prompt: string, choices: Array<string>, config?: SelectConfig | undefined | null): string
export interface CheckboxConfig {
  allSelectedByDefault?: boolean
  default?: Array<number>
  helpMessage?: string
  keepFilter?: boolean
  pageSize?: number
  resetCursor?: boolean
  startingCursor?: number
  startingFilterInput?: string
  vimMode?: boolean
  filtering?: boolean
  helpMessageDisabled?: boolean
}
export declare function checkbox(
  prompt: string,
  choices: Array<string>,
  config?: CheckboxConfig | undefined | null,
): Array<string>
export interface InputConfig {
  default?: string
  formatter?: (value: string) => string
  helpMessage?: string
  initialValue?: string
  pageSize?: number
  placeholder?: string
  validators?: ((text: string) => StringValidatorResult)[]
}
export declare function input(prompt: string, config?: InputConfig | undefined | null): string
export interface ConfirmConfig {
  default?: boolean
  defaultValueFormatter?: (value: boolean) => string
  errorMessage?: string
  formatter?: (value: boolean) => string
  helpMessage?: string
  parser?: (value: boolean) => boolean
  placeholder?: string
  startingInput?: string
}
export declare function confirm(prompt: string, config?: ConfirmConfig | undefined | null): boolean
export interface PasswordConfig {
  customConfirmationErrorMessage?: string
  customConfirmationMessage?: string
  displayMode?: 'hidden' | 'masked' | 'full'
  displayToggle?: boolean
  helpMessage?: string
  formatter?: (text: string) => string
  validators?: ((text: string) => StringValidatorResult)[]
  confirmation?: boolean
}
export interface StringValidatorResult {
  validation: 'valid' | 'invalid'
  errMsg?: string
}
export declare function password(prompt: string, config?: PasswordConfig | undefined | null): string
/**
 * Creates a new progress bar with the specified total number of steps.
 *
 * # Arguments
 *
 * * `total` - The total number of steps for the progress bar.
 *
 * # Returns
 *
 * A new `ProgressBar` instance.
 */
export declare function createProgressBar(total: number): ProgressBar
/**
 * Creates a new spinner progress bar.
 *
 * # Returns
 *
 * A new `ProgressBar` instance with a spinner style.
 */
export declare function createSpinner(): ProgressBar
/** Command metadata */
export interface CommandMeta {
  /**
   * Command name
   *
   * This is the name of the command that will be used to call it from the CLI.
   * If the command is the main command, the name will be the name of the binary.
   * If the command is a subcommand, the name will be the name of the subcommand.
   */
  name?: string
  /**
   * CLI version
   *
   * This is optional and can be used to display the version of the CLI
   * when the command is called with the `--version` flag or `-V` option.
   *
   * If not provided, the CLI will not display the version and you can't
   * call the command with the `--version` flag or `-V` option.
   */
  version?: string
  /**
   * Command description
   *
   * Command description will be displayed in the help output.
   */
  about?: string
  /**
   * Enable styled mode
   *
   * Determines whether the CLI output should be displayed in the styled format.
   */
  styled?: boolean
  /**
   * Subcommand required
   *
   * If true, the command will fail if no subcommand is provided.
   */
  subcommandRequired?: boolean
}
export interface CommandOption {
  /**
   * Option type for argument
   *
   * `type` option and `action` option are used to specify how to parse the argument.
   *
   * - `option` and (`store` or `store_false`): Boolean flag
   * - `option` and  `count`: Counter flag
   * - `option` and `set`: Option flag
   * - `option` and `append`: Multiple option flag
   * - `positional` and `set`: Positional argument
   * - `positional` and `append`: Multiple positional argument
   *
   * Defaults to `option` if not specified.
   */
  type?: 'positional' | 'option'
  /** Specify the value type for the argument. */
  parser?: 'string' | 'number' | 'boolean'
  /**
   * Specify how to react to an argument when parsing it.
   *
   * - `set`: Overwrite previous values with new ones
   * - `append`: Append new values to all previous ones
   * - `count`: Count how many times a flag occurs
   * - `store`: Store the value as a boolean flag
   * - `store_false`: Store the value as a boolean flag with opposite meaning
   *
   * Defaults to `set` if not specified.
   */
  action?: 'set' | 'append' | 'count' | 'store' | 'store_false'
  /**
   * Short option name
   *
   * This is a single character that can be used to represent the option
   * in the command line. For example, `-v` for the `--verbose` option.
   * Panics if the length of the string is empty. If the size of string
   * is greater than 1, the first character will be used as the short option.
   *
   * This option will be ignored if option `type` is not `option`.
   *
   * Defaults to the first character of the long option name.
   */
  short?: string
  /**
   * Long option name
   *
   * This is the name of the option that will be used to represent the option,
   * preceded by two dashes. For example, `--verbose` option.
   *
   * This option will be ignored if option `type` is not `option`.
   *
   * Defaults to the name of the argument.
   */
  long?: string
  /** Option aliases */
  alias?: Array<string>
  /** Hidden option aliases */
  hiddenAlias?: Array<string>
  /** Short option aliases */
  shortAlias?: Array<string>
  /** Hidden short option aliases */
  hiddenShortAlias?: Array<string>
  /**
   * Value hint for shell completion
   *
   * Provide the shell a hint about how to complete this argument.
   *
   * **Warning**: this will implicitly set `action` to `set`.
   */
  valueHint?:
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
    | 'email'
  /** Option description */
  help?: string
  /**
   * Required argument
   *
   * If true, the argument is required and the command will fail without it.
   */
  required?: boolean
  /** Value for the argument when not present */
  default?: string
  /**
   * Value for the argument when the flag is present but no value is specified.
   *
   * This configuration option is often used to give the user a shortcut and
   * allow them to efficiently specify an option argument without requiring an
   * explicitly value. The `--color` argument is a common example. By supplying
   * a default, such as `default_missing_value("always")`, the user can quickly
   * just add `--color` to the command line to produce the desired color output.
   */
  defaultMissing?: string
  /**
   * Limit the count of values for the argument
   *
   * If the expected number of parameters required is a fixed value, pass in the
   * number directly. If you want to limit the number of values to a range, for
   * example, pass `1..5` or `1..=4` to specify a range from 1 to 4 inclusive.
   */
  numArgs?: string
  /** Requires that options use the `--option=val` syntax */
  requiredEquals?: boolean
  /**
   * Hide argument in help output
   *
   * Do not display the argument in the help message.
   */
  hidden?: boolean
  /**
   * Global argument
   *
   * Specifies that an argument can be matched to all child subcommands
   */
  global?: boolean
  /**
   * Options that conflict with this argument
   *
   * This argument is mutually exclusive with the specified arguments.
   */
  conflictsWith?: Array<string>
  /**
   * Exclusive argument
   *
   * This argument must be passed alone; it conflicts with all other arguments.
   */
  exclusive?: boolean
  /**
   * Hide default value in help output
   *
   * Do not display the default value of the argument in the help message.
   *
   * This is useful when default behavior of an arg is explained elsewhere
   * in the help text.
   */
  hideDefaultValue?: boolean
}
/**
 * Command definition
 *
 * This is the object that defines a command.
 * It contains the metadata, options, and callback function.
 */
export interface Command {
  meta: CommandMeta
  options: Record<string, CommandOption>
  callback?: (ctx: Context) => void
  subcommands?: Record<string, Command>
}
export declare class ProgressBar {
  finish(): void
  finishAndClear(): void
  finishUsingStyle(): void
  finishWithMessage(msg: string): void
  setPosition(pos: number): void
  setLength(len: number): void
  setMessage(msg: string): void
  setPrefix(prefix: string): void
  setTabWidth(width: number): void
  setTemplate(template: string): void
  setTickStrings(s: Array<string>): void
  setProgressChars(s: string): void
  tick(): void
  abandon(): void
  abandonWithMessage(msg: string): void
  inc(delta: number): void
  incLength(delta: number): void
  reset(): void
  println(msg: string): void
  suspend(f: () => void): void
  enableSteadyTick(ms: number): void
  disableSteadyTick(): void
}
/**
 * Command context
 *
 * This is the context object that is passed to the command callback.
 */
export declare class Context {
  /**
   * Raw arguments
   *
   * The raw arguments parsed by command line or manually given.
   */
  rawArgs: string[]
  ask(prompt: string, config?: InputConfig | undefined | null): string
  confirm(prompt: string, config?: ConfirmConfig | undefined | null): boolean
  createProgressBar(total: number): ProgressBar
  createSpinner(): ProgressBar
  /**
   * Create a new command context
   *
   * This method is used to create a new command context,
   * and is not intended to be used directly.
   *
   * @param args - Parsed arguments
   * @param raw_args - Raw arguments
   */
  constructor(args: Record<string, any>, raw_args: string[])
  /** Get the parsed arguments */
  get args(): Record<string, any>
  /** Get the raw arguments */
  getRawArgs(): string[]
  /** Get the argument value by specified key */
  get(key: string): any
}
