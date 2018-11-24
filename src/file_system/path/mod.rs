

mod slice;
mod buf;

use std::io::Error;
use std::path::Path;
use std::path::PathBuf;
pub use self::slice::*;
pub use self::buf::*;


#[inline(always)]
pub fn path_lock<A: LockPathConst>(a: A) -> A::LockFile {
     a.path_lock()
}

pub trait LockPathConst {
     type LockFile;

     fn path_lock(self) -> Self::LockFile;
}

impl<'a> LockPathConst for PathBuf {
     type LockFile = Result<LockPathBuf, Error>;

     #[inline(always)]
     fn path_lock(self) -> Self::LockFile {
          LockPathBuf::lock(self)
     }
}


impl<'a, A: AsRef<Path>> LockPathConst for &'a A {
     type LockFile = Result<LockPath<'a>, Error>;

     #[inline(always)]
     fn path_lock(self) -> Self::LockFile {
          LockPath::lock(self.as_ref())
     }
}



impl<'a> LockPathConst for &'a Path {
     type LockFile = Result<LockPath<'a>, Error>;

     #[inline(always)]
     fn path_lock(self) -> Self::LockFile {
          LockPath::lock(self)
     }
}