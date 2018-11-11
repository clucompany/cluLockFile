
mod file_system;
pub use self::file_system::*;


use std::fmt::Debug;


pub trait LockFile: Debug + Drop {
     fn is_lock(&self) -> bool;

     fn unlock(self) where Self: Sized {}
     fn boxed(self) -> Box<LockFile> where Self: Sized + 'static {
          Box::new(self)
     }
}


pub trait LockFileConst<A> {
     type LockFile;

     fn create(arg: A) -> Self::LockFile;
     fn recovery(arg: A) -> Self::LockFile;
}
