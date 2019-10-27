#![allow(non_snake_case)]
#![feature(const_fn)]
#![feature(const_constructor)]

pub mod state {
	mod active_state;
	pub use self::active_state::*;
	
	mod move_active_state;
	pub use self::move_active_state::*;
}

mod err;

use crate::state::ActiveLockState;
use crate::state::MoveActiveLockState;
pub use self::err::*;


pub mod file_system {
	pub mod flock;
	pub mod path;
}


pub type DefLockFile<D> = crate::file_system::flock::FlLockFile<D>;

pub trait LockFile {
	fn is_active_lock(&self) -> bool;
	
	fn move_active_lock_state(self) -> MoveActiveLockState<Self> where Self: Sized;
	fn active_lock_state(&self) -> ActiveLockState;
}

