
extern crate cluFlock;


use crate::FileExpLock;
use crate::Lock;


use std::ops::DerefMut;
use std::ops::Deref;
use self::cluFlock::FileFlock;
use std::io::Error;
use std::fs::File;
use self::cluFlock::Flock;

#[derive(Debug)]
pub struct FlockLockRawFile(FileFlock);

impl FileExpLock for FlockLockRawFile {}


impl FlockLockRawFile {
     #[inline]
     const fn new(a: FileFlock) -> Self {
          FlockLockRawFile(a)
     }

     pub fn lock(arg: File) -> Result<Self, Error> {
          Ok( Self::new(arg.try_exclusive_lock()?) )
     }

}

impl Lock for FlockLockRawFile {
     #[inline(always)]
     fn is_lock(&self) -> bool {
          true
     }
}

impl AsRef<File> for FlockLockRawFile {
     #[inline(always)]
     fn as_ref(&self) -> &File {
          &self.0
     }
}

impl AsMut<File> for FlockLockRawFile {
     #[inline(always)]
     fn as_mut(&mut self) -> &mut File {
          &mut self.0
     }
}



impl<'a> Deref for FlockLockRawFile {
     type Target = File;

     #[inline(always)]
     fn deref(&self) -> &Self::Target {
          &self.0
     }
}

impl<'a> DerefMut for FlockLockRawFile {
     #[inline(always)]
     fn deref_mut(&mut self) -> &mut Self::Target {
          &mut self.0
     }
}