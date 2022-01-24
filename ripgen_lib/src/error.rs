use thiserror::Error;

#[derive(Error, Debug)]
/// The RipGen error type.
pub enum RipGenError {
    #[error("Unable to parse provided domain.")]
    ErrorParsingDomain(String),
    #[error("Provided domain does not have a root domain.")]
    DomainMissingRoot,
}