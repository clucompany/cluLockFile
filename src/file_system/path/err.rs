
use crate::err::SyncFileErrType;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum ErrCreateFile {
	CreateNewFile,
}

impl SyncFileErrType for ErrCreateFile {}

