//! `rust-util` is a collection of utilities that I use across several programs, or just stuff that
//! I write and want to keep in a library.
//!
//! That's all for now.
//! The lib.rs file just re-exports everything

#![feature(test)]

extern crate num;
extern crate test;

//mod primes;
mod min_heap;
mod num_util;
pub mod primestwo;
