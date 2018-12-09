
extern crate cluFlock;


use crate::Lock;


use std::ops::Deref;
use std::fs::File;
use std::io::Error;
use self::cluFlock::FileSliceFlock;
use self::cluFlock::Flock;

#[derive(Debug)]
pub struct FlockLockRawFileSlice<'a>(FileSliceFlock<'a>);

//impl<'a> FileExpLock for FlockLockRawFileSlice<'a> {}

impl<'a> FlockLockRawFileSlice<'a> {
     #[inline]
     const fn new(a: FileSliceFlock<'a>) -> Self {
          FlockLockRawFileSlice(a)
     }

     pub fn lock(arg: &'a std::fs::File) -> Result<Self, Error> {
          Ok( Self::new(arg.try_exclusive_lock()?) )
     }
}


impl<'a> AsRef<File> for FlockLockRawFileSlice<'a> {
     #[inline(always)]
     fn as_ref(&self) -> &File {
          &self.0
     }
}



impl<'a> Deref for FlockLockRawFileSlice<'a> {
     type Target = File;

     #[inline(always)]
     fn deref(&self) -> &Self::Target {
          &self.0
     }
}

/*impl<'a> DerefMut for FlockLockRawFileSlice<'a> {
     #[inline(always)]
     fn deref_mut(&mut self) -> &mut Self::Target {
          &mut self.0
     }
}*/

impl<'a> Lock for FlockLockRawFileSlice<'a> {
     #[inline(always)]
     fn is_lock(&self) -> bool {
          true
     }
}
