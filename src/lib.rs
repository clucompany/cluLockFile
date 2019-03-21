#![feature(const_fn)]
#![allow(non_snake_case)]

mod check;
pub use self::check::*;

mod type_check;
pub use self::type_check::*;

mod error;
pub use self::error::*;

pub mod file_system;


pub trait Locker {
	fn is_lock(&self) -> IsLock;
	fn check_lock(self) -> CheckLock<Self> where Self: Sized;
	fn exists(&self) -> bool;
}

