use std::fmt;

#[derive(Debug, Clone)]
pub struct InvalidDocumentError;

impl fmt::Display for InvalidDocumentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid document")
    }
}