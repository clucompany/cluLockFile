#![allow(non_snake_case)]
#![feature(const_fn)]
#![feature(const_constructor)]

use crate::state::ToLockState;

pub mod state {
	mod a_state;
	pub use self::a_state::*;
	
	mod move_a_state;
	pub use self::move_a_state::*;
	
	mod to;
	pub use self::to::*;
}

mod err;
pub use self::err::*;


pub mod file_system {
	pub mod flock;
	pub mod path;
}


pub type DefSyncFile<D> = crate::file_system::flock::FlockSyncFile<D>;


pub trait SyncFile: ToLockState {
	fn is_sync(&self) -> bool;
}

