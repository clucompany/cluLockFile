
use std::ops::DerefMut;
use std::ops::Deref;
use crate::Locker;
use crate::check::CheckLock;
use crate::type_check::IsLock;
use crate::error::LockFileError;
use std::path::Path;
use std::path::PathBuf;
use std::fs::OpenOptions;
use std::fs;
use std::fs::File;

pub trait PathElement: AsRef<Path> {
	fn exists(&self) -> bool;
}

impl PathElement for PathBuf {
	#[inline(always)]
	fn exists(&self) -> bool {
		(**self).exists()
	}
}

impl<'a> PathElement for &'a Path {
	#[inline(always)]
	fn exists(&self) -> bool {
		(**self).exists()
	}
}

impl<'a, A> PathElement for &'a A where A: AsRef<Path> {
	#[inline(always)]
	fn exists(&self) -> bool {
		self.as_ref().exists()	
	}
}


mod unlock {
	use std::path::Path;
	use std::fs;
	use std::io;

	pub trait PathUnlock: Sized {
		fn unlock(&mut self) -> Result<(), io::Error>;
	}

	impl<A> PathUnlock for A where A: AsRef<Path> + Sized {
		#[inline(always)]
		fn unlock(&mut self) -> Result<(), io::Error> {
			fs::remove_file(self)
		}
	}
}


#[derive(Debug)]
pub struct PathLock<T> where T: PathElement {
	file: File,
	path: T	
}

impl<T> PathLock<T> where T: PathElement {
	#[inline]
	pub const fn new(f: File, a: T) -> Self {
		Self {
			file: f,
			path: a
		}
	}
	
	pub fn lock(path: T) -> Result<Self, LockFileError<T>> {
		let file = match OpenOptions::new().write(true).create_new(true).open(&path) {
			Ok(file) => file,
			//Err(ref e) if e.kind() == AlreadyExists => return Err(Error::new(ErrorKind::Other, "the file is already locked")),
			Err(e) => return Err( LockFileError::new(path, e) ),
		};

		Ok( Self::new(file, path) )
	}
	
	pub fn is_lock(&self) -> IsLock {
		self.path.exists().into()
	}
	
	pub fn check_lock(self) -> CheckLock<Self> {
		From::from({
			match self.path.exists() {
				true => Some(self),
				_ => None,
			}
		})
	}
}

impl<T> AsRef<Path> for PathLock<T> where T: PathElement {
	#[inline(always)]
	fn as_ref(&self) -> &Path {
		self.path.as_ref()
	}
}

impl<T> Deref for PathLock<T> where T: PathElement {
	type Target = File;
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target {
		&self.file	
	}
}

impl<T> DerefMut for PathLock<T> where T: PathElement {
	#[inline(always)]
	fn deref_mut(&mut self)	-> &mut Self::Target {
		&mut self.file	
	}
}

impl<T> Locker for PathLock<T> where T: PathElement {
	#[inline(always)]
	fn is_lock(&self) -> IsLock {
		PathLock::is_lock(self)
	}
	#[inline(always)]
	fn check_lock(self) -> CheckLock<Self> where Self: Sized {
		PathLock::check_lock(self)
	}
	#[inline(always)]
	fn exists(&self) -> bool {
		self.path.exists()
	}
}

impl<T> From<(File, T)> for PathLock<T> where T: PathElement {
	#[inline(always)]
	fn from((file, path): (File, T)) -> Self {
		Self::new(file, path)
	}
}

impl<T> Drop for PathLock<T> where T: PathElement {
	fn drop(&mut self) {
		let _e = fs::remove_file(&self.path);
	}
}


pub trait PathLockTo where Self: PathElement + Sized {
	fn path_lock(self) -> Result<PathLock<Self>, LockFileError<Self>>;
}

impl<T> PathLockTo for T where T: PathElement + Sized {
	#[inline(always)]
	fn path_lock(self) -> Result<PathLock<T>, LockFileError<T>> {
		PathLock::lock(self)
	}
}

