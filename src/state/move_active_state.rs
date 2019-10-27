
use crate::state::ActiveLockState;
use std::fmt::Display;
use std::fmt;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum MoveActiveLockState<T> {
	AlwaysOn(T),
	AlwaysOff,
	
	On(T),
	Off,
}

impl<T> MoveActiveLockState<T> {
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
			Self::AlwaysOn(_)	=> true,
			Self::AlwaysOff => false,
			Self::On(_)	=> true,
			Self::Off => false,
		}
	}
	
	#[inline]
	pub fn into(self) -> Option<T> {
		match self {
			Self::AlwaysOn(a)	=> Some(a),
			Self::AlwaysOff => None,
			Self::On(a) => Some(a),
			Self::Off => None,
		}
	}
	
	#[inline]
	pub fn into_all(self) -> (Option<T>, ActiveLockState) {
		match self {
			Self::AlwaysOn(a)	=> (Some(a), ActiveLockState::always_on()),
			Self::AlwaysOff => (None, ActiveLockState::always_off()),
			Self::On(a)	=> (Some(a), ActiveLockState::state(true)),
			Self::Off => (None, ActiveLockState::state(false)),
		}
	}
}

impl<T> From<T> for MoveActiveLockState<T> {
	#[inline(always)]
	fn from(a: T) -> Self {
		Self::AlwaysOn(a)
	}
}

impl<T> Into<Option<T>> for MoveActiveLockState<T> {
	#[inline(always)]
	fn into(self) -> Option<T> {
		MoveActiveLockState::into(self)
	}
}

impl<T> Into<(Option<T>, ActiveLockState)> for MoveActiveLockState<T> {
	#[inline(always)]
	fn into(self) -> (Option<T>, ActiveLockState) {
		MoveActiveLockState::into_all(self)
	}
}


impl<T> Display for MoveActiveLockState<T> {
	fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		let str = match self {
			Self::AlwaysOn(_)	=> "MoveActiveLockState::AlwaysOn(<data>)",
			Self::AlwaysOff => "MoveActiveLockState::AlwaysOff",
			Self::On(_) => "MoveActiveLockState::On(<data>)",
			Self::Off => "MoveActiveLockState::Off",
		};
		
		fmt.write_str(str)
	}
}

