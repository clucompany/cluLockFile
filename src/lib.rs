


mod file_system;
use std::ops::DerefMut;
use std::fs::File;
use std::ops::Deref;
pub use self::file_system::*;

use std::fmt::Debug;

pub trait Lock: Debug {
     fn is_lock(&self) -> bool;

     fn unlock(self) where Self: Sized {}
     fn unlock_boxed(self: Box<Self>) where Self: Sized {}
}


pub trait FileExpLock: Lock + Debug + Deref<Target = File> + DerefMut + AsRef<File> + AsMut<File> {}


