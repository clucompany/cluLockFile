
use std::fmt::Display;
use std::fmt;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum CheckLock<T> {
	MoveSelf(T),
	State(Option<T>),
}

impl<T> From<T> for CheckLock<T> {
	#[inline(always)]
	fn from(a: T) -> Self {
		CheckLock::MoveSelf(a)
	}
}
impl<T> From<Option<T>> for CheckLock<T> {
	#[inline(always)]
	fn from(a: Option<T>) -> Self {
		CheckLock::State(a)
	}
}

impl<T> Into<Option<T>> for CheckLock<T> {
	fn into(self) -> Option<T> {
		match self {
			CheckLock::MoveSelf(a) => Some(a),
			CheckLock::State(option) => option,
		}	
	}
}


impl<T> Display for CheckLock<T> {
	fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		fmt.write_str({
			match self {
				CheckLock::MoveSelf(_a) => "CheckLock::Unknown(T)",
				CheckLock::State(Some(_a)) => "CheckLock::State(Some(T))",
				CheckLock::State(None) => "CheckLock::State(None)",
			}
		})
	}
}

