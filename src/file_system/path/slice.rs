

use Lock;
use std::io::Error;
use std::path::Path;
use std::fs;
use std::fs::OpenOptions;


#[derive(Debug)]
pub struct PathSliceLock<'a>(&'a Path);

impl<'a> PathSliceLock<'a> {
     #[inline]
     const fn new(path: &'a Path) -> Self {
          PathSliceLock(path)
     }

     pub fn lock(path: &'a Path) -> Result<Self, Error> {
          match OpenOptions::new().write(true).create_new(true).open(path) {
               Ok(file) => file,
               //Err(ref e) if e.kind() == AlreadyExists => return Err(Error::new(ErrorKind::Other, "the file is already locked")),
               Err(e) => return Err( e ),
          };

          Ok( Self::new(path) )
     }
}

impl<'a> Lock for PathSliceLock<'a> {
     fn is_lock(&self) -> bool {
          self.0.exists()
     }
}

impl<'a> Drop for PathSliceLock<'a> {
     #[inline]
     fn drop(&mut self) {
          let _e = fs::remove_file(self.0);
     }
}
