
use crate::file_system::path::err::CreateFileErr;
use crate::file_system::path::FilePathLock;
use crate::file_system::path::element::PathElement;
use crate::err::LockFileErr;

pub trait ToPathLock where Self: PathElement + Sized {
	fn file_path_lock(self) -> Result<FilePathLock<Self>, LockFileErr<Self, CreateFileErr>>;
}

impl<T> ToPathLock for T where T: PathElement + Sized {
	#[inline(always)]
	fn file_path_lock(self) -> Result<FilePathLock<T>, LockFileErr<T, CreateFileErr>> {
		FilePathLock::create_file(self)
	}
}

