

use std::path::PathBuf;
use LockProjectConst;
use file_system::ErrFileSysLock;
use LockProject;
use std::fs::File;
use std::fs;

#[derive(Debug)]
pub struct LockBufEasy(PathBuf);

impl LockBufEasy {
     #[inline]
     const fn new(path: PathBuf) -> Self {
          LockBufEasy(path)
     }

     #[inline]
     fn _create_file(path: PathBuf) -> Result<Self, ErrFileSysLock> {
          let _file = match File::create(&path) {
               Ok(a) => a,
               Err(e) => return Err( ErrFileSysLock::ErrIo(e) ),
          };

          Ok( Self::new(path) )
     }
}

impl LockProjectConst<PathBuf> for LockBufEasy {
     type LockProject = Result<Self, ErrFileSysLock>;
     fn create(path: PathBuf) -> Self::LockProject {
          if path.exists() {
               return Err( ErrFileSysLock::LockExists )
          }

          Self::_create_file(path)
     }

     fn recovery(path: PathBuf) -> Self::LockProject {
          if path.exists() {
               return Ok( Self::new(path) );
          }

          Self::_create_file(path)
     }
}

impl LockProject for LockBufEasy {
     #[inline]
     fn is_lock(&self) -> bool {
          self.0.exists()
     }
}



impl<'a> Drop for LockBufEasy {
     #[inline]
     fn drop(&mut self) {
          let _e = fs::remove_file(&self.0);
     }
}
