

use std::path::PathBuf;
use LockFileConst;
use file_system::ErrFileSysLock;
use LockFile;
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

impl LockFileConst<PathBuf> for LockBufEasy {
     type LockFile = Result<Self, ErrFileSysLock>;
     fn create(path: PathBuf) -> Self::LockFile {
          if path.exists() {
               return Err( ErrFileSysLock::LockExists )
          }

          Self::_create_file(path)
     }

     #[inline]
     fn recovery(_path: PathBuf) -> Self::LockFile {
          Err( ErrFileSysLock::RecoveryNotSupported )
     }
}

impl LockFile for LockBufEasy {
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
