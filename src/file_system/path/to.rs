
use crate::file_system::path::err::ErrCreateFile;
use crate::file_system::path::FilePathSync;
use crate::file_system::path::element::PathElement;
use crate::err::SyncFileErr;

pub trait PathLockTo where Self: PathElement + Sized {
	fn file_path_lock(self) -> Result<FilePathSync<Self>, SyncFileErr<Self, ErrCreateFile>>;
}

impl<T> PathLockTo for T where T: PathElement + Sized {
	#[inline(always)]
	fn file_path_lock(self) -> Result<FilePathSync<T>, SyncFileErr<T, ErrCreateFile>> {
		FilePathSync::create_file(self)
	}
}

