
extern crate cluFlock;


use crate::FileExpLock;
use crate::Lock;


use std::ops::Deref;
use std::ops::DerefMut;
use std::fs::File;
use std::path::Path;
use self::cluFlock::FileFlock;
use self::cluFlock::ToFlock;
use std::io::ErrorKind::AlreadyExists;
use std::fs;
use std::io::Error;
use std::fs::OpenOptions;

#[derive(Debug)]
pub struct FlockLockFileSlice<'a>(FileFlock, Option<DropPath<'a>>);

impl<'a> FileExpLock for FlockLockFileSlice<'a> {}


impl<'a> FlockLockFileSlice<'a> {
     #[inline]
     const fn new(a: FileFlock, b: Option<DropPath<'a>>) -> Self {
          FlockLockFileSlice(a, b)
     }

     pub fn file(f: File) -> Result<Self, Error> {
          Ok( Self::new(f.try_exclusive_lock()?, None) )
     }

     pub fn lock(path: &'a Path) -> Result<Self, Error> {
          let (file_lock, drop_path) = match OpenOptions::new().write(true).create_new(true).open(path) {
               Ok(file) => {
                    let drop_path = DropPath(path);

                    (file.try_exclusive_lock()?, drop_path)
               },
               Err(e) => return Err(e),
          };

          Ok( Self::new(file_lock, Some(drop_path)) )
     }

     pub fn recovery(path: &'a Path) -> Result<Self, Error> {
          let (lock, option_drop_path) = match OpenOptions::new().write(true).create_new(true).open(path) {
               Ok(file) => {
                    let drop_path = DropPath(path);
                    
                    ( file.wait_exclusive_lock()?, Some( drop_path ) )
               },
               Err(ref e) if e.kind() == AlreadyExists => {
                    let f = OpenOptions::new().read(true).open(path)?; 


                    (f.try_exclusive_lock()?, None)
               },
               Err(e) => return Err(e),
          };
          

          Ok( Self::new(lock, option_drop_path) )
     }
}

impl<'a> Lock for FlockLockFileSlice<'a> {
     #[inline(always)]
     fn is_lock(&self) -> bool {
          true
     }
}

impl<'a> AsRef<File> for FlockLockFileSlice<'a> {
     #[inline(always)]
     fn as_ref(&self) -> &File {
          &self.0
     }
}

impl<'a> AsMut<File> for FlockLockFileSlice<'a> {
     #[inline(always)]
     fn as_mut(&mut self) -> &mut File {
          &mut self.0
     }
}

impl<'a> Deref for FlockLockFileSlice<'a> {
     type Target = File;

     #[inline(always)]
     fn deref(&self) -> &Self::Target {
          &self.0
     }
}

impl<'a> DerefMut for FlockLockFileSlice<'a> {
     #[inline(always)]
     fn deref_mut(&mut self) -> &mut Self::Target {
          &mut self.0
     }
}


#[derive(Debug)]
struct DropPath<'a>(&'a Path);

impl<'a> Drop for DropPath<'a> {
     fn drop(&mut self) {
          let _e = fs::remove_file(self.0);
     }
}
