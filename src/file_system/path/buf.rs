

use std::io::Error;
use std::path::PathBuf;
use std::io::ErrorKind::AlreadyExists;
use Locker;
use std::fs;
use std::fs::OpenOptions;
use std::io::ErrorKind;

#[derive(Debug)]
pub struct LockPathBuf(PathBuf);

impl LockPathBuf {
     #[inline]
     const fn new(path: PathBuf) -> Self {
          LockPathBuf(path)
     }

     pub fn lock(path: PathBuf) -> Result<Self, Error> {
          match OpenOptions::new().write(true).create_new(true).open(&path) {
               Ok(file) => file,
               Err(ref e) if e.kind() == AlreadyExists => return Err(Error::new(ErrorKind::Other, "the file is already locked")),
               Err(e) => return Err( e ),
          };

          Ok( Self::new(path) )
     }
}

impl Locker for LockPathBuf {
     #[inline]
     fn is_lock(&self) -> bool {
          self.0.exists()
     }
}



impl Drop for LockPathBuf {
     #[inline]
     fn drop(&mut self) {
          let _e = fs::remove_file(&self.0);
     }
}
