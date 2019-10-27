
use crate::file_system::flock::element::DontAutoRemovePath;
use crate::file_system::flock::err::FlRecoveryErr;
use crate::file_system::flock::err::FlReadFileErr;
use crate::err::LockFileErr;
use crate::file_system::flock::err::FlCreateFileErr;
use std::io::ErrorKind::AlreadyExists;
use crate::file_system::flock::element::MaybeAutoRemovePath;
use crate::file_system::flock::element::FlElement;
use crate::file_system::flock::element::AutoRemovePath;
use crate::state::ActiveLockState;
use crate::state::MoveActiveLockState;
use crate::LockFile;
use std::path::Path;
use std::fs::File;
use std::fs::OpenOptions;
use cluFlock::ToFlock;
use cluFlock::FlockLock;
use cluFlock::err::FlockError;

pub mod element;
pub mod err;

mod to;
pub use self::to::*;


#[derive(Debug)]
pub struct FlLockFile<D> where D: FlElement {
	data: D
}

impl<D> FlLockFile<D> where D: FlElement {
	#[inline]
	const fn __new(data: D) -> Self {
		Self {
			data: data,
		}
	}
	
	pub const unsafe fn raw(data: D) -> Self {
		Self::__new(data)
	}
}

impl<P> FlLockFile<AutoRemovePath<FlockLock<File>, P>> where P: AsRef<Path> {
	pub fn create_file(path: P) -> Result<Self, LockFileErr<P, FlCreateFileErr>> {
		let file = match OpenOptions::new().write(true).create_new(true).read(true).open(&path) {
			Ok(file) => file,
			Err(e) => return Err( LockFileErr::new(path, e, FlCreateFileErr::CreateFile) )
		};
		
		let flock = match file.try_exclusive_lock() {
			Ok(a)	=> a,
			Err(e) => {
				let e = e.err(); //close file descriptor!
				
				if let Err(e) = std::fs::remove_file(&path) {
					return Err(
						LockFileErr::new(path, e, FlCreateFileErr::RemovePathAndTryExclusiveFlock)
					)
				}
				
				return Err(LockFileErr::new(path, e, FlCreateFileErr::TryExclusiveFlock) )
			},
		};
		
		Ok( Self::__new(AutoRemovePath::new(flock, path)) )
	}
}

impl FlLockFile<DontAutoRemovePath<FlockLock<File>>> {
	pub fn open_file<P>(path: P) -> Result<Self, LockFileErr<P, FlReadFileErr>> where P: AsRef<Path> {
		let file = match OpenOptions::new().write(false).create_new(false).read(true).open(&path) {
			Ok(file) => file,
			Err(e) => return Err( LockFileErr::new(path, e, FlReadFileErr::CreateFile) )
		};
		
		let flock = match file.try_exclusive_lock() {
			Ok(a)	=> a,
			Err(e) => {
				let e = e.err(); //close file descriptor
				return Err( LockFileErr::new(path, e, FlReadFileErr::TryExclusiveFlock) )
			},
		};
		
		Ok( Self::__new(DontAutoRemovePath::new(flock)) )
	}
	
	#[inline]
	pub unsafe fn raw_new(file: File) -> Result<Self, FlockError<File>> {
		match file.try_exclusive_lock() {
			Ok(a) => Ok(Self::__new(DontAutoRemovePath::new(a))),
			Err(e) => Err(e),
		}
	}
}

impl<P> FlLockFile<MaybeAutoRemovePath<FlockLock<File>, P>> where P: AsRef<Path> {
	pub fn recovery_file(path: P) -> Result<Self, LockFileErr<P, FlRecoveryErr>> {
		
		match OpenOptions::new().write(true).create_new(true).read(true).open(&path) {
			Ok(file) => {				
				let flock = match file.wait_exclusive_lock() {
					Ok(a)	=> a,
					Err(e) => {
						let e = e.err(); //close file descriptor
						
						if let Err(e) = std::fs::remove_file(&path) {
							return Err(
								LockFileErr::new(path, e, FlRecoveryErr::WaitExFlockFileAndRemovePath)
							);
						}
						
						return Err( LockFileErr::new(path, e, FlRecoveryErr::WaitExFlockFile) )
					},
				};
				
				Ok(Self::__new(MaybeAutoRemovePath::remove_path(flock, path)))
			},
			Err(ref e) if e.kind() == AlreadyExists => {
				let file = match OpenOptions::new().read(true).open(&path) {
					Ok(a)	=> a,
					Err(e) => return Err( LockFileErr::new(path, e, FlRecoveryErr::OpenFile) ),
				};
				
				let flock = match file.try_exclusive_lock() {
					Ok(a)	=> a,
					Err(e) => {
						let e = e.err();
						return Err( LockFileErr::new(path, e, FlRecoveryErr::TryExFlockFile) )
					},
					//DROP FILE! this
				};

				Ok(Self::__new(MaybeAutoRemovePath::dont_remove_path(flock)))
			},
			
			Err(e) => Err( LockFileErr::new(path, e, FlRecoveryErr::CreateFile) )
		}
	}
}

impl<P> LockFile for FlLockFile<P> where P: FlElement {
	#[inline]
	fn is_active_lock(&self) -> bool {
		true
	}
	
	#[inline]
	fn move_active_lock_state(self) -> MoveActiveLockState<Self> where Self: Sized {
		MoveActiveLockState::always_on(self)
	}
	
	#[inline]
	fn active_lock_state(&self) -> ActiveLockState {
		ActiveLockState::AlwaysOn
	}
}
