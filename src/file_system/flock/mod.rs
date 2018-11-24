
//extern crate cluFlock;

mod buf;
mod slice;

use std::io::Error;
use std::path::Path;
use std::path::PathBuf;
pub use self::buf::*;
pub use self::slice::*;


#[inline(always)]
pub fn flock<A: LockFlockConst>(a: A) -> A::LockFile {
     a.flock_lock()
}

#[inline(always)]
pub fn flock_reclock<A: LockFlockConst>(a: A) -> A::LockFile {
     a.flock_lock()
}

pub trait LockFlockConst {
     type LockFile;

     fn flock_lock(self) -> Self::LockFile;
     fn flock_reclock(self) -> Self::LockFile;
}

impl<'a> LockFlockConst for PathBuf {
     type LockFile = Result<LockFlockBuf, Error>;

     #[inline(always)]
     fn flock_lock(self) -> Self::LockFile {
          LockFlockBuf::lock(self)
     }

     #[inline(always)]
     fn flock_reclock(self) -> Self::LockFile {
          LockFlockBuf::recovery(self)
     }
}


impl<'a, A: AsRef<Path>> LockFlockConst for &'a A {
     type LockFile = Result<LockFlock<'a>, Error>;

     #[inline(always)]
     fn flock_lock(self) -> Self::LockFile {
          LockFlock::lock(self.as_ref())
     }

     #[inline(always)]
     fn flock_reclock(self) -> Self::LockFile {
          LockFlock::recovery(self.as_ref())
     }
}



impl<'a> LockFlockConst for &'a Path {
     type LockFile = Result<LockFlock<'a>, Error>;

     #[inline(always)]
     fn flock_lock(self) -> Self::LockFile {
          LockFlock::lock(self)
     }

     #[inline(always)]
     fn flock_reclock(self) -> Self::LockFile {
          LockFlock::recovery(self)
     }
}