#![feature(nll, universal_impl_trait)]
#![allow(unused)]

extern crate byteorder;
extern crate encoding;
#[macro_use] extern crate failure;
extern crate serde;

#[macro_use] extern crate failure_derive;
#[macro_use] extern crate serde_derive;

pub mod lcf_reader;
pub mod rgb;
pub mod traits;