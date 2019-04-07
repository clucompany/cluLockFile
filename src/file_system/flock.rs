
extern crate cluFlock;

use crate::error::LockFileError;
use std::io::ErrorKind::AlreadyExists;
use std::fs::File;
use crate::Locker;
use crate::check::CheckLock;
use crate::type_check::IsLock;
use std::path::Path;
use std::fs::OpenOptions;
use cluFlock::ToFlock;
use crate::IsLockType;
use std::fs;

#[derive(Debug)]
pub struct FlockLock<P> where P: AsRef<Path> {
	path: cluFlock::FlockLock<File>,
	drop: Option<DropPath<P>>,
}

impl<P> FlockLock<P> where P: AsRef<Path> {
	#[inline]
	const fn new(t: cluFlock::FlockLock<File>, p: Option<DropPath<P>>) -> Self {
		Self {
			path: t,
			drop: p,
		}
	}
	
	pub fn lock(path: P) -> Result<Self, LockFileError<PathOrDrop<P>>> {
		let (file_lock, drop_path) = match OpenOptions::new().write(true).create_new(true).open(&path) {
			Ok(file) => {
				let drop_path = DropPath::new(path);
				//Drop path, safe!
				
				let flock = match file.try_exclusive_lock() {
					Ok(a)	=> a,
					Err(e) => return Err( LockFileError::new(drop_path.into(), e.into()) ),
					//DROP FILE! this
				};
				
				
				(flock, drop_path)
			},
			Err(e) => return Err( LockFileError::new(path.into(), e) ), //FlockError::new(path, e)
		};

		Ok( Self::new(file_lock, Some(drop_path)) )
	}

	pub fn recovery(path: P) -> Result<Self, LockFileError<PathOrDrop<P>>> {
		let (lock, option_drop_path) = match OpenOptions::new().write(true).create_new(true).open(&path) {
			Ok(file) => {
				let drop_path = DropPath::new(path);
				//Drop path, safe!
				
				//WAIT METHODS!
				let flock = match file.wait_exclusive_lock() {
					Ok(a)	=> a,
					Err(e) => return Err( LockFileError::new(drop_path.into(), e.into()) ),
					//DROP FILE! this
				};
				
				( flock, Some( drop_path ) )
			},
			Err(ref e) if e.kind() == AlreadyExists => {
				let f = match OpenOptions::new().read(true).open(&path) {
					Ok(a)	=> a,
					Err(e) => return Err( LockFileError::new(path.into(), e) ),
				};
				
				let flock = match f.try_exclusive_lock() {
					Ok(a)	=> a,
					Err(e) => return Err( LockFileError::new(path.into(), e.into()) ),
					//DROP FILE! this
				};

				(flock, None)
			},
			Err(e) => return Err( LockFileError::new(path.into(), e) ),
		};
		

		Ok( Self::new(lock, option_drop_path) )
	}
	
	pub const fn is_lock(&self) -> IsLock {
		IsLock::Individually(IsLockType::AlwaysOn)
	}
	
	pub const fn check_lock(self) -> CheckLock<Self> {
		CheckLock::MoveSelf(self)
	}
}

impl<P> Locker for FlockLock<P> where P: AsRef<Path> {
	#[inline(always)]
	fn is_lock(&self) -> IsLock {
		FlockLock::is_lock(self)
	}
	#[inline(always)]
	fn check_lock(self) -> CheckLock<Self> where Self: Sized {
		FlockLock::check_lock(self)
	}
	#[inline(always)]
	fn exists(&self) -> bool {
		true
	}
}

#[derive(Debug)]
pub enum PathOrDrop<A> where A: AsRef<Path> {
	DropPath(DropPath<A>),
	Path(A),
}

impl<A> From<DropPath<A>> for PathOrDrop<A> where A: AsRef<Path> {
	#[inline(always)]
	fn from(a: DropPath<A>)	-> Self {
		PathOrDrop::DropPath(a)	
	}
}


impl<A> From<A> for PathOrDrop<A> where A: AsRef<Path> {
	#[inline(always)]
	fn from(a: A) -> Self {
		PathOrDrop::Path(a)
	}
}


impl<A> PathOrDrop<A> where A: AsRef<Path> {
	pub fn as_path(&self) -> &A {
		match self {
			PathOrDrop::DropPath(a) => a.as_ref(),
			PathOrDrop::Path(a) => a,
		}	
	}
}

impl<A> AsRef<Path> for PathOrDrop<A> where A: AsRef<Path> {
	#[inline(always)]
	fn as_ref(&self) -> &Path {
		self.as_path().as_ref()
	}
}



#[derive(Debug, Clone)]
pub struct DropPath<A> where A: AsRef<Path> {
	path: A	
}

impl<A> DropPath<A> where A: AsRef<Path> {
	#[inline]
	pub const fn new(a: A) -> Self {
		Self {
			path: a
		}	
	}
}

impl<A> AsRef<A> for DropPath<A> where A: AsRef<Path> {
	#[inline(always)]
	fn as_ref(&self) -> &A {
		&self.path
	}
}

impl<A> AsRef<Path> for DropPath<A> where A: AsRef<Path> {
	#[inline(always)]
	fn as_ref(&self) -> &Path {
		self.path.as_ref()
	}
}

impl<A> From<A> for DropPath<A> where A: AsRef<Path> {
	#[inline(always)]
	fn from(a: A) -> Self {
		Self::new(a)	
	}
}

impl<A> Drop for DropPath<A> where A: AsRef<Path> {
	#[inline(always)]
	fn drop(&mut self) {
		let _e = fs::remove_file(self.path.as_ref());	
	}
}


pub trait FlockLockTo where Self: AsRef<Path> + Sized {
	fn flock_lock(self) -> Result<FlockLock<Self>, LockFileError<PathOrDrop<Self>>>;
	fn flock_recovery_lock(self) -> Result<FlockLock<Self>, LockFileError<PathOrDrop<Self>>>;
	
}

impl<T> FlockLockTo for T where T: AsRef<Path> + Sized {
	#[inline(always)]
	fn flock_lock(self) -> Result<FlockLock<Self>, LockFileError<PathOrDrop<Self>>> {
		FlockLock::lock(self)
	}
	
	#[inline(always)]
	fn flock_recovery_lock(self) -> Result<FlockLock<Self>, LockFileError<PathOrDrop<Self>>> {
		FlockLock::recovery(self)
	}
}

