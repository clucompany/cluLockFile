
use cluFlock::FlockError;
use std::ops::Deref;
use std::io;

#[derive(Debug)]
pub struct LockFileError<T> {
	value: T,
	error: io::Error,
}

impl<T> LockFileError<T> {
	pub const fn new(a: T, err: io::Error) -> Self {
		Self {
			value: a,
			error: err,
		}	
	}
	
	#[inline(always)]
	pub fn value(self) -> T {
		self.value
	}
	
	#[inline(always)]
	pub fn error(self) -> io::Error {
		self.error	
	}
	
	#[inline(always)]
	pub fn all(self) -> (T, io::Error) {
		(self.value, self.error)
	}
}

impl<T> From<LockFileError<T>> for io::Error {
	#[inline(always)]
	fn from(a: LockFileError<T>) -> io::Error {
		a.error
	}
}

impl<T> From<FlockError<T>> for LockFileError<T> {
	#[inline(always)]
	fn from(a: FlockError<T>) -> LockFileError<T> {
		From::from(a.all())
	}
}

impl<T> Deref for LockFileError<T> {
	type Target = io::Error;	
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target {
		&self.error	
	}
}

impl<T> From<(T, io::Error)> for LockFileError<T> {
	#[inline(always)]
	fn from((t, err): (T, io::Error)) -> Self {
		LockFileError::new(t, err)	
	}
}