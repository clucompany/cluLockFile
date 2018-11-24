
mod file_system;
pub use self::file_system::*;


use std::fmt::Debug;


pub trait Locker: Debug + Drop {
     fn is_lock(&self) -> bool;

     fn unlock(self) where Self: Sized {}
     fn boxed(self: Box<Self>) where Self: Sized {}
}

