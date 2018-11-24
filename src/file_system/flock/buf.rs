
extern crate cluFlock;

use std::path::PathBuf;
use Locker;
use self::cluFlock::ExclusiveLock;
use self::cluFlock::Flock;
use std::io::ErrorKind::AlreadyExists;
use std::fs;
use std::io::Error;
use std::io::ErrorKind;
use std::fs::OpenOptions;

#[derive(Debug)]
pub struct LockFlockBuf(ExclusiveLock, Option<PathBuf>);

impl LockFlockBuf {
     #[inline]
     const fn new(a: ExclusiveLock, b: Option<PathBuf>) -> Self {
          LockFlockBuf(a, b)
     }

     pub fn lock(path: PathBuf) -> Result<Self, Error> {
          let lock = match OpenOptions::new().write(true).create_new(true).open(&path) {
               Ok(file) => file.file_exclusive_lock()?,
               Err(ref e) if e.kind() == AlreadyExists => return Err(Error::new(ErrorKind::Other, "the file is already locked")),
               Err(e) => return Err(e),
          };

          Ok( Self::new(lock, Some(path)) )
     }

     pub fn recovery(path: PathBuf) -> Result<Self, Error> {
          let (lock, path) = match OpenOptions::new().write(true).create_new(true).open(&path) {
               Ok(file) => (file.file_exclusive_lock()?, Some(path)),
               Err(ref e) if e.kind() == AlreadyExists => {
                    let f = OpenOptions::new().read(true).open(path)?; 

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


impl Locker for LockFlockBuf {
     #[inline(always)]
     fn is_lock(&self) -> bool {
          true
     }
}

impl Drop for LockFlockBuf {
     fn drop(&mut self) {
          if let Some(ref path) = self.1 {
               let _e = fs::remove_file(path);
          }
     }
}
