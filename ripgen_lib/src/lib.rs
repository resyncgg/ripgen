#![deny(warnings)]

pub use chain::{
    RipGenChain,
    RipGenIterator
};
pub use domain::DomainComponents;
pub use error::RipGenError;
pub use manager::RipGenManager;

mod manager;
mod domain;
mod error;
mod words;
mod chain;
pub(crate) mod transform;

#[cfg(feature = "dnsgen")]
pub mod dnsgen;

/// Placeholder for a HashSet iterator with annoying lifetimes
pub type WordlistIterator<'domain> = std::collections::hash_set::Iter<'domain, &'domain str>;