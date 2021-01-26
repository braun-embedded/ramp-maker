//! RampMaker - Stepper Acceleration Ramp Generator
//!
//! RampMaker is a library that generates acceleration profiles for stepper
//! motors. It can be used independently, or together with [Step/Dir].
//!
//! [Step/Dir]: https://crates.io/crates/step-dir

#![cfg_attr(all(not(test), not(feature = "std")), no_std)]
#![deny(missing_docs, broken_intra_doc_links)]

mod flat;
mod trapezoidal;

pub use self::{flat::Flat, trapezoidal::Trapezoidal};
