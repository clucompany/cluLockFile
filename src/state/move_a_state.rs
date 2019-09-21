
use std::fmt::Display;
use std::fmt;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum MoveActiveSyncState<T> {
	AlwaysOn(T),
	AlwaysOff,
	On(T),
	Off,
}

impl<T> MoveActiveSyncState<T> {
	#[inline]
	pub const fn always_on(t: T) -> Self {
		Self::AlwaysOn(t)
	}
	
	#[inline]
	pub const fn always_off() -> Self {
		Self::AlwaysOff
	}
	
	#[inline]
	pub const fn on(t: T) -> Self {
		Self::On(t)
	}
	
	#[inline]
	pub const fn off() -> Self {
		Self::Off
	}
	
	
	#[inline]
	pub fn is_lock(&self) -> bool {
		match self {
			Self::AlwaysOn(_)		=> true,
			Self::AlwaysOff		=> false,
			Self::On(_)			=> true,
			Self::Off				=> false,
		}
	}
	
	#[inline]
	pub fn into(self) -> Option<T> {
		match self {
			Self::AlwaysOn(a)	=>	Some(a),
			Self::AlwaysOff	=>	None,
			Self::On(a)		=>	Some(a),
			Self::Off			=>	None,
		}
	}
}

impl<T> From<T> for MoveActiveSyncState<T> {
	#[inline(always)]
	fn from(a: T) -> Self {
		Self::AlwaysOn(a)
	}
}

impl<T> Into<Option<T>> for MoveActiveSyncState<T> {
	#[inline(always)]
	fn into(self) -> Option<T> {
		MoveActiveSyncState::into(self)
	}
}


impl<T> Display for MoveActiveSyncState<T> {
	fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		let str = match self {
			Self::AlwaysOn(_)	=> "MoveActiveSyncState::AlwaysOn",
			Self::AlwaysOff	=> "MoveActiveSyncState::AlwaysOff",
			Self::On(_)		=> "MoveActiveSyncState::On(<data>)",
			Self::Off			=> "MoveActiveSyncState::Off",
		};
		
		fmt.write_str(str)
	}
}

