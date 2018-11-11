
extern crate cluFlock;


use LockFile;
use LockFileConst;
use file_system::ErrFileSysLock;
use std::path::Path;
use std::fs::File;
use std::fs;
use self::cluFlock::ExclusiveFlockLock;
use self::cluFlock::Flock;

#[derive(Debug)]
pub struct LockFlock<'a>(File, &'a Path, ExclusiveFlockLock<'a>);


impl<'a> LockFlock<'a> {
     #[inline]
     fn new(file: File, path: &'a Path, lock: ExclusiveFlockLock<'a>) -> Self {
          LockFlock(file, path, lock)
     }
}

impl<'a> LockFileConst<&'a Path> for LockFlock<'a> {
     type LockFile = Result<Self, ErrFileSysLock>;
     fn create(path: &'a Path) -> Self::LockFile {
          match path.exists() {
               true => return Err( ErrFileSysLock::LockExists ),
               _ => {
                    let file = match File::open(&path) {
                         Ok(a) => a,
                         Err(e) => return Err( ErrFileSysLock::ErrIo(e) ),
                    };

                    let lock = match file.exclusive_lock() {
                         Ok(lock) => lock,
                         Err(e) => return Err( ErrFileSysLock::ErrIo(e) ),
                    };

                    Ok( Self::new(file, path, lock) )
               },
          }
     }

     #[inline]
     fn recovery(path: &'a Path) -> Self::LockFile {
          match path.exists() {
               true => {
                    let file = match File::open(&path) {
                         Ok(a) => a,
                         Err(e) => return Err( ErrFileSysLock::ErrIo(e) ),
                    };

                    let lock = match file.try_exclusive_lock() {
                         Ok(Some(lock)) => lock,
                         Ok(None) => return Err( ErrFileSysLock::LockExists ),
                         Err(e) => return Err( ErrFileSysLock::ErrIo(e) ),
                    };

                    Ok( Self::new(file, path, lock) )
               },
               _ => {
                    let file = match File::open(&path) {
                         Ok(a) => a,
                         Err(e) => return Err( ErrFileSysLock::ErrIo(e) ),
                    };

                    let lock = match file.exclusive_lock() {
                         Ok(lock) => lock,
                         Err(e) => return Err( ErrFileSysLock::ErrIo(e) ),
                    };

                    Ok( Self::new(file, path, lock) )
               },
          }
     }
}

impl<'a> LockFile for LockFlock<'a> {
     #[inline]
     fn is_lock(&self) -> bool {
          true
     }
}



impl<'a> Drop for LockFlock<'a> {
     #[inline]
     fn drop(&mut self) {
          drop(self.0);
          drop(self.2);
          let _e = fs::remove_file(&self.1);
     }
}


