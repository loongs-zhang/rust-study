//! # generator
//!
//! Rust generator library
//!
mod detail;
mod gen_impl;
mod reg_context;
mod runtime;
mod scope;
mod stack;
mod yield_;

pub use crate::gen_impl::{Generator, Gn, LocalGenerator};
pub use crate::runtime::{get_local_data, is_generator, Error};
pub use crate::scope::Scope;
pub use crate::yield_::{
    co_get_yield, co_set_para, co_yield_with, done,
};
