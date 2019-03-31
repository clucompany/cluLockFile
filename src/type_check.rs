
use std::fmt::Display;
use std::fmt;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum IsLock {
	StateLock(bool),
	Unknown(IsLockUnk),
}

impl From<bool> for IsLock {
	#[inline(always)]
	fn from(a: bool) -> Self {
		IsLock::StateLock(a)
	}
}

impl From<IsLockUnk> for IsLock {
	#[inline(always)]
	fn from(a: IsLockUnk) -> Self {
		IsLock::Unknown(a)
	}
}

impl Default for IsLock {
	#[inline(always)]
	fn default() -> Self {
		IsLock::Unknown(Default::default())
	}
}

impl Display for IsLock {
	fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), std::fmt::Error> {
		fmt.write_str({
			match self {
				IsLock::Unknown(IsLockUnk::Unknown) => "IsLock::Unknown(IsLockUnk::Unknown)",
				IsLock::Unknown(IsLockUnk::AlwaysOn) => "IsLock::StateLock(IsLockUnk::AlwaysOn)",
				IsLock::StateLock(true) => "IsLock::StateLock(true)",
				IsLock::StateLock(false) => "IsLock::StateLock(false)",
			}
		})
	}
}


#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum IsLockUnk {
	Unknown,
	AlwaysOn,
}

impl From<()> for IsLockUnk {
	#[inline(always)]
	fn from(_a: ()) -> Self {
		Default::default()	
	}
}

impl Default for IsLockUnk {
	#[inline(always)]
	fn default() -> Self {
		IsLockUnk::Unknown	
	}	
}

impl Display for IsLockUnk {
	fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		fmt.write_str({
			match self {
				IsLockUnk::Unknown => "IsLock::Unknown",
				IsLockUnk::AlwaysOn => "IsLock::AlwaysOn",
			}
		})
	}
}

