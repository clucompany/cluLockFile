
extern crate cluFlock;

use Locker;
use self::cluFlock::ExclusiveLock;
use self::cluFlock::Flock;
use std::io::ErrorKind::AlreadyExists;
use std::path::Path;
use std::fs;
use std::io::Error;
use std::io::ErrorKind;
use std::fs::OpenOptions;

#[derive(Debug)]
pub struct LockFlock<'a>(ExclusiveLock, Option<&'a Path>);

impl<'a> LockFlock<'a> {
     #[inline]
     const fn new(a: ExclusiveLock, b: Option<&'a Path>) -> Self {
          LockFlock(a, b)
     }

     pub fn lock(arg: &'a Path) -> Result<Self, Error> {
          let lock = match OpenOptions::new().write(true).create_new(true).open(arg) {
               Ok(file) => file.file_exclusive_lock()?,
               Err(ref e) if e.kind() == AlreadyExists => return Err(Error::new(ErrorKind::Other, "the file is already locked")),
               Err(e) => return Err(e),
          };

          Ok( Self::new(lock, Some(arg)) )
     }

     pub fn recovery(arg: &'a Path) -> Result<Self, Error> {
          let (lock, path) = match OpenOptions::new().write(true).create_new(true).open(arg) {
               Ok(file) => (file.file_exclusive_lock()?, Some(arg)),
               Err(ref e) if e.kind() == AlreadyExists => {
                    let f = OpenOptions::new().read(true).open(arg)?; 

                    match f.try_file_exclusive_lock() {
                         Ok(Some(lock)) => (lock, None),
                         Ok(None) => return Err(Error::new(ErrorKind::Other, "the file is already locked")),
                         Err(e) => return Err(e),
                    }
               },
               Err(e) => return Err(e),
          };

          Ok( Self::new(lock, path) )
     }
}

impl<'a> Locker for LockFlock<'a> {
     #[inline(always)]
     fn is_lock(&self) -> bool {
          true
     }
}

impl<'a> Drop for LockFlock<'a> {
     fn drop(&mut self) {
          if let Some(path) = self.1 {
               let _e = fs::remove_file(path);
          }
     }
}
