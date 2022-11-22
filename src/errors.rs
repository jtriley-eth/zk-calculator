use std::fmt;

use halo2_proofs::{dev::VerifyFailure, plonk::Error};

/// Parser Errors.
pub enum ParserError {
    /// Thrown when an invalid operator is provided.
    InvalidOperator,
    /// Thrown when an invalid numeric operand is provided.
    InvalidOperand,
    /// Thrown when too many whitespace-separated inputs are provided.
    TooManyInputs,
    /// Thrown when not enough whitespace-separated inputs are provided.
    NotEnoughInputs,
}

/// Debug implementation for Parser Error.
impl fmt::Debug for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParserError::InvalidOperator => write!(
                f,
                "invalid operator. valid operators include `+`, `-`, and `*`."
            ),
            ParserError::InvalidOperand => write!(f, "invalid operand, operand must be numeric"),
            ParserError::TooManyInputs => {
                write!(f, "too many inputs, valid format is `a operator b`")
            }
            ParserError::NotEnoughInputs => {
                write!(f, "not enough inputs, valid format is `a operator b`")
            }
        }
    }
}

/// General Circuit Errors.
pub enum CircuitError {
    /// Thrown when `MockProver::run` fails to prove the circuit.
    ProverError(Error),
    /// Thrown when verification fails.
    VerifierError(Vec<VerifyFailure>),
    /// Thrown when no operation has been specified.
    /// This should never happen.
    NoOperation,
}

impl fmt::Debug for CircuitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CircuitError::ProverError(prover_error) => {
                write!(f, "prover error in circuit: {}", prover_error)
            }
            CircuitError::VerifierError(verifier_error) => {
                write!(f, "verifier error in circuit: {:#?}", verifier_error)
            }
            CircuitError::NoOperation => {
                write!(f, "no operation is set (this should never happen.")
            }
        }
    }
}
