#![allow(dead_code)]
#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]

pub mod cli;
pub mod commit;
pub mod config;
pub mod error;
pub mod git;
pub mod lint;
pub mod rules;
