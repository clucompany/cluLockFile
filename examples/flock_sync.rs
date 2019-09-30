
extern crate cluSyncFile;

use crate::cluSyncFile::SyncFile;
use crate::cluSyncFile::file_system::flock::FlockSyncFileTo;
use std::io;
use std::path::Path;

pub fn main() -> Result<(), io::Error> {
	{
		let lock = Path::new("/tmp/test.lock").flock_sync_create_file()?;

		println!("#1 {:?}, is_lock: {}", lock, lock.is_sync());
	}

	Ok( () )
}

