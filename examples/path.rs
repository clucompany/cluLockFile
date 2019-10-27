
extern crate cluLockFile;

use crate::cluLockFile::LockFile;
use std::path::Path;
use std::io;
use cluLockFile::file_system::path::ToPathLock;

pub fn main() -> Result<(), io::Error> {
	let light_lock = Path::new("/tmp/t0.lock").file_path_lock()?;

	let light2_lock = "/tmp/t1.lock".file_path_lock()?;

	let light3_buf_lock = {
		let mut path = Path::new("/tmp").to_path_buf();
		path.push(format!("t{}.lock", 3));
		
		path
	}.file_path_lock().unwrap();

	let light4_lock = "/tmp/t4.lock".file_path_lock()?;

	println!("#1 {:?}, is_lock: {}", light_lock, light_lock.is_active_lock());
	println!("#2 {:?}, is_lock: {}", light2_lock, light2_lock.is_active_lock());
	println!("#3 {:?}, is_lock: {}", light3_buf_lock, light3_buf_lock.is_active_lock());
	println!("#4 {:?}, is_lock: {}", light4_lock, light3_buf_lock.is_active_lock());

	drop(light_lock);
	drop(light2_lock);
	drop(light3_buf_lock);
	drop(light4_lock);

	Ok( () )
}
