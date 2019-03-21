
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum CheckLock<T> {
	Unknown(T),
	State(Option<T>),
}

impl<T> From<T> for CheckLock<T> {
	#[inline(always)]
	fn from(a: T) -> Self {
		CheckLock::Unknown(a)
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
			CheckLock::Unknown(a) => Some(a),
			CheckLock::State(option) => option,
		}	
	}
}


impl<T> Display for CheckLock<T> {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
		write!(f, "{}", {
			match self {
				CheckLock::Unknown(_a) => "CheckLock::Unknown(T)",
				CheckLock::State(Some(_a)) => "CheckLock::State(Some(T))",
				CheckLock::State(None) => "CheckLock::State(None)",
			}
		})
	}
}

