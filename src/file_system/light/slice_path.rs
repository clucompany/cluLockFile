

use LockProjectConst;
use file_system::ErrFileSysLock;
use LockProject;
use std::fs::File;
use std::path::Path;
use std::fs;

#[derive(Debug)]
pub struct LockSliceEasy<'a>(&'a Path);

impl<'a> LockSliceEasy<'a> {
     #[inline]
     const fn new(path: &'a Path) -> Self {
          LockSliceEasy(path)
     }

     #[inline]
     fn _create_file(path: &'a Path) -> Result<Self, ErrFileSysLock> {
          let _file = match File::create(&path) {
               Ok(a) => a,
               Err(e) => return Err( ErrFileSysLock::ErrIo(e) ),
          };

          Ok( Self::new(path) )
     }
}

impl<'a> LockProjectConst<&'a Path> for LockSliceEasy<'a> {
     type LockProject = Result<Self, ErrFileSysLock>;
     fn create(path: &'a Path) -> Self::LockProject {
          if path.exists() {
               return Err( ErrFileSysLock::LockExists )
          }

          Self::_create_file(path)
     }

     fn recovery(path: &'a Path) -> Self::LockProject {
          if path.exists() {
               return Ok( Self::new(path) );
          }

          Self::_create_file(path)
     }
}

impl<'a> LockProject for LockSliceEasy<'a> {
     #[inline]
     fn is_lock(&self) -> bool {
          self.0.exists()
     }
}



impl<'a> Drop for LockSliceEasy<'a> {
     #[inline]
     fn drop(&mut self) {
          let _e = fs::remove_file(self.0);
     }
}
