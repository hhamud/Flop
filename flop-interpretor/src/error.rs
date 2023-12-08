use thiserror::Error;

#[derive(Debug, PartialEq, Error)]
pub enum EvalError {
    /// Operation and comparison error
    OperationError(ExpectedError),
}
