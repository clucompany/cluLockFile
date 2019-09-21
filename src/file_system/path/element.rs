
use std::path::Path;
use std::path::PathBuf;

pub trait PathElement {
	fn path_exists(&self) -> bool;
	fn as_ref(&self) -> &Path;
	fn remove_file(&self) -> Result<(), std::io::Error>;
}

impl PathElement for PathBuf {
	#[inline(always)]
	fn path_exists(&self) -> bool {
		(**self).exists()
	}
	
	#[inline(always)]
	fn as_ref(&self) -> &Path {
		self
	}
	
	#[inline(always)]
	fn remove_file(&self) -> Result<(), std::io::Error> {
		std::fs::remove_file(self)	
	}
}

impl<'a> PathElement for &'a Path {
	#[inline(always)]
	fn path_exists(&self) -> bool {
		(**self).exists()
	}
	
	#[inline(always)]
	fn as_ref(&self) -> &Path {
		self
	}
	
	#[inline(always)]
	fn remove_file(&self) -> Result<(), std::io::Error> {
		std::fs::remove_file(self)
	}
}

impl<'a> PathElement for &'a mut Path {
	#[inline(always)]
	fn path_exists(&self) -> bool {
		(**self).exists()
	}
	
	#[inline(always)]
	fn as_ref(&self) -> &Path {
		self
	}
	
	#[inline(always)]
	fn remove_file(&self) -> Result<(), std::io::Error> {
		std::fs::remove_file(self)
	}
}

impl<'a, A> PathElement for &'a A where A: AsRef<Path> {
	#[inline(always)]
	fn path_exists(&self) -> bool {
		A::as_ref(self).exists()
	}
	
	#[inline(always)]
	fn as_ref(&self) -> &Path {
		A::as_ref(self)
	}
	
	#[inline(always)]
	fn remove_file(&self) -> Result<(), std::io::Error> {
		std::fs::remove_file(A::as_ref(self))
	}
}

impl<'a, A> PathElement for &'a mut A where A: AsRef<Path> {
	#[inline(always)]
	fn path_exists(&self) -> bool {
		A::as_ref(self).exists()
	}
	
	#[inline(always)]
	fn as_ref(&self) -> &Path {
		A::as_ref(self)
	}
	
	#[inline(always)]
	fn remove_file(&self) -> Result<(), std::io::Error> {
		std::fs::remove_file(A::as_ref(self))
	}
}

