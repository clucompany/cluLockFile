

mod slice_path;
mod buf_path;

use file_system::ErrFileSysLock;
use std::path::Path;
use std::path::PathBuf;
pub use self::slice_path::*;
pub use self::buf_path::*;
use LockFileConst;

pub trait LockEasyConst {
     type LockFile;

     fn create_elockfile(self) -> Self::LockFile;
     fn recovery_elockfile(self) -> Self::LockFile;
}

impl<'a> LockEasyConst for PathBuf {
     type LockFile = Result<LockBufEasy, ErrFileSysLock>;

     #[inline]
     fn create_elockfile(self) -> Self::LockFile {
          LockBufEasy::create(self)
     }

     #[inline]
     fn recovery_elockfile(self) -> Self::LockFile {
          LockBufEasy::recovery(self)
     }
}


impl<'a, A: AsRef<Path>> LockEasyConst for &'a A {
     type LockFile = Result<LockSliceEasy<'a>, ErrFileSysLock>;

     #[inline]
     fn create_elockfile(self) -> Self::LockFile {
          LockSliceEasy::create(self.as_ref())
     }

     #[inline]
     fn recovery_elockfile(self) -> Self::LockFile {
          LockSliceEasy::recovery(self.as_ref())
     }
}


/*
impl<'a> LockEasyConst for &'a PathBuf {
     type LockFile = Result<LockSliceEasy<'a>, ErrFileSysLock>;

     #[inline]
     fn create_elockfile(self) -> Self::LockFile {
          LockSliceEasy::create(self)
     }

     #[inline]
     fn recovery_elockfile(self) -> Self::LockFile {
          LockSliceEasy::create(self)
     }
}*/


impl<'a> LockEasyConst for &'a Path {
     type LockFile = Result<LockSliceEasy<'a>, ErrFileSysLock>;

     #[inline]
     fn create_elockfile(self) -> Self::LockFile {
          LockSliceEasy::create(self)
     }

     #[inline]
     fn recovery_elockfile(self) -> Self::LockFile {
          LockSliceEasy::recovery(self)
     }
}