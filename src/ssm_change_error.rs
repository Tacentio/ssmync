use std::error::Error;
use std::fmt;
/// Describes the kinds of errors that can occur
/// when attempting to create an SSMChange from the
/// `Result` of an ssm:GetParameter call.
/// Used in SSMChange::calculate_change().
#[derive(Debug)]
pub enum CalculateSSMChangeErrorKind {
    NoParameterInResponse,
    NoValueInParameter,
    UnexpectedError,
}

impl CalculateSSMChangeErrorKind {
    /// Described how each kind of error should be displayed as a string.
    fn as_str(&self) -> &'static str {
        use CalculateSSMChangeErrorKind::*;
        match *self {
            NoParameterInResponse => "no parameter in response",
            NoValueInParameter => "no value in parameter",
            UnexpectedError => "unexpected error",
        }
    }
}

impl fmt::Display for CalculateSSMChangeErrorKind {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_str(self.as_str())
    }
}

#[derive(Debug)]
pub struct SSMChangeError {
    pub kind: CalculateSSMChangeErrorKind,
}

impl SSMChangeError {
    pub fn new(kind: CalculateSSMChangeErrorKind) -> SSMChangeError {
        SSMChangeError { kind }
    }
}

impl fmt::Display for SSMChangeError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_str(self.kind.as_str())
    }
}

impl Error for SSMChangeError {}
