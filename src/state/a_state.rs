
use std::fmt::Display;
use std::fmt;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum ActiveSyncState {
	State(bool),
	
	AlwaysOn,
	AlwaysOff,
}

impl ActiveSyncState {
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
			Self::State(a)								=> *a,
			Self::AlwaysOn								=> true,
			Self::AlwaysOff							=> false,
		}
	}	
}

impl From<bool> for ActiveSyncState {
	#[inline(always)]
	fn from(a: bool) -> Self {
		Self::State(a)
	}
}

impl Display for ActiveSyncState {
	fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), std::fmt::Error> {
		fmt.write_str({
			match self {
				Self::State(true)						=> "ActiveSyncState::State(true)",
				Self::State(false)						=> "ActiveSyncState::State(false)",
				Self::AlwaysOn							=> "ActiveSyncState::AlwaysOn",
				Self::AlwaysOff						=> "ActiveSyncState::AlwaysOff",
			}
		})
	}
}

