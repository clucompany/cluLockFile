

mod slice_path;
mod buf_path;

use file_system::ErrFileSysLock;
use std::path::Path;
use std::path::PathBuf;
pub use self::slice_path::*;
pub use self::buf_path::*;
use LockProjectConst;

pub trait LockEasyConst {
     type LockProject;

     fn create_elockproject(self) -> Self::LockProject;
     fn recovery_elockproject(self) -> Self::LockProject;
}

impl<'a> LockEasyConst for PathBuf {
     type LockProject = Result<LockBufEasy, ErrFileSysLock>;

     #[inline]
     fn create_elockproject(self) -> Self::LockProject {
          LockBufEasy::create(self)
     }

     #[inline]
     fn recovery_elockproject(self) -> Self::LockProject {
          LockBufEasy::recovery(self)
     }
}


impl<'a, A: AsRef<Path>> LockEasyConst for &'a A {
     type LockProject = Result<LockSliceEasy<'a>, ErrFileSysLock>;

     #[inline]
     fn create_elockproject(self) -> Self::LockProject {
          LockSliceEasy::create(self.as_ref())
     }

     #[inline]
     fn recovery_elockproject(self) -> Self::LockProject {
          LockSliceEasy::recovery(self.as_ref())
     }
}


/*
impl<'a> LockEasyConst for &'a PathBuf {
     type LockProject = Result<LockSliceEasy<'a>, ErrFileSysLock>;

     #[inline]
     fn create_elockproject(self) -> Self::LockProject {
          LockSliceEasy::create(self)
     }

     #[inline]
     fn recovery_elockproject(self) -> Self::LockProject {
          LockSliceEasy::create(self)
     }
}*/


impl<'a> LockEasyConst for &'a Path {
     type LockProject = Result<LockSliceEasy<'a>, ErrFileSysLock>;

     #[inline]
     fn create_elockproject(self) -> Self::LockProject {
          LockSliceEasy::create(self)
     }

     #[inline]
     fn recovery_elockproject(self) -> Self::LockProject {
          LockSliceEasy::recovery(self)
     }
}