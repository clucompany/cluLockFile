
use crate::err::SyncFileErrType;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum ErrFSFCreateFile {
	CreateFile,
	TryExclusiveFlock,
}
impl SyncFileErrType for ErrFSFCreateFile {}


#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum ErrFSFReadFile {
	CreateFile,
	TryExclusiveFlock,
}
impl SyncFileErrType for ErrFSFReadFile {}


#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum ErrFSFRecovery {
	OpenFile,
	CreateFile,
	TryExFlockFile,
	WaitExFlockFile,
}
impl SyncFileErrType for ErrFSFRecovery {}
