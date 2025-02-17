#![deny(clippy::all, clippy::pedantic, clippy::restriction)]
#![allow(
  clippy::comparison_chain,
  clippy::else_if_without_else,
  clippy::enum_glob_use,
  clippy::expect_used,
  clippy::filter_map,
  clippy::if_not_else,
  clippy::implicit_return,
  clippy::indexing_slicing,
  clippy::integer_arithmetic,
  clippy::let_underscore_must_use,
  clippy::map_unwrap_or,
  clippy::match_same_arms,
  clippy::missing_docs_in_private_items,
  clippy::missing_errors_doc,
  clippy::missing_inline_in_public_items,
  clippy::needless_pass_by_value,
  clippy::non_ascii_literal,
  clippy::panic,
  clippy::print_stdout,
  clippy::shadow_unrelated,
  clippy::string_add,
  clippy::struct_excessive_bools,
  clippy::too_many_lines,
  clippy::unreachable,
  clippy::unwrap_used,
  clippy::use_debug,
  clippy::wildcard_enum_match_arm,
  clippy::wildcard_imports
)]

#[macro_use]
extern crate lazy_static;

#[cfg(test)]
#[macro_use]
pub mod testing;

#[cfg(test)]
#[macro_use]
pub mod tree;

#[cfg(test)]
pub mod node;

#[cfg(fuzzing)]
pub(crate) mod fuzzing;

mod alias;
mod analyzer;
mod assignment;
mod assignment_resolver;
mod binding;
mod color;
mod command_ext;
mod common;
mod compilation_error;
mod compilation_error_kind;
mod compilation_result_ext;
mod compiler;
mod config;
mod config_error;
mod count;
mod default;
mod dependency;
mod empty;
mod enclosure;
mod error;
mod error_result_ext;
mod evaluator;
mod expression;
mod fragment;
mod function;
mod function_context;
mod interrupt_guard;
mod interrupt_handler;
mod item;
mod justfile;
mod keyed;
mod keyword;
mod lexer;
mod line;
mod list;
mod load_dotenv;
mod load_error;
mod module;
mod name;
mod ordinal;
mod output;
mod output_error;
mod parameter;
mod parameter_kind;
mod parser;
mod platform;
mod platform_interface;
mod position;
mod positional;
mod range_ext;
mod recipe;
mod recipe_context;
mod recipe_resolver;
mod run;
mod runtime_error;
mod scope;
mod search;
mod search_config;
mod search_error;
mod set;
mod setting;
mod settings;
mod shebang;
mod show_whitespace;
mod string_literal;
mod subcommand;
mod suggestion;
mod table;
mod thunk;
mod token;
mod token_kind;
mod unresolved_dependency;
mod unresolved_recipe;
mod use_color;
mod variables;
mod verbosity;
mod warning;

pub use crate::run::run;

#[cfg(feature = "summary")]
pub mod summary;
