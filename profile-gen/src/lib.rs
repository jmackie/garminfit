#![feature(slice_patterns)]
#![feature(stmt_expr_attributes)]
#![feature(custom_attribute)]
#![allow(dead_code)]
#![allow(unused_attributes)]
#![recursion_limit = "128"]

extern crate calamine;
extern crate failure;
extern crate inflector;
extern crate itertools;
#[macro_use]
extern crate lazy_static;
extern crate proc_macro2;
#[macro_use]
extern crate quote;

pub mod base;
pub mod error;
pub mod util;
pub mod worksheet;

pub use calamine::{
    open_workbook,
    Xlsx,
};
pub use proc_macro2::TokenStream;
