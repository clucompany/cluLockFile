

mod slice;
mod buf;

pub use self::slice::*;
pub use self::buf::*;
use crate::Lock;

use std::io::Error;
use std::path::Path;
use std::path::PathBuf;



#[inline(always)]
pub fn path_lock<A: ConstPathLock>(a: A) -> Result<A::LockFile, Error> {
     a.path_lock()
}

pub trait ConstPathLock {
     type LockFile: Lock;

     fn path_lock(self) -> Result<Self::LockFile, Error>;
}

impl ConstPathLock for PathBuf {
     type LockFile = PathLock;

     #[inline(always)]
     fn path_lock(self) -> Result<Self::LockFile, Error> {
          PathLock::lock(self)
     }
}


impl<'a, A: AsRef<Path>> ConstPathLock for &'a A {
     type LockFile = PathSliceLock<'a>;

     #[inline(always)]
     fn path_lock(self) -> Result<Self::LockFile, Error> {
          PathSliceLock::lock(self.as_ref())
     }
}



impl<'a> ConstPathLock for &'a Path {
     type LockFile = PathSliceLock<'a>;

     #[inline(always)]
     fn path_lock(self) -> Result<Self::LockFile, Error> {
          PathSliceLock::lock(self)
     }
}
