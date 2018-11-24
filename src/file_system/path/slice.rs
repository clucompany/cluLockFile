

use std::io::ErrorKind::AlreadyExists;
use std::io::Error;
use std::io::ErrorKind;
use Locker;
use std::path::Path;
use std::fs;
use std::fs::OpenOptions;

#[derive(Debug)]
pub struct LockPath<'a>(&'a Path);

impl<'a> LockPath<'a> {
     #[inline]
     const fn new(path: &'a Path) -> Self {
          LockPath(path)
     }

     pub fn lock(path: &'a Path) -> Result<Self, Error> {
          match OpenOptions::new().write(true).create_new(true).open(path) {
               Ok(file) => file,
               Err(ref e) if e.kind() == AlreadyExists => return Err(Error::new(ErrorKind::Other, "the file is already locked")),
               Err(e) => return Err( e ),
          };

          Ok( Self::new(path) )
     }
}

impl<'a> Locker for LockPath<'a> {
     #[inline]
     fn is_lock(&self) -> bool {
          self.0.exists()
     }
}



impl<'a> Drop for LockPath<'a> {
     #[inline]
     fn drop(&mut self) {
          let _e = fs::remove_file(self.0);
     }
}
