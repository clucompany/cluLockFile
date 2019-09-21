
use crate::file_system::flock::err::ErrFSFRecovery;
use crate::file_system::flock::err::ErrFSFReadFile;
use crate::err::SyncFileErr;
use crate::file_system::flock::err::ErrFSFCreateFile;
use std::io::ErrorKind::AlreadyExists;
use crate::file_system::flock::element::DataOrRemoveData;
use crate::file_system::flock::element::FSFElement;
use crate::file_system::flock::element::AutoRemovePath;
use crate::state::ActiveSyncState;
use crate::state::MoveActiveSyncState;
use crate::state::ToLockState;
use crate::SyncFile;
use std::path::Path;
use std::fs::File;
use std::fs::OpenOptions;
use cluFlock::ToFlock;
use cluFlock::FlockLock;
use crate::file_system::flock::element::PrevAutoRemovePath;
use cluFlock::err::FlockError;

pub mod element;
pub mod err;

mod to;
pub use self::to::*;


#[derive(Debug)]
pub struct FlockSyncFile<D> where D: FSFElement {
	data: D
}

impl<D> FlockSyncFile<D> where D: FSFElement {
	#[inline]
	const fn __new(data: D) -> Self {
		Self {
			data: data,
		}
	}
}

impl<P> FlockSyncFile<AutoRemovePath<FlockLock<File>, P>> where P: AsRef<Path> {
	pub fn create_file(path: P) -> Result<Self, SyncFileErr<P, ErrFSFCreateFile>> {
		let file = match OpenOptions::new().write(true).create_new(true).read(true).open(&path) {
			Ok(file)	=> file,
			Err(e)	=> return Err( SyncFileErr::new(path, e, ErrFSFCreateFile::CreateFile) )
		};
		let prev_auto_remove = PrevAutoRemovePath::new(path);
		
		let flock = match file.try_exclusive_lock() {
			Ok(a)	=> a,
			Err(e)	=> return Err( SyncFileErr::new(prev_auto_remove.into_remove_file().0, e.err(), ErrFSFCreateFile::TryExclusiveFlock) ),
		};
		
		Ok( Self::__new(prev_auto_remove.to_auto_remove_path(flock)) )
	}
}

impl FlockSyncFile<FlockLock<File>> {
	pub fn open_file<P>(path: P) -> Result<Self, SyncFileErr<P, ErrFSFReadFile>> where P: AsRef<Path> {
		let file = match OpenOptions::new().write(false).create_new(false).read(true).open(&path) {
			Ok(file)	=> file,
			Err(e)	=> return Err( SyncFileErr::new(path, e, ErrFSFReadFile::CreateFile) )
		};
		
		let flock = match file.try_exclusive_lock() {
			Ok(a)	=> a,
			Err(e)	=> return Err( SyncFileErr::new(path, e.err(), ErrFSFReadFile::TryExclusiveFlock) ),
		};
		
		Ok( Self::__new(flock) )
	}
}

impl<P> FlockSyncFile<DataOrRemoveData<FlockLock<File>, P>> where P: AsRef<Path> {
	pub fn recovery_file(path: P) -> Result<Self, SyncFileErr<P, ErrFSFRecovery>> {
		
		match OpenOptions::new().write(true).create_new(true).open(&path) {
			Ok(file) => {
				let prev_auto_remove = PrevAutoRemovePath::new(path);
				
				let flock = match file.wait_exclusive_lock() {
					Ok(a)	=> a,
					Err(e)	=> return Err( SyncFileErr::new(prev_auto_remove.into_remove_file().0, e.err(), ErrFSFRecovery::WaitExFlockFile) ),
					//DROP FILE! this
				};
				
				Ok( Self::__new(DataOrRemoveData::remove_file(
					prev_auto_remove.to_auto_remove_path(flock)
				)) )
			},
			Err(ref e) if e.kind() == AlreadyExists => {
				let file = match OpenOptions::new().read(true).open(&path) {
					Ok(a)	=> a,
					Err(e)	=> return Err( SyncFileErr::new(path, e, ErrFSFRecovery::OpenFile) ),
				};
				
				let flock = match file.try_exclusive_lock() {
					Ok(a)	=> a,
					Err(e)	=> return Err( SyncFileErr::new(path, e.err(), ErrFSFRecovery::TryExFlockFile) ),
					//DROP FILE! this
				};

				Ok( Self::__new(DataOrRemoveData::data(
					flock
				)) )
			},
			
			Err(e) => Err( SyncFileErr::new(path, e, ErrFSFRecovery::CreateFile) )
		}
	}
}

impl FlockSyncFile<FlockLock<File>> {
	pub unsafe fn raw_new(file: File) -> Result<Self, FlockError<File>> {
		match file.try_exclusive_lock() {
			Ok(a) => Ok(Self::__new(a)),
			Err(e) => Err(e),
		}
	}
}

impl<P> SyncFile for FlockSyncFile<P> where P: FSFElement {
	#[inline(always)]
	fn is_sync(&self) -> bool {
		true
	}
}

impl<P> ToLockState for FlockSyncFile<P> where P: FSFElement {
	#[inline]
	fn move_sync_state(self) -> MoveActiveSyncState<Self> where Self: Sized {
		MoveActiveSyncState::always_on(self)
	}
	
	#[inline]
	fn sync_state(&self) -> ActiveSyncState {
		ActiveSyncState::AlwaysOn
	}
}
