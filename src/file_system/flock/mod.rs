

mod path;
mod file;

use crate::FileExpLock;
use crate::Lock;
use std::path::Path;
use std::io::Error;
use std::fs::File;
use std::path::PathBuf;
pub use self::path::*;
pub use self::file::*;


#[inline(always)]
pub fn flock_lock<A: ConstFlockFileLock>(a: A) -> Result<A::LockFile, Error> {
     a.flock_lock()
}

#[inline(always)]
pub fn flock_recovery_lock<A: ConstFlockFileLock>(a: A) -> Result<A::LockFile, Error> {
     a.flock_recovery_lock()
}

pub trait ConstFlockFileLock {
     type LockFile: Lock + FileExpLock;


     fn flock_lock(self) -> Result<Self::LockFile, Error>;
     fn flock_recovery_lock(self) -> Result<Self::LockFile, Error>;
}

/*
impl<'a, A: AsRef<File>> ConstFlockFileLock for &'a A {
     type LockFile = FlockLockRawFileSlice<'a>;

     #[inline(always)]
     fn flock_lock(self) -> Result<Self::LockFile, Error> {
          FlockLockRawFileSlice::lock(self.as_ref())
     }
}


impl<'a, A: AsMut<File>> ConstFlockFileLock for &'a mut A {
     type LockFile = FlockLockRawFileSlice<'a>;

     #[inline(always)]
     fn flock_lock(self) -> Result<Self::LockFile, Error> {
          FlockLockRawFileSlice::lock(self.as_mut())
     }
}*/


impl ConstFlockFileLock for File {
     type LockFile = FlockLockRawFile;

     #[inline(always)]
     fn flock_lock(self) -> Result<Self::LockFile, Error> {
          FlockLockRawFile::lock(self)
     }

     #[inline(always)]
     fn flock_recovery_lock(self) -> Result<Self::LockFile, Error> {
          FlockLockRawFile::lock(self)
     }
}


impl ConstFlockFileLock for PathBuf {
     type LockFile = FlockLockFile;

     #[inline(always)]
     fn flock_lock(self) -> Result<Self::LockFile, Error> {
          FlockLockFile::lock(self)
     }

     #[inline(always)]
     fn flock_recovery_lock(self) -> Result<Self::LockFile, Error> {
          FlockLockFile::recovery(self)
     }
}


impl<'a, A: AsRef<Path>> ConstFlockFileLock for &'a A {
     type LockFile = FlockLockFileSlice<'a>;

     #[inline(always)]
     fn flock_lock(self) -> Result<Self::LockFile, Error> {
          FlockLockFileSlice::lock(self.as_ref())
     }

     #[inline(always)]
     fn flock_recovery_lock(self) -> Result<Self::LockFile, Error> {
          FlockLockFileSlice::recovery(self.as_ref())
     }
}

impl<'a, A: AsMut<Path>> ConstFlockFileLock for &'a mut A {
     type LockFile = FlockLockFileSlice<'a>;

     #[inline(always)]
     fn flock_lock(self) -> Result<Self::LockFile, Error> {
          FlockLockFileSlice::lock(self.as_mut())
     }

     #[inline(always)]
     fn flock_recovery_lock(self) -> Result<Self::LockFile, Error> {
          FlockLockFileSlice::recovery(self.as_mut())
     }
}


impl<'a> ConstFlockFileLock for &'a Path {
     type LockFile = FlockLockFileSlice<'a>;

     #[inline(always)]
     fn flock_lock(self) -> Result<Self::LockFile, Error> {
          FlockLockFileSlice::lock(self)
     }

     #[inline(always)]
     fn flock_recovery_lock(self) -> Result<Self::LockFile, Error> {
          FlockLockFileSlice::recovery(self)
     }
}

impl<'a> ConstFlockFileLock for &'a mut Path {
     type LockFile = FlockLockFileSlice<'a>;

     #[inline(always)]
     fn flock_lock(self) -> Result<Self::LockFile, Error> {
          FlockLockFileSlice::lock(self)
     }

     #[inline(always)]
     fn flock_recovery_lock(self) -> Result<Self::LockFile, Error> {
          FlockLockFileSlice::recovery(self)
     }
}