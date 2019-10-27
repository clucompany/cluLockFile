
use std::fmt::Display;
use std::fmt;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum ActiveLockState {
	State(bool),
	
	AlwaysOn,
	AlwaysOff,
}

impl ActiveLockState {
	#[inline]
	pub fn state(b: bool) -> Self {
		Self::State(b)
	}
	
	#[inline]
	pub fn always_on() -> Self {
		Self::AlwaysOn
	}
	
	#[inline]
	pub fn always_off() -> Self {
		Self::AlwaysOff
	}
	
	#[inline]
	pub fn is_lock(&self) -> bool {
		match self {
			Self::State(a) => *a,
			Self::AlwaysOn => true,
			Self::AlwaysOff => false,
		}
	}
	
	#[inline]
	pub fn into(self) -> bool {
		match self {
			Self::State(a) => a,
			Self::AlwaysOn => true,
			Self::AlwaysOff => false,
		}
	}
}

impl From<bool> for ActiveLockState {
	#[inline(always)]
	fn from(a: bool) -> Self {
		Self::State(a)
	}
}

impl Display for ActiveLockState {
	fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), std::fmt::Error> {
		fmt.write_str({
			match self {
				Self::State(true)	=> "ActiveLockState::State(true)",
				Self::State(false) => "ActiveLockState::State(false)",
				Self::AlwaysOn => "ActiveLockState::AlwaysOn",
				Self::AlwaysOff => "ActiveLockState::AlwaysOff",
			}
		})
	}
}

