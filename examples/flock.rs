
extern crate cluLockFile;

use crate::cluLockFile::LockFile;
use crate::cluLockFile::file_system::flock::ToFlLockFile;
use std::io;
use std::path::Path;

pub fn main() -> Result<(), io::Error> {
	let lock = Path::new("/tmp/test.lock").fl_create_file()?;
	println!("#1 {:?}, is_lock: {}", lock, lock.is_active_lock());
	
	Ok( () )
}

