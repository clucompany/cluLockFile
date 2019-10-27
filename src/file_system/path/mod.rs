
use std::path::Path;
use crate::file_system::path::err::CreateFileErr;
use crate::err::LockFileErr;
use crate::file_system::path::element::PathElement;
use crate::state::ActiveLockState;
use crate::state::MoveActiveLockState;
use crate::LockFile;
use std::ops::DerefMut;
use std::ops::Deref;
use std::fs::OpenOptions;
use std::fs::File;

pub mod element;

mod to;
pub use self::to::*;

pub mod err;


#[derive(Debug)]
pub struct FilePathLock<T> where T: PathElement {
	file: File,
	path: T
}

impl<T> FilePathLock<T> where T: PathElement {
	pub fn create_file(path: T) -> Result<Self, LockFileErr<T, CreateFileErr>> {
		let file = match OpenOptions::new().read(true).write(true).create_new(true).open(path.as_ref()) {
			Ok(file) => file,
			Err(e) => return Err( LockFileErr::new(path, e, CreateFileErr::NewFile) ),
		};

		Ok( Self::__new(file, path) )
	}
}


impl<T> FilePathLock<T> where T: PathElement {
	pub unsafe fn recovery_sync(path: T) -> Result<Self, LockFileErr<T, CreateFileErr>> {
		let file = match OpenOptions::new().read(true).write(true).create_new(false).open(path.as_ref()) {
			Ok(file) => file,
			Err(e) => return Err( LockFileErr::new(path, e, CreateFileErr::NewFile) ),
		};
		
		Ok( Self::__new(file, path) )
	}
	
	#[inline(always)]
	pub const unsafe fn new(file: File, path: T) -> Self {
		Self::__new(file, path)
	}
	
	#[inline]
	const fn __new(file: File, path: T) -> Self {
		Self {
			file: file,
			path: path
		}
	}
}

impl<T> AsRef<Path> for FilePathLock<T> where T: PathElement {
	#[inline(always)]
	fn as_ref(&self) -> &Path {
		self.path.as_ref()
	}
}

impl<T> AsRef<File> for FilePathLock<T> where T: PathElement {
	#[inline(always)]
	fn as_ref(&self) -> &File {
		&self.file
	}
}


impl<T> Deref for FilePathLock<T> where T: PathElement {
	type Target = File;
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target {
		&self.file
	}
}

impl<T> DerefMut for FilePathLock<T> where T: PathElement {
	#[inline(always)]
	fn deref_mut(&mut self)	-> &mut Self::Target {
		&mut self.file
	}
}

impl<T> LockFile for FilePathLock<T> where T: PathElement {
	#[inline]
	fn is_active_lock(&self) -> bool {
		self.path.path_exists()
	}
	
	#[inline]
	fn move_active_lock_state(self) -> MoveActiveLockState<Self> where Self: Sized {
		match self.is_active_lock() {
			true => MoveActiveLockState::On(self),
			_ => MoveActiveLockState::Off,
		}
	}
	
	#[inline]
	fn active_lock_state(&self) -> ActiveLockState {
		ActiveLockState::state(self.is_active_lock())
	}
}


impl<T> Drop for FilePathLock<T> where T: PathElement {
	fn drop(&mut self) {
		let _e = self.path.remove_file();
	}
}
