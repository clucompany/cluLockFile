
use std::ops::Deref;
use std::ops::DerefMut;
use std::path::Path;

pub trait FlElement: Deref<Target = <Self as FlElement>::Target> + DerefMut {
	type Target;
	
	fn is_auto_remove_path(&self) -> bool;
}

#[derive(Debug)]
pub struct DontAutoRemovePath<D> {
	data: D
}
impl<D> FlElement for DontAutoRemovePath<D> {
	type Target = D;
	
	#[inline]
	fn is_auto_remove_path(&self) -> bool {
		false
	}
}

impl<D> DontAutoRemovePath<D> {
	#[inline]
	pub const fn new(d: D) -> Self {
		Self {
			data: d	
		}
	}
}

impl<D> Deref for DontAutoRemovePath<D> {
	type Target = <Self as FlElement>::Target;
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target {
		&self.data
	}
}

impl<D> DerefMut for DontAutoRemovePath<D> {
	#[inline(always)]
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.data
	}
}



#[derive(Debug)]
pub struct MaybeAutoRemovePath<D, P> where P: AsRef<Path> {
	data: D,
	is_remove_path: Option<P>,
}
impl<D, P> FlElement for MaybeAutoRemovePath<D, P> where P: AsRef<Path> {
	type Target = D;
	
	#[inline]
	fn is_auto_remove_path(&self) -> bool {
		match self.is_remove_path {
			Some(_) => true,
			_ => false,
		}
	}
}


impl<D, P> MaybeAutoRemovePath<D, P> where P: AsRef<Path> {
	#[inline]
	const fn __new(data: D, is_remove_path: Option<P>) -> Self {
		Self {
			data: data,
			is_remove_path: is_remove_path,
		}
	}
	
	pub fn remove_path(data: D, path: P) -> Self {
		Self::__new(data, Some(path))
	}
	
	pub fn dont_remove_path(data: D) -> Self {
		Self::__new(data, None)
	}
	
	
}

impl<D, P> Deref for MaybeAutoRemovePath<D, P> where P: AsRef<Path> {
	type Target = D;
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target {
		&self.data
	}
}

impl<D, P> DerefMut for MaybeAutoRemovePath<D, P> where P: AsRef<Path> {
	#[inline(always)]
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.data
	}
}

impl<D, P> Drop for MaybeAutoRemovePath<D, P> where P: AsRef<Path> {
	#[inline]
	fn drop(&mut self) {
		if let Some(path) = &self.is_remove_path {
			let _e = std::fs::remove_file(path);
		}
	}
}


#[derive(Debug)]
pub struct AutoRemovePath<D, P> where P: AsRef<Path> {
	data: D,
	path: P,
}

impl<D, P> FlElement for AutoRemovePath<D, P> where P: AsRef<Path> {
	type Target = D;
	
	#[inline]
	fn is_auto_remove_path(&self) -> bool {
		true
	}
}


impl<D, P> AutoRemovePath<D, P> where P: AsRef<Path> {
	pub fn new(data: D, path: P) -> Self {
		Self::__new(data, path)
	}
	
	#[inline]
	const fn __new(data: D, path: P) -> Self {
		Self {
			data: data,
			path: path,
		}
	}
}

impl<D, P> Deref for AutoRemovePath<D, P> where P: AsRef<Path> {
	type Target = D;
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target {
		&self.data
	}
}

impl<D, P> DerefMut for AutoRemovePath<D, P> where P: AsRef<Path> {
	#[inline(always)]
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.data
	}
}

impl<D, P> Drop for AutoRemovePath<D, P> where P: AsRef<Path> {
	fn drop(&mut self) {
		let _e = std::fs::remove_file(&self.path);
	}
}