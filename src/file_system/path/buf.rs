

use crate::Lock;

use std::io::Error;
use std::path::PathBuf;
use std::fs;
use std::fs::OpenOptions;

#[derive(Debug)]
pub struct PathLock(PathBuf);

impl PathLock {
     #[inline]
     const fn new(path: PathBuf) -> Self {
          PathLock(path)
     }

     pub fn lock(path: PathBuf) -> Result<Self, Error> {
          match OpenOptions::new().write(true).create_new(true).open(&path) {
               Ok(file) => file,
               //Err(ref e) if e.kind() == AlreadyExists => return Err(Error::new(ErrorKind::Other, "the file is already locked")),
               Err(e) => return Err( e ),
          };

          Ok( Self::new(path) )
     }
}

impl Lock for PathLock {
     fn is_lock(&self) -> bool {
          self.0.exists()
     }
}

impl Drop for PathLock {
     #[inline]
     fn drop(&mut self) {
          let _e = fs::remove_file(&self.0);
     }
}
