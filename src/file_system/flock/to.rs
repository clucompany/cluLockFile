
use crate::file_system::flock::element::DontAutoRemovePath;
use crate::file_system::flock::err::FlRecoveryErr;
use crate::file_system::flock::err::FlReadFileErr;
use crate::file_system::flock::err::FlCreateFileErr;
use crate::err::LockFileErr;
use crate::file_system::flock::element::MaybeAutoRemovePath;
use std::fs::File;
use cluFlock::FlockLock;
use crate::file_system::flock::element::AutoRemovePath;
use crate::file_system::flock::FlLockFile;
use std::path::Path;

pub trait ToFlLockFile where Self: AsRef<Path> + Sized {
	fn fl_create_file(self) -> 
		Result<
			FlLockFile<AutoRemovePath<FlockLock<File>, Self>>, 
			LockFileErr<Self, FlCreateFileErr>
		>;
		
	fn fl_open_file(self) -> 
		Result<
			FlLockFile<DontAutoRemovePath<FlockLock<File>>>, 
			LockFileErr<Self, FlReadFileErr>
		>;
		
	fn fl_recovery_file(self) -> 
		Result<
			FlLockFile<MaybeAutoRemovePath<FlockLock<File>, Self>>, 
			LockFileErr<Self, FlRecoveryErr>
		>;
}


impl<T> ToFlLockFile for T where T: AsRef<Path> {
	#[inline(always)]
	fn fl_create_file(self) -> Result<FlLockFile<AutoRemovePath<FlockLock<File>, Self>>, LockFileErr<Self, FlCreateFileErr>> {
		FlLockFile::create_file(self)
	}
	
	#[inline(always)]
	fn fl_open_file(self) -> Result<FlLockFile<DontAutoRemovePath<FlockLock<File>>>, LockFileErr<Self, FlReadFileErr>> {
		FlLockFile::open_file(self)
	}
	
	#[inline(always)]
	fn fl_recovery_file(self) -> Result<FlLockFile<MaybeAutoRemovePath<FlockLock<File>, Self>>, LockFileErr<Self, FlRecoveryErr>> {
		FlLockFile::recovery_file(self)
	}
}

