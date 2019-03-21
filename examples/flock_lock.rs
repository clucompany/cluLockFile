
extern crate cluLockFile;

use cluLockFile::file_system::FlockLockTo;
use std::io;

pub fn main() -> Result<(), io::Error> {
	{
		let lock = "/tmp/test.lock".flock_recovery_lock()?;

		println!("{}", ::std::mem::size_of_val(&lock));
		
		println!("#1 {:?}, is_lock: {}", lock, lock.is_lock());
	}

	Ok( () )
}
