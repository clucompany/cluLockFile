
extern crate cluFlock;


use crate::FileExpLock;
use crate::Lock;


use std::ops::DerefMut;
use std::ops::Deref;
use std::fs::File;
use self::cluFlock::FileFlock;
use self::cluFlock::ToFlock;
use std::path::PathBuf;
use std::io::ErrorKind::AlreadyExists;
use std::fs;
use std::io::Error;
use std::fs::OpenOptions;


#[derive(Debug)]
pub struct FlockLockFile(FileFlock, Option<DropPathBuf>);

impl FileExpLock for FlockLockFile {}


impl FlockLockFile {
     #[inline]
     const fn new(a: FileFlock, b: Option<DropPathBuf>) -> Self {
          FlockLockFile(a, b)
     }

     pub fn file(f: File) -> Result<Self, Error> {
          Ok( Self::new(f.try_exclusive_lock()?, None) )
     }

     pub fn lock(path: PathBuf) -> Result<Self, Error> {
          let (file_lock, drop_path) = match OpenOptions::new().write(true).create_new(true).open(&path) {
               Ok(file) => {
                    let drop_path = DropPathBuf::new(path);

                    (file.try_exclusive_lock()?, drop_path)
               },
               Err(e) => return Err(e),
          };

          Ok( Self::new(file_lock, Some(drop_path)) )
     }

     pub fn recovery(path: PathBuf) -> Result<Self, Error> {
          let (lock, option_drop_path) = match OpenOptions::new().write(true).create_new(true).open(&path) {
               Ok(file) => {
                    let drop_path = DropPathBuf::new(path);
                    
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

impl Lock for FlockLockFile {
     #[inline(always)]
     fn is_lock(&self) -> bool {
          true
     }
}


impl AsRef<File> for FlockLockFile {
     #[inline(always)]
     fn as_ref(&self) -> &File {
          &self.0
     }
}

impl AsMut<File> for FlockLockFile {
     #[inline(always)]
     fn as_mut(&mut self) -> &mut File {
          &mut self.0
     }
}


impl Deref for FlockLockFile {
     type Target = File;

     #[inline(always)]
     fn deref(&self) -> &Self::Target {
          &self.0
     }
}

impl DerefMut for FlockLockFile {
     #[inline(always)]
     fn deref_mut(&mut self) -> &mut Self::Target {
          &mut self.0
     }
}

#[derive(Debug)]
struct DropPathBuf(PathBuf);

impl DropPathBuf {
     #[inline]
     const fn new(p: PathBuf) -> Self {
          DropPathBuf(p)
     }
}

impl Drop for DropPathBuf {
     fn drop(&mut self) {
          let _e = fs::remove_file(&self.0);
     }
}
