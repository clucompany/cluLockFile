
use cluFlock::unlock::WaitFlockUnlock;
use cluFlock::FlockLock;
use cluFlock::element::FlockElement;
use std::ops::Deref;
use std::ops::DerefMut;
use std::path::Path;


pub trait FSFElement {}

impl<T> FSFElement for FlockLock<T> where T: FlockElement + WaitFlockUnlock {}


#[derive(Debug)]
pub enum DataOrRemoveData<D, P> where P: AsRef<Path> {
	Remove(AutoRemovePath<D, P>),
	Data(D)
}

impl<D, P> FSFElement for DataOrRemoveData<D, P> where P: AsRef<Path> {}

impl<D, P> DataOrRemoveData<D, P> where P: AsRef<Path> {
	#[inline(always)]
	pub fn remove_file(p: AutoRemovePath<D, P>) -> Self {
		Self::Remove(p)
	}
	
	#[inline(always)]
	pub fn data(d: D) -> Self {
		Self::Data(d)
	}
}

impl<D, P> Deref for DataOrRemoveData<D, P> where P: AsRef<Path> {
	type Target = D;
	
	#[inline]
	fn deref(&self) -> &Self::Target {
		match self {
			Self::Remove(a) => a.deref(),
			Self::Data(a) => a,
		}
	}
}

impl<D, P> DerefMut for DataOrRemoveData<D, P> where P: AsRef<Path> {
	#[inline]
	fn deref_mut(&mut self) -> &mut Self::Target {
		match self {
			Self::Remove(a) => a.deref_mut(),
			Self::Data(a) => a,
		}
	}
}


#[derive(Debug)]
pub struct PrevAutoRemovePath<P> where P: AsRef<Path> {
	path: P,
}

#[derive(Debug)]
struct __PrevAutoRemovePath<P> where P: AsRef<Path> {
	path: P
}

impl<P> PrevAutoRemovePath<P> where P: AsRef<Path> {
	#[inline]
	pub const fn new(path: P) -> Self {
		Self {
			path: path
		}
	}
	
	pub fn into_dont_remove_file(self) -> P {
		let new_self: __PrevAutoRemovePath<P> = unsafe {
			cluFullTransmute::mem::full_transmute(std::mem::ManuallyDrop::new(self))
		};
		
		new_self.path
	}
	
	pub fn into_remove_file(self) -> (P, Result<(), std::io::Error>) {
		let new_self: __PrevAutoRemovePath<P> = unsafe {
			cluFullTransmute::mem::full_transmute(std::mem::ManuallyDrop::new(self))
		};
		
		let remove_file = std::fs::remove_file(&new_self.path);
		( new_self.path, remove_file )
	}
	
	#[inline]
	pub fn to_auto_remove_path<D>(self, data: D) -> AutoRemovePath<D, P> {
		AutoRemovePath::prev(self, data)
	}
}

impl<P> Deref for PrevAutoRemovePath<P> where P: AsRef<Path> {
	type Target = P;
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target {
		&self.path
	}
}

impl<P> DerefMut for PrevAutoRemovePath<P> where P: AsRef<Path> {
	#[inline(always)]
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.path
	}
}

impl<P> AsRef<Path> for PrevAutoRemovePath<P> where P: AsRef<Path> {
	#[inline(always)]
	fn as_ref(&self) -> &Path {
		self.path.as_ref()
	}
}

impl<P> Drop for PrevAutoRemovePath<P> where P: AsRef<Path> {
	#[inline]
	fn drop(&mut self) {
		let _e = std::fs::remove_file(self.path.as_ref());
	}
}


#[derive(Debug)]
pub struct AutoRemovePath<D, P> where P: AsRef<Path> {
	data: D,
	path: PrevAutoRemovePath<P>,
}

impl<D, P> FSFElement for AutoRemovePath<D, P> where P: AsRef<Path> {}


impl<D, P> AutoRemovePath<D, P> where P: AsRef<Path> {
	#[inline]
	pub fn prev(prev: PrevAutoRemovePath<P>, data: D) -> Self {
		Self::__new(data, prev)
	}
	
	#[inline]
	const fn __new(data: D, path: PrevAutoRemovePath<P>) -> Self {
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

impl<D, P> AsRef<Path> for AutoRemovePath<D, P> where P: AsRef<Path> {
	#[inline(always)]
	fn as_ref(&self) -> &Path {
		self.path.as_ref()
	}
}
