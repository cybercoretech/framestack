//! The adapters compiled into this build of `framestack`. ADR-0002 rules out
//! loading third-party adapter code until a trust model exists.

use framestack_core::Adapter;
use framestack_leptos_axum::LeptosAxum;

pub fn all() -> Vec<Box<dyn Adapter>> {
    vec![Box::new(LeptosAxum)]
}
