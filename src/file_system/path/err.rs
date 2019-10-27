
use crate::err::LockFileErrEnum;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum CreateFileErr {
	NewFile,
}

impl LockFileErrEnum for CreateFileErr {}

