
use crate::err::LockFileErrEnum;

//
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum FlCreateFileErr {
	CreateFile,
	TryExclusiveFlock,
	RemovePath,
	
	RemovePathAndTryExclusiveFlock,
}
impl LockFileErrEnum for FlCreateFileErr {}


//
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum FlReadFileErr {
	CreateFile,
	TryExclusiveFlock,
}
impl LockFileErrEnum for FlReadFileErr {}


//
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum FlRecoveryErr {
	OpenFile,
	CreateFile,
	TryExFlockFile,
	WaitExFlockFile,
	
	RemovePath,
	WaitExFlockFileAndRemovePath,
}
impl LockFileErrEnum for FlRecoveryErr {}
