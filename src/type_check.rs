
use std::fmt::Display;
use std::fmt;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum IsLock {
	StateLock(bool),
	Individually(IsLockType),
}

impl From<bool> for IsLock {
	#[inline(always)]
	fn from(a: bool) -> Self {
		IsLock::StateLock(a)
	}
}

impl From<IsLockType> for IsLock {
	#[inline(always)]
	fn from(a: IsLockType) -> Self {
		IsLock::Individually(a)
	}
}

impl Default for IsLock {
	#[inline(always)]
	fn default() -> Self {
		IsLock::Individually(Default::default())
	}
}

impl Display for IsLock {
	fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), std::fmt::Error> {
		fmt.write_str({
			match self {
				IsLock::Individually(IsLockType::UnableToDetermine) => "IsLockType::Individually(IsLockType::UnableToDetermine)",
				IsLock::Individually(IsLockType::AlwaysOn) => "IsLockType::Individually(IsLockType::AlwaysOn)",
				IsLock::StateLock(true) => "IsLock::StateLock(true)",
				IsLock::StateLock(false) => "IsLock::StateLock(false)",
			}
		})
	}
}


#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum IsLockType {
	UnableToDetermine,
	AlwaysOn,
}

impl From<()> for IsLockType {
	#[inline(always)]
	fn from(_a: ()) -> Self {
		Default::default()
	}
}

impl Default for IsLockType {
	#[inline(always)]
	fn default() -> Self {
		IsLockType::UnableToDetermine	
	}	
}

impl Display for IsLockType {
	fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		fmt.write_str({
			match self {
				IsLockType::UnableToDetermine => "IsLockType::UnableToDetermine",
				IsLockType::AlwaysOn => "IsLockType::AlwaysOn",
			}
		})
	}
}

