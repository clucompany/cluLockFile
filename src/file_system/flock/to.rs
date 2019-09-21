
use crate::file_system::flock::err::ErrFSFRecovery;
use crate::file_system::flock::err::ErrFSFReadFile;
use crate::file_system::flock::err::ErrFSFCreateFile;
use crate::err::SyncFileErr;
use crate::file_system::flock::element::DataOrRemoveData;
use std::fs::File;
use cluFlock::FlockLock;
use crate::file_system::flock::element::AutoRemovePath;
use crate::file_system::flock::FlockSyncFile;
use std::path::Path;

pub trait FlockSyncFileTo where Self: AsRef<Path> + Sized {
	fn flock_sync_create_file(self) -> 
		Result<
			FlockSyncFile<AutoRemovePath<FlockLock<File>, Self>>, 
			SyncFileErr<Self, ErrFSFCreateFile>
		>;
		
	fn flock_sync_open_file(self) -> 
		Result<
			FlockSyncFile<FlockLock<File>>, 
			SyncFileErr<Self, ErrFSFReadFile>
		>;
		
	fn flock_sync_recovery_file(self) -> 
		Result<
			FlockSyncFile<DataOrRemoveData<FlockLock<File>, Self>>, 
			SyncFileErr<Self, ErrFSFRecovery>
		>;
}


impl<T> FlockSyncFileTo for T where T: AsRef<Path> {
	#[inline(always)]
	fn flock_sync_create_file(self) -> Result<FlockSyncFile<AutoRemovePath<FlockLock<File>, Self>>, SyncFileErr<Self, ErrFSFCreateFile>> {
		FlockSyncFile::create_file(self)
	}
	
	#[inline(always)]
	fn flock_sync_open_file(self) -> Result<FlockSyncFile<FlockLock<File>>, SyncFileErr<Self, ErrFSFReadFile>> {
		FlockSyncFile::open_file(self)
	}
	
	#[inline(always)]
	fn flock_sync_recovery_file(self) -> Result<FlockSyncFile<DataOrRemoveData<FlockLock<File>, Self>>, SyncFileErr<Self, ErrFSFRecovery>> {
		FlockSyncFile::recovery_file(self)
	}
}

