

mod easy;
mod flock;

pub use self::easy::*;
pub use self::flock::*;


use std::io;


#[derive(Debug)]
pub enum ErrFileSysLock {
     LockExists,
     RecoveryNotSupported,
     ErrIo(io::Error),
}


