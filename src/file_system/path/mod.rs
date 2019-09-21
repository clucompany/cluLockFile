
use std::path::Path;
use crate::file_system::path::err::ErrCreateFile;
use crate::err::SyncFileErr;
use crate::file_system::path::element::PathElement;
use crate::state::ActiveSyncState;
use crate::state::ToLockState;
use crate::state::MoveActiveSyncState;
use crate::SyncFile;
use std::ops::DerefMut;
use std::ops::Deref;
use std::fs::OpenOptions;
use std::fs::File;

pub mod element;

mod to;
pub use self::to::*;

pub mod err;


#[derive(Debug)]
pub struct FilePathSync<T> where T: PathElement {
	file: File,
	path: T
}

impl<T> FilePathSync<T> where T: PathElement {
	pub fn create_file(path: T) -> Result<Self, SyncFileErr<T, ErrCreateFile>> {
		let file = match OpenOptions::new().read(true).write(true).create_new(true).open(path.as_ref()) {
			Ok(file) => file,
			Err(e) => return Err( SyncFileErr::new(path, e, ErrCreateFile::CreateNewFile) ),
		};

		Ok( Self::__new(file, path) )
	}
}


impl<T> FilePathSync<T> where T: PathElement {
	pub unsafe fn recovery_sync(path: T) -> Result<Self, SyncFileErr<T, ErrCreateFile>> {
		let file = match OpenOptions::new().read(true).write(true).create_new(false).open(path.as_ref()) {
			Ok(file) => file,
			Err(e) => return Err( SyncFileErr::new(path, e, ErrCreateFile::CreateNewFile) ),
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

impl<T> AsRef<Path> for FilePathSync<T> where T: PathElement {
	#[inline(always)]
	fn as_ref(&self) -> &Path {
		self.path.as_ref()
	}
}

impl<T> AsRef<File> for FilePathSync<T> where T: PathElement {
	#[inline(always)]
	fn as_ref(&self) -> &File {
		&self.file
	}
}


impl<T> Deref for FilePathSync<T> where T: PathElement {
	type Target = File;
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target {
		&self.file
	}
}

impl<T> DerefMut for FilePathSync<T> where T: PathElement {
	#[inline(always)]
	fn deref_mut(&mut self)	-> &mut Self::Target {
		&mut self.file
	}
}

impl<T> SyncFile for FilePathSync<T> where T: PathElement {
	#[inline(always)]
	fn is_sync(&self) -> bool {
		self.path.path_exists()
	}
}


impl<T> ToLockState for FilePathSync<T> where T: PathElement {
	fn move_sync_state(self) -> MoveActiveSyncState<Self> where Self: Sized {
		match self.is_sync() {
			true => MoveActiveSyncState::On(self),
			_ => MoveActiveSyncState::Off,
		}
	}
	
	#[inline]
	fn sync_state(&self) -> ActiveSyncState {
		self.is_sync().into()
	}
}


impl<T> Drop for FilePathSync<T> where T: PathElement {
	fn drop(&mut self) {
		let _e = self.path.remove_file();
	}
}
