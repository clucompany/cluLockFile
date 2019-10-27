
use std::fmt::Debug;
use std::ops::DerefMut;
use std::ops::Deref;
use std::io;

pub trait LockFileErrEnum where Self: Debug + Clone + PartialEq + PartialOrd {}

#[derive(Debug)]
pub struct LockFileErr<T, ET> where ET: LockFileErrEnum {
	data: T,
	err_type: ET,
	err: io::Error,
}

impl<T, ET> LockFileErr<T, ET> where ET: LockFileErrEnum {
	#[inline]
	pub const fn new(a: T, err: io::Error, err_type: ET) -> Self {
		Self {
			data: a,
			err_type: err_type,
			err: err,
		}
	}
	
	#[inline(always)]
	pub fn into(self) -> T {
		self.data
	}
	
	#[inline(always)]
	pub fn r#type(self) -> ET {
		self.err_type
	}
	
	#[inline(always)]
	pub fn err(self) -> io::Error {
		self.err
	}
	
	#[inline(always)]
	pub fn all(self) -> (T, ET, io::Error) {
		(self.data, self.err_type, self.err)
	}
}

impl<T, ET> From<LockFileErr<T, ET>> for io::Error where ET: LockFileErrEnum {
	#[inline(always)]
	fn from(a: LockFileErr<T, ET>) -> io::Error {
		a.err
	}
}

impl<T, ET> Deref for LockFileErr<T, ET> where ET: LockFileErrEnum {
	type Target = io::Error;
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target {
		&self.err
	}
}

impl<T, ET> DerefMut for LockFileErr<T, ET> where ET: LockFileErrEnum {
	#[inline(always)]
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.err
	}
}
