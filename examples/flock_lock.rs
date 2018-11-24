
extern crate cluLockFile;

use cluLockFile::LockFlockConst;
use cluLockFile::Locker;


pub fn main() {
     let lock = "/tmp/test.lock".flock_reclock().unwrap();
     
     println!("#1 {:?}, is_lock: {}", lock, lock.is_lock());

     drop(lock);
}
