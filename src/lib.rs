
mod file_system;
pub use self::file_system::*;


use std::fmt::Debug;


pub trait LockProject: Debug + Drop {
     fn is_lock(&self) -> bool;

     fn unlock(self) where Self: Sized {}
}


pub trait LockProjectConst<A> {
     type LockProject;

     fn create(arg: A) -> Self::LockProject;
     fn recovery(arg: A) -> Self::LockProject;
}
