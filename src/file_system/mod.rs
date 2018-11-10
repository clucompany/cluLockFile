

mod easy;
pub use self::easy::*;

use std::io;


#[derive(Debug)]
pub enum ErrFileSysLock {
     LockExists,
     RecoveryNotSupported,
     ErrIo(io::Error),
}


