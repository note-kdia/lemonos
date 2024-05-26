use crate::efi::EfiStatus;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Error {
    EfiError(EfiStatus),
}

impl From<EfiStatus> for Error {
    fn from(e: EfiStatus) -> Self {
        Error::EfiError(e)
    }
}
