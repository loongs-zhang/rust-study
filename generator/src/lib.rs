//! # generator
//!
//! Rust generator library
//!
mod stack;
mod register;
mod reg_context;
mod generator;
mod runtime;
mod scope;
mod co_yield;

pub use crate::generator::{Generator, Gn, LocalGenerator};
pub use crate::runtime::{get_local_data, is_generator, Error};
pub use crate::scope::Scope;
pub use crate::co_yield::{
    co_get_yield, co_set_para, co_yield_with, done,
};
